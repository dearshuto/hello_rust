pub mod hello_grpc {
    tonic::include_proto!("hello_grpc");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let message = if 1 < args.len() {
        args[0x1].to_string()
    } else {
        "Hello".to_string()
    };

    let mut client =
        hello_grpc::greeter_client::GreeterClient::connect("http://[::1]:50051").await?;
    let simple = hello_grpc::Simple { value: message };

    let response = client.request(simple).await?;
    println!("{}", response.into_inner().message);

    Ok(())
}
