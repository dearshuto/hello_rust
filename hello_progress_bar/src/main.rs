use std::{thread::sleep, time::Duration};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

fn main() {
    let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

    let multi_progress = MultiProgress::new();
    let pb = multi_progress.add(ProgressBar::new(100));
    pb.set_style(spinner_style.clone());
    pb.set_prefix(format!("[{}/64]", 1));

    for index in 0..100 {
        sleep(Duration::from_millis(100));
        pb.set_message(format!("{}/100", index));
        pb.inc(1);
    }
    pb.finish_with_message("Hello World");
}
