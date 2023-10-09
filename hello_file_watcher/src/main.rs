use std::path::Path;

use notify::{Config, RecursiveMode, Watcher};

fn main() {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = notify::RecommendedWatcher::new(tx, Config::default()).unwrap();
    watcher
        .watch(
            Path::new("/Users/shuto/develop/temp/"),
            RecursiveMode::Recursive,
        )
        .unwrap();

    let mut count = 0;
    for res in rx {
        match res {
            Ok(event) => println!("changed: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }

        count += 1;
        if 3 < count {
            println!("unwatch");
            watcher
                .unwatch(Path::new("/Users/shuto/develop/temp/"))
                .unwrap();
            break;
        }
    }
}
