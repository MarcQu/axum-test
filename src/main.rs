use axum::{routing::get, Router};

fn init_router() -> Router {
    Router::new().route("/", get(hello_world))
}

async fn hello_world() -> &'static str {
    "Hello world!"
}

#[tokio::main]
async fn main() {
    let app = init_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
