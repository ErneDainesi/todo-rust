use actix_web::{
    get, http::header::ContentType, post, web, App, HttpRequest, HttpResponse, HttpServer,
};
use leptos::*;
use todo_rust::todo::{Todo, TodoItem, TodosForm, TodosList};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct FormData {
    title: String,
    description: String,
}

#[derive(Clone, Copy, Debug)]
struct AppState {
    id_counter: usize,
}

fn get_example_data() -> Vec<TodoItem> {
    vec![
        TodoItem {
            id: 1,
            title: "Prueba".to_string(),
            description: "Prueba desc".to_string(),
        },
        TodoItem {
            id: 2,
            title: "Otra Prueba".to_string(),
            description: "Otra prueba desc".to_string(),
        },
    ]
}

#[get("/")]
async fn index() -> HttpResponse {
    let html = leptos::ssr::render_to_string(|cx| {
        view! {
            cx,
            <head>
                <meta charset="UTF-8"></meta>
                <meta name="viewport" content="width=device-width, initial-scale=1.0"></meta>
                <script src="https://unpkg.com/htmx.org@1.9.4"></script>
                <title>todo_rust</title>
            </head>
            <body>
                <h1>Todo!</h1>
                <TodosForm/>
                <TodosList todos = get_example_data() />
            </body>
        }
    });
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html)
}

#[post("/")]
async fn add_todo(app_data: web::Data<AppState>, form: web::Form<FormData>) -> HttpResponse {
    let id = app_data.id_counter + 1; // this doesn't really work, but ill use sql ids later
    let form_data = TodoItem {
        id,
        title: form.title.to_owned(),
        description: form.description.to_owned(),
    };
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

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(add_todo)
            .app_data(web::Data::new(AppState { id_counter: 0 }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
