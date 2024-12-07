mod grpc_controller;
mod post;
mod sql_operations;

use grpc_controller::PostsServicesStruct;
use log::info;
use posts::posts_service_server::PostsServiceServer;
use tonic::transport::Server;

pub mod posts {
    tonic::include_proto!("studyvault");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let addr = "0.0.0.0:8081".parse()?;
    let file_service = PostsServicesStruct;
    info!("gRPC Server listening on {}", addr);
    Server::builder()
        .add_service(PostsServiceServer::new(file_service))
        .serve(addr)
        .await?;
    Ok(())
}
