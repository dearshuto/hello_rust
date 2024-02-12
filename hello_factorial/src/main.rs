fn factional(value: u64) -> u64 {
    if value == 0 {
        1
    } else {
        value * factional(value - 1)
    }
}

// 0! ~ 20! までを出力
fn main() {
    for index in 0..=20 {
        println!("{:2}! = {:19}", index, factional(index));
    }
}
