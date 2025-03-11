/// y = x^2 を [0, 2] で積分した値をモンテカルロ積分で算出するサンプル

fn function(x: f32) -> f32 {
    x * x
}

// 一様分布で計算
fn normal<const N: u32>() -> f32 {
    let sum: f32 = (0..N)
        .map(|_| {
            let rand = rand::random_range(0.0f32..2.0);
            function(rand)
        })
        .sum();
    sum / (0.5 * N as f32)
}

// 確率密度関数を y = x/2 として重みづけ
fn weight<const N: u32>() -> f32 {
    let sum: f32 = (0..N)
        .map(|_| {
            let x = rand::random_range(0.0f32..1.0);
            let weighted_x = (4.0 * x).sqrt();
            let p = 0.5 * weighted_x;
            function(weighted_x) / p
        })
        .sum();
    sum / (N as f32)
}

fn print<const N: u32>() {
    println!(
        "N=>{:>6}      = {:.4} / {:.4}",
        N,
        normal::<N>(),
        weight::<N>()
    );
}

fn main() {
    // 期待値は手計算で求めた値
    println!("expected: {:.4}", 8.0 / 3.0);

    // サンプリング回数を指定して計算
    println!("                normal / weight");
    print::<1>();
    print::<5>();
    print::<10>();
    print::<50>();
    print::<100>();
    print::<150>();
    print::<500>();
    print::<1000>();
    print::<10000>();
    print::<100000>();
}
