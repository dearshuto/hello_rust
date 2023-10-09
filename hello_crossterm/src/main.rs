use std::time::Duration;

use futures_timer::Delay;
use tokio::select;

use futures_util::{FutureExt, StreamExt};

use crossterm::cursor::position;
use crossterm::event::{Event, KeyCode};

async fn run() {
    let mut event_stream = crossterm::event::EventStream::new();
    loop {
        let delay = Delay::new(Duration::from_millis(1_000)).fuse();
        let event = event_stream.next().fuse();

        select! {
            _ = delay => { println!(".\r"); },
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        println!("Event::{:?}\r", event);

                        if event == Event::Key(KeyCode::Char('c').into()) {
                            println!("Cursor position: {:?}\r", position());
                        }

                        if event == Event::Key(KeyCode::Esc.into()) {
                            break;
                        }
                    }
                    Some(Err(e)) => println!("Error: {:?}\r", e),
                    None => break,
                }
            }
        };

        // match event {
        //     crossterm::event::Event::Resize(width, height) => {
        //         println!("{}, {}", width, height);
        //     }
        //     _ => {}
        // }
    }
}

#[tokio::main]
async fn main() {
    run().await;
}
