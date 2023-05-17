use axum::Router;
use db::{get_client, PrismaClient};
use std::net::SocketAddr;
use std::sync::Arc;
use miette::IntoDiagnostic;

#[derive(Clone)]
pub struct AppState {
    pub client: Arc<PrismaClient>,
}

#[tokio::main]
async fn run() -> miette::Result<()> {
    let addr: SocketAddr = "[::]:8080".parse().into_diagnostic()?;
    let client = Arc::new(get_client().await.into_diagnostic()?);
    let router = Router::new().with_state(AppState { client });

    axum::Server::bind(&addr).serve(router.into_make_service()).await.into_diagnostic()?;
    
    Ok(())
}


pub fn main() {
    let result = run();

    if let Some(err) = result.err()  {
        eprintln!("Error: {}", err);
    }
} 