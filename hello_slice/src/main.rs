// スライスの記法と挙動を試すプログラム

fn main() {
    let mut data = vec![1, 4, 0, 2, 6, 8, 5, 9];
    println!("original: {:?}", data);

    // 特定の範囲だけソート
    data[4..].sort();
    println!("after   : {:?}", data);
}
