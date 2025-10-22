use crate::auth;
use crate::db::BlogDB;
use ammonia::Builder as HtmlSanitizer;
use comrak::{
    markdown_do_html,
    ComrakExtensionOptions,
    ComrakOptions,
    ComrakParseOptions,
    ComrakRenderOptions,
};
use regex::Regex;
use rocket::{
    form::Form,
    fs::TempFile,
    serde::{ Deserialize, json::Json },
    rocket::{ post, routes, Build, Rocket },
};
use rocket_db_pools::{Connection, sqlx::Row};

#[derive(FromForm)]
struct PostUpload<'r> {
    file: TempFile<'r>,
    publish: bool,
    queued: bool,
}

#[derive(Debug, Deserialize)]
struct FrontMatter {
    title: Option<String>,
    category: Option<String>,
    publish_date: Option<DateTime<Utc>>,
}

#[post("/blog/upload", data = "<form>")]
pub async fn upload(
    user: auth::AuthUser,
    mut db: Connection<BlogDB>,
    mut form: Form<PostUpload<'_>>) -> Json<serde_json::Value> {
    // Read uploaded file to memory
    let mut md = String::new();
    if let Err(e) = form.file.open(1 << 22)
        .await
        .and_then(|mut f| Box::pin(async move {
            use tokio::io::AsyncReadExt;
            f.read_to_string(&mut md).await
        })) {
            // An error occured
            return Json(json!({ "ok": false, "error": format!("read failure: {e}") }));
    }

    // Split front matter
    let (fm, body) = match split_front_matter(&md) {
        Ok((fm, body)) => (fm, body),
        Err(e) => return Json(json!({ "ok": false, "error": e })),
    };
    // Parse front matter
    let meta: FrontMatter = match fm
        .map(|y| serde_yaml::from_str::<FrontMatter>(y))
        .transpose()
    {
        Ok(m) => m.unwrap_or(FrontMatter { 
            title: None, 
            category: None, 
            publish_date: None,
        }),
        Err(e) => return Json(json!({ "ok": false, "error": format!("bad front matter: {e}")})),
    };

    // Resolve Obsidian features
    let body = rewrite_wikilinks(&body);
    
    // Make sure we have a title
    if meta.title == None {
        let filename = form.file.name().unwrap_or("untitled.md");
        if let Some(title) = infer_title(&body, &filename) {
            meta.title = title;
        } else {
            return Json(json!({ "ok": false, "error": "could not infer title".into() }));
        }
    }

    // Markdown -> HTML
    let body = md_do_html(&body);

    // Sanitize HTML
    let body = sanitize(&body);

    // Generate the slug
    let slug = slug::slugify(&meta.title);

    // Write to db
    // Build the query
    let mut qb: QueryBuilder<Postgres> = QueryBuilder::new("INSERT INTO post (");

    // Fields
    let mut sep = qb.separated(", ");
    sep.push("slug");
    sep.push("title");
    sep.push("body");
    sep.push("published");
    sep.push("queued");
    if meta.category.is_some() { sep.push("category"); }
    if meta.publish_date.is_some() { sep.push("publish_date"); }
    
    // Values
    qb.push(") VALUES (");
    let mut v = qb.separated(", ");
    v.push_bind(slug.clone());
    v.push_bind(meta.title.clone());
    v.push_bind(body);
    if form.publish {
        v.push_bind(true);
        v.push_bind(false);
    } else if form.queue {
        v.push_bind(false);
        v.push_bind(true);
    } else {
        v.push_bind(false);
        v.push_bind(false);
    }
    if let Some(c) = meta.category.clone() { v.push_bind(c); }
    if let Some(pd) = meta.category.clone() { v.push_bind(pd); }

    // Finish building query and run
    qb.push(") RETURNING id");
    let row = match qb.build().fetch_one(&mut **db).await {
        Ok(r) => r,
        Err(e) => return Json(json!({ "ok": false, "error": format!("database error: {e}") })),
    }
    let id: i64 = match row.try_get("id") {
        Ok(i) => i,
        Err(e) => return Json(json!({ "ok": false, "error": format!("database error: {e}") })),
    };

    Json(json!({ "ok": true, "post_id": id, "slug": slug }))
}

fn split_front_matter(md: &str) -> Result<(Option<&str>, &str), String> {
    let trimmed = md.trim_start();
    if !trimmed.starts_with("---\n") && !trimmed.starts_with("---\r\n") {
        // No front matter, just return the markdown
        return Ok((None, md));
    }
    
    // Find closing tag
    let rest = &trimmed[4..];
    if let Some(end) = rest.find("\n---") {
        let fm = &rest[..end]; // YAML without metadata tags
        let after = &rest[end + 4..];
        Ok((Some(fm.trim_matches('\r')), after))
    } else {
        Err("Unclosed front matter '---'".into())
    }
}

fn rewrite_wikilinks(s: &str) -> String {
    // TODO: Handle external links
    let re = Regex::new(r"\[\[([^\]\|]+)(?:\|([^\]]+))?\]\]").unwrap();
    re.replace_all(s, |caps: &regex::Captures| {
        let target = caps.get(1).unwrap().as_str().trim();
        let text = caps.get(2).map(|m| m.as_str()).unwrap_or(target);
        let slug = slug::slugify(target);
        format!("[{text}](/blog/{slug})")
    }).into_owned()
}

fn md_to_html(s: &str) -> String {
    let options = ComrakOptions {
        extension: ComrakExtensionOptions {
            strikethrough: true,
            table: true,
            autolink: true,
            tasklist: true,
            footnotes: true,
            ..Default::default()
        },
        parse: ComrakParseOptions { smart: true, ..Default::default() },
        render: ComrakRenderOptions { hardbreaks: false, ..Default::default() },
    };
    markdown_to_html(r, &options)
}

fn sanitize(html: &str) -> String {
    HtmlSanitizer::default()
        .link_rel(None)
        clean(html)
        .to_string()
}

fn infer_title(md: &str, filename: &str) -> Option<String> {
    // First ATX header or filename
    for line in md.lines() {
        let line = line.trim();
        if line.starts_with("# ") {
            return Some(line.trim_start_matches("# ").trim().to_string());
        }
        if !line.is_empty() { 
            // Trim extension from filename
            let filename = Path::new(filename)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or(filename);
            return Some(filename.into()); 
        }
    }
    None
}
