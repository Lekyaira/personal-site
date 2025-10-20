use rocket_db_pools::Database;

#[derive(Database)]
#[database("blog")]
pub struct BlogDB(rocket_db_pools::sqlx::PgPool);
