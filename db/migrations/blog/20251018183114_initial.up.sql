-- Tables
CREATE TABLE post (
	id serial,
	slug text not null,
	title text not null,
	body text not null,
	category text not null default 'general',
	published boolean not null default false,
	queued boolean not null default false,
	upload_date timestamp without time zone not null default NOW(),
	publish_date timestamp without time zone,
	num_reads integer not null default 0,
	PRIMARY KEY (id)
);

CREATE TABLE users (
	id serial,
	username text not null,
	email text not null,
	PRIMARY KEY (id)
);

CREATE TABLE comments (
	id serial,
	user_id integer,
	post_id integer not null,
	created_at timestamp without time zone not null default NOW(),
	updated_at timestamp without time zone,
	body text not null,
	PRIMARY KEY (id),
	FOREIGN KEY (user_id) REFERENCES users(id)
		ON UPDATE CASCADE
		ON DELETE SET NULL,
	FOREIGN KEY (post_id) REFERENCES post(id)
		ON UPDATE CASCADE
		ON DELETE CASCADE
);

CREATE TABLE subscribed_post (
	user_id integer not null,
	post_id integer not null,
	PRIMARY KEY (user_id, post_id),
	FOREIGN KEY (user_id) REFERENCES users(id)
		ON UPDATE CASCADE
		ON DELETE CASCADE,
	FOREIGN KEY (post_id) REFERENCES post(id)
		ON UPDATE CASCADE
		ON DELETE CASCADE
);

CREATE TABLE updated_post (
	post_id integer not null unique,
	last_updated timestamp without time zone not null default NOW(),
	PRIMARY KEY (post_id),
	FOREIGN KEY (post_id) REFERENCES post(id)
);

CREATE TABLE publish_queue (
	id serial,
	next_run_at timestamp not null,
	run_interval interval not null default interval '7 days',
	active boolean not null default true,
	PRIMARY KEY (id)
);

-- Views
CREATE OR REPLACE VIEW unpublished_posts
AS
SELECT *
FROM post
WHERE publish_date IS NOT NULL
AND publish_date < CURRENT_TIMESTAMP
AND published = false;

CREATE OR REPLACE VIEW next_queued_post
AS
SELECT *
FROM post
WHERE published = false
AND queued = true
ORDER BY upload_date
LIMIT 1;

-- Functions
CREATE OR REPLACE FUNCTION get_subscriber_emails()
RETURNS TABLE(email text, title text)
AS $$
BEGIN
	RETURN QUERY
	SELECT u.email, p.title
	FROM users AS u
	JOIN subscribed_post AS s ON u.id = s.user_id
	JOIN post AS p ON s.post_id = p.id
	JOIN updated_post AS up ON p.id = up.post_id
	ORDER BY u.email;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION publish_next_post()
RETURNS integer
AS $$
DECLARE updated_count integer := 0;
BEGIN
	WITH next AS (
		SELECT id
		FROM post
		WHERE published = false
		AND queued = true
		ORDER BY upload_date
		LIMIT 1
		FOR UPDATE SKIP LOCKED
	)
	UPDATE post
	SET published = true, queued = false, publish_date = NOW()
	WHERE post.id IN (SELECT id FROM next);

	GET DIAGNOSTICS updated_count = ROW_COUNT;
	RETURN updated_count;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION tick_publish_queue()
RETURNS integer
AS $$
DECLARE
	v_id int;
	v_interval interval;
	published int := 0;
BEGIN
	WITH due AS (
		SELECT id, run_interval
		FROM publish_queue
		WHERE active = true
		AND next_run_at <= NOW()
		ORDER BY next_run_at
		LIMIT 1
		-- Only run once per date
		FOR UPDATE SKIP LOCKED
	)
	UPDATE publish_queue q
	SET next_run_at = q.next_run_at + d.run_interval
	FROM due d
	WHERE q.id = d.id
	RETURNING q.id, d.run_interval
	INTO v_id, v_interval;

	IF v_id IS NULL THEN
		RETURN 0;
	END IF;

	published := publish_next_post();

	RETURN published;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION get_comments(view_post integer)
RETURNS TABLE(username text, 
	body text, 
	created_at timestamp without time zone,
	updated_at timestamp without time zone
)
AS $$
BEGIN
	RETURN QUERY
	SELECT u.username, c.body, c.created_at, c.updated_at
	FROM comments AS c
	LEFT JOIN users AS u ON c.user_id = u.id
	WHERE c.post_id = view_post
	ORDER BY c.created_at;
END;
$$ LANGUAGE plpgsql;

-- Handler Functions
CREATE OR REPLACE FUNCTION handle_existing_updated_post()
RETURNS TRIGGER
AS $$
BEGIN
	UPDATE updated_post
	SET last_updated = NOW()
	WHERE post_id = NEW.post_id;

	IF FOUND THEN
		RETURN NULL;
	END IF;

	RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Triggers
DROP TRIGGER IF EXISTS update_updated_post_on_conflect ON updated_post;
CREATE TRIGGER update_updated_post_on_conflict
BEFORE INSERT ON updated_post
FOR EACH ROW
	EXECUTE FUNCTION handle_existing_updated_post();
