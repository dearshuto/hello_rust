use std::time::Duration;

#[tokio::main]
async fn main() {
    let (sender, reciever) = std::sync::mpsc::channel();

    // 現在の時間を 10 回送信する
    let task = tokio::spawn(async move {
        for _ in 0..10 {
            let now = std::time::SystemTime::now();
            sender.send(now).unwrap();
            tokio::time::sleep(Duration::from_millis(333)).await;
        }
    });

    // 非同期に通知される現在時刻を出力
    // 1 秒待ってもなにも届かなければ通知が終了したとしてループを抜ける
    loop {
        let Ok(value) = reciever.recv_timeout(Duration::from_millis(1000)) else {
            break;
        };

        println!("{:?}", value);
    }

    task.await.unwrap();
}
