use crate::api::DiscordeState;
use crate::chat::ChatSvc;
use crate::db::Database;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod api;
mod chat;
mod db;
mod models;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(if std::env::var("RUST_LOG").is_ok() {
            tracing_subscriber::EnvFilter::from_default_env()
        } else {
            tracing_subscriber::EnvFilter::new("info")
        })
        .init();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let db = Arc::new(Database::new(PathBuf::from("database")).await);
    axum::serve(
        listener,
        api::routes(DiscordeState {
            chat: ChatSvc::new(db.clone()),
            db,
        })
        .into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
