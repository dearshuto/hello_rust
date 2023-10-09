use tonic::{transport::Server, Response, Status};

pub mod hello_grpc {
    tonic::include_proto!("hello_grpc");
}
use hello_grpc::greeter_server::{Greeter, GreeterServer};
use hello_grpc::{Reply, Simple};

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn request(&self, request: tonic::Request<Simple>) -> Result<Response<Reply>, Status> {
        println!("{}", request.into_inner().value);

        let reply = hello_grpc::Reply {
            message: "Server Response".to_string(),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;
    Ok(())
}
