use actix_web::web;
use anyhow::Ok;
use sqlx::{Pool, Sqlite, SqlitePool};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct TodoSchema {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub status: bool,
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
            status: false,
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
            description VARCHAR(250) NOT NULL,
            status TINYINT NOT NULL DEFAULT 0
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
        status: false,
    })
}

pub async fn remove_todo(db: &Pool<Sqlite>, id: i64) -> anyhow::Result<()> {
    let delete = format!("DELETE FROM todo WHERE id = {id}");
    sqlx::query(&delete).execute(db).await?;
    Ok(())
}

pub async fn get_todo_by_id(db: &Pool<Sqlite>, id: i64) -> anyhow::Result<TodoSchema> {
    let select = format!("SELECT * FROM todo WHERE id = {id}");
    let res = sqlx::query_as::<_, TodoSchema>(&select).fetch_one(db).await?;
    Ok(res)
}

pub async fn set_status(db: &Pool<Sqlite>, id: i64, status: bool) -> anyhow::Result<bool> {
    let status = !status;
    let update = format!("UPDATE todo SET status = {status} WHERE id = {id}");
    sqlx::query(&update).execute(db).await?;
    Ok(status)
}

pub async fn get_todos(db: &Pool<Sqlite>) -> anyhow::Result<Vec<TodoSchema>> {
    let res = sqlx::query_as::<_, TodoSchema>("SELECT * FROM todo")
        .fetch_all(db)
        .await?;
    Ok(res)
}
