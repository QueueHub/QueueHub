mod handler;
mod model;
mod response;
mod route;

mod queueHub;

use axum::{http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
}, Router};
use route::create_router;
use tower_http::cors::CorsLayer;
use queueHub::queue::handler::{
    queue_router
}

#[tokio::main]
async fn main() {

    queueHub::queue::handler::queue_handler();


    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app1 = create_router().layer(cors);
    let app = Router::new().nest("/queue", queue_router());

    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();






}
