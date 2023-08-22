use actix_web::{get, http::header::ContentType, App, HttpResponse, HttpServer};
use leptos::*;
use todo_rust::todo::Todo;

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
                <section>
                    <Todo/>
                </section>
            </body>
        }
    });
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
