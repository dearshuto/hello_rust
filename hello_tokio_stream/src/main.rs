//! tokio-stream を使って複数の異なる型のストリームを購読するサンプル

use tokio_stream::StreamExt;

#[derive(Debug, Default)]
struct Data {
    #[allow(unused)]
    num: u32,
}

// ReceiverStream は同じ型しか受け付けないので enum でくるむ
#[derive(Debug)]
enum Union {
    #[allow(unused)]
    Data(Data),
    #[allow(unused)]
    Number(usize),
}

fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread().build().unwrap();

    let (tx1, rx1) = tokio::sync::mpsc::channel::<Union>(1);
    let (tx2, rx2) = tokio::sync::mpsc::channel::<Union>(1);

    // Stream に変換
    let rx1 = tokio_stream::wrappers::ReceiverStream::new(rx1);
    let rx2 = tokio_stream::wrappers::ReceiverStream::new(rx2);

    // StreamMap でまとめて監視
    let mut map = tokio_stream::StreamMap::new();
    map.insert("one", rx1);
    map.insert("two", rx2);

    // StreamMap を購読するタスクを作成
    // 対になる Sender がすべてドロップするまではループが回り続ける
    let receive_task_handle = runtime.spawn(async move {
        loop {
            let result = map.next().await;
            let (key, val) = match result {
                Some(result) => result,
                None => break,
            };
            println!(
                "recv, key:{:?}, val:{:?} // map.len() = {}",
                key,
                val,
                map.len()
            );
        }
    });

    // 適当な値を send する
    let send_task_handle0 = runtime.spawn(async move {
        tx1.send(Union::Data(Data::default())).await.unwrap();
    });
    let send_task_handle1 = runtime.spawn(async move {
        tx2.send(Union::Number(5)).await.unwrap();
        tx2.send(Union::Number(8)).await.unwrap();
    });

    // タスクの完了待ち
    runtime.block_on(async move {
        tokio::join!();
        let result = tokio::join!(receive_task_handle, send_task_handle0, send_task_handle1);
        result.0.unwrap();
        result.1.unwrap();
        result.2.unwrap();
    });
}
