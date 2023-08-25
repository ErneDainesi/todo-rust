use actix_web::web;
use anyhow::Ok;
use sqlx::{Pool, Sqlite, SqlitePool};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct TodoSchema {
    pub id: i64,
    pub title: String,
    pub description: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct TodoFormData {
    pub title: String,
    pub description: String,
}

impl TodoSchema {
    pub fn new(form: web::Form<TodoFormData>, id: i64) -> Self {
        TodoSchema {
            id,
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

pub async fn insert_todo(
    db: &Pool<Sqlite>,
    title: &str,
    description: &str,
) -> anyhow::Result<TodoSchema> {
    let insert =
        format!("INSERT INTO todo (title, description) VALUES ('{title}', '{description}')");
    let res = sqlx::query(&insert).execute(db).await?;
    Ok(TodoSchema {
        id: res.last_insert_rowid(),
        title: title.to_string(),
        description: description.to_string(),
    })
}

pub async fn remove_todo(db: &Pool<Sqlite>, id: i64) -> anyhow::Result<()> {
    let delete = format!("DELETE FROM todo WHERE id = {id}");
    sqlx::query(&delete).execute(db).await?;
    Ok(())
}

pub async fn get_todos(db: &Pool<Sqlite>) -> anyhow::Result<Vec<TodoSchema>> {
    let res = sqlx::query_as::<_, TodoSchema>("SELECT * FROM todo")
        .fetch_all(db)
        .await?;
    Ok(res)
}
