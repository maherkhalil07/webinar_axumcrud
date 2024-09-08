mod db;
mod rest;
mod view;
use crate::db::{init_db, create_db_if_not_exists};
use anyhow::Result;
use axum::{Extension, Router};
use sqlx::postgres::PgPool;
use std::env;
use std::net::SocketAddr;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt}; // Arc is needed to share Mutex across async tasks

/// Build the overall web service router.
/// Constructing the router in a function makes it easy to re-use in unit tests.
fn router(connection_pool: PgPool) -> Router {
    Router::new()
        // Nest service allows you to attach another router to a URL base.
        // "/" inside the service will be "/books" to the outside world.
        .nest_service("/books", rest::books_service())
        // Add the web view
        .nest_service("/", view::view_service())
        // Add the connection pool as a "layer", available for dependency injection.
        .layer(Extension(connection_pool))
}

//todo: seperate the function of creating the db


#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env if available
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

     // Ensure the database exists
     create_db_if_not_exists().await?;

    // Initialize the database and obtain a connection pool
    let connection_pool = init_db().await?;

    // Initialize the Axum routing service
    let app = router(connection_pool);
    
    
    let ip = env::var("ip").expect("IP address not set in .env");
    let port = env::var("port").expect("Port not set in .env");
    
    // Define the address to listen on (everything)
    let addr: SocketAddr = format!("{}:{}", ip, port)
    .parse()
    .expect("Unable to parse socket address");

    info!("Server running on http://{}", addr);

    let listner = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listner, app).await.unwrap_or_else(|e| {
        error!("Server error: {}", e);
        std::process::exit(1);
    });

    Ok(())
    
}
