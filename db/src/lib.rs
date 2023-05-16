#[allow(warnings, unused)]
mod prisma;

pub use prisma::PrismaClient;
use prisma_client_rust::NewClientError;

pub async fn get_client() -> Result<PrismaClient, NewClientError> {
    PrismaClient::_builder().build().await
}
