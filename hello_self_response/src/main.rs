use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;
use std::task::Context;
use std::task::Poll;

use tonic::{transport::Server, Response, Status};

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

pub mod hello_grpc {
    tonic::include_proto!("hello_grpc");
}

struct MySignal {
    is_running: Arc<Mutex<bool>>,
    count: u8,
}

impl MySignal {
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(Mutex::new(true)),
            count: 0,
        }
    }

    pub fn clone_shared_state(&self) -> Arc<Mutex<bool>> {
        self.is_running.clone()
    }
}

impl Future for MySignal {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let data = self.get_mut();
        data.count += 1;

        let mut is_running = data.is_running.lock().unwrap();
        *is_running = data.count < 10;
        if *is_running {
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();
    let signal = MySignal::new();
    let shared_state = signal.clone_shared_state();

    // サーバーを別スレッドで起動
    let server_task = tokio::spawn(async move {
        Server::builder()
            .add_service(GreeterServer::new(greeter))
            .serve_with_shutdown(addr, signal)
            .await
            .unwrap();
    });
    println!("Server running");

    // クライアントからサーバーにメッセージを送信
    loop {
        let is_running = *shared_state.lock().unwrap();
        if !is_running {
            break;
        }

        let message = "AAAA".to_string();
        let mut client =
            hello_grpc::greeter_client::GreeterClient::connect("http://[::1]:50051").await?;
        let simple = hello_grpc::Simple { value: message };

        let response = client.request(simple).await?;
        println!("{}", response.into_inner().message);

        let duration = std::time::Duration::from_millis(500);
        std::thread::sleep(duration);
    }

    server_task.await?;

    Ok(())
}
