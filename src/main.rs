use actix_web::{
    delete, get,
    http::{header::ContentType, StatusCode},
    post, web, App, HttpResponse, HttpServer,
};
use anyhow::Ok;
use leptos::*;
use sqlx::sqlite::SqlitePool;
use std::sync::{Arc, Mutex};
use todo_rust::{
    db::{connect_to_db, get_todos, insert_todo, remove_todo, TodoFormData, TodoSchema},
    todo::{Todo, TodosForm, TodosList},
};

#[derive(Clone, Debug)]
struct AppState {
    db: Arc<Mutex<SqlitePool>>,
}

#[get("/")]
async fn index(app_data: web::Data<AppState>) -> HttpResponse {
    let db = app_data.db.lock().unwrap();
    let todos = get_todos(&db).await.unwrap();
    let html = leptos::ssr::render_to_string(|cx| {
        view! {
            cx,
            <head>
                <meta charset="UTF-8"></meta>
                <meta name="viewport" content="width=device-width, initial-scale=1.0"></meta>
                <link rel="stylesheet" href="../static/swagg.css"></link>
                <title>todo_rust</title>
            </head>
            <body>
                <h1 class="page-head">Todo!</h1>
                <TodosForm/>
                <TodosList todos />
                <script src="https://unpkg.com/htmx.org@1.9.4"></script>
            </body>
        }
    });
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html)
}

#[post("/")]
async fn add_todo(app_data: web::Data<AppState>, form: web::Form<TodoFormData>) -> HttpResponse {
    let db = app_data.db.lock().unwrap();
    let form_data = insert_todo(&db, &form.title, &form.description)
        .await
        .unwrap();
    let html = leptos::ssr::render_to_string(|cx| {
        view! {
            cx,
            <Todo data = form_data />
        }
    });
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html)
}

#[delete("/{id}")]
async fn delete_todo(app_data: web::Data<AppState>, id: web::Path<i64>) -> HttpResponse {
    let db = app_data.db.lock().unwrap();
    remove_todo(&db, *id).await.unwrap();
    HttpResponse::Ok()
        .status(StatusCode::OK)
        .finish()
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let db = Arc::new(Mutex::new(connect_to_db().await?));
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(add_todo)
            .service(delete_todo)
            .service(
                actix_files::Files::new("/static", "static/")
                    .use_last_modified(true)
                    .index_file("swagg.css"),
            )
            .app_data(web::Data::new(AppState { db: db.clone() }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .unwrap();
    Ok(())
}
