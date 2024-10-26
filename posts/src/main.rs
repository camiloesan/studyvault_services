mod grpc_controller;
mod post;
mod sql_operations;

use grpc_controller::PostsServicesStruct;
use posts::posts_service_server::PostsServiceServer;
use tonic::transport::Server;

pub mod posts {
    tonic::include_proto!("studyvault");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:8081".parse()?;
    let file_service = PostsServicesStruct::default();
    println!("gRPC Server listening on {}", addr);
    Server::builder()
        .add_service(PostsServiceServer::new(file_service))
        .serve(addr)
        .await?;
    Ok(())
}
