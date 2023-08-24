use actix_web::web;
use anyhow::Ok;
use sqlx::{Pool, Sqlite, SqlitePool};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct TodoSchema {
    pub title: String,
    pub description: String,
}

impl TodoSchema {
    pub fn new(form: web::Form<TodoSchema>) -> Self {
        TodoSchema {
            title: form.title.to_owned(),
            description: form.description.to_owned(),
        }
    }
}

pub async fn connect_to_db() -> anyhow::Result<SqlitePool> {
    let db = SqlitePool::connect("sqlite::memory:").await?;
    create_db(&db).await?;
    Ok(db)
}

async fn create_db(db: &Pool<Sqlite>) -> anyhow::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            title VARCHAR(250) NOT NULL,
            description VARCHAR(250) NOT NULL
        )",
    )
    .execute(db)
    .await?;
    Ok(())
}

pub async fn insert_todo(db: &Pool<Sqlite>, title: &str, description: &str) -> anyhow::Result<()> {
    let insert =
        format!("INSERT INTO todo (title, description) VALUES ('{title}', '{description}')");
    sqlx::query(&insert).execute(db).await?;
    Ok(())
}

pub async fn get_todos(db: &Pool<Sqlite>) -> anyhow::Result<Vec<TodoSchema>> {
    let res = sqlx::query_as::<_, TodoSchema>("SELECT * FROM todo")
        .fetch_all(db)
        .await?;
    Ok(res)
}
