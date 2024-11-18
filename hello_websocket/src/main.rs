use futures_util::{SinkExt, StreamExt};
use tokio::{io::AsyncWriteExt, net::TcpListener};
use tokio_tungstenite::tungstenite::Message;

async fn run() {
    // サーバー
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let server_task = tokio::spawn(async move {
        while let Ok((stream, addr)) = listener.accept().await {
            println!("Server: addr {:?}", addr);

            let mut server_stream = tokio_tungstenite::accept_async(stream)
                .await
                .expect("Error during the websocket handshake occurred");
            server_stream.close(None).await.unwrap();
            // for i in 0..10 {
            //     server_stream
            //         .send(Message::text(format!("From Server: {}", i)))
            //         .await
            //         .unwrap();
            //     server_stream.flush().await.unwrap();
            //     tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            // }
        }
    });

    let (client_stream, _response) = tokio_tungstenite::connect_async("ws://localhost:8080")
        .await
        .unwrap();

    let (write, read) = client_stream.split();

    write.reunite(read).unwrap().close(None).await.unwrap();

    // クライアントの読み込みタスク
    let client_read_task = tokio::spawn(async {
        // read.for_each(|message| async {
        //     let Ok(message) = message else {
        //         return;
        //     };
        //     let data = message.into_data();
        //     println!("client: read data: {:?}", data);
        // })
        // .await;
    });

    let (result0, result1) = futures_util::future::join(client_read_task, server_task).await;
    result0.unwrap();
    result1.unwrap();
}

#[tokio::main]
async fn main() {
    run().await;
}
