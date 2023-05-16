use axum::Router;
use db::{get_client, PrismaClient};
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub client: Arc<PrismaClient>,
}

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let client = Arc::new(get_client().await.unwrap());
    let router = Router::new().with_state(AppState { client });

    axum::Server::bind(&addr).serve(router.into_make_service()).await.unwrap();
    
}
