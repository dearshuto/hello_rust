use std::path::Path;

use notify::{Config, RecursiveMode, Watcher};

fn main() {
    let path = Path::new(".");
    println!("watch: {}", path.display());

    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = notify::RecommendedWatcher::new(tx, Config::default()).unwrap();
    watcher.watch(path, RecursiveMode::Recursive).unwrap();

    let mut count = 0;
    for res in rx {
        match res {
            Ok(event) => println!("changed: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }

        count += 1;
        if 3 < count {
            println!("unwatch");
            watcher.unwatch(Path::new(path)).unwrap();
            break;
        }
    }
}
