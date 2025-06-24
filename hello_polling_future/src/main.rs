/// Future でポーリングを実装するサンプル
use std::time::Duration;

use pin_project::pin_project;
use tokio::time::Instant;

trait Task {
    type Context: Unpin;

    fn execute(context: Self::Context) -> Option<Self::Context>;
}

struct MyTask;
impl Task for MyTask {
    type Context = u32;

    fn execute(mut context: Self::Context) -> Option<Self::Context> {
        if 10 <= context {
            return None;
        }
        println!("Poll {}", context);
        context += 1;
        return Some(context);
    }
}

#[pin_project]
struct PollingSystem<T>
where
    T: Task,
{
    #[pin]
    timer: tokio::time::Sleep,

    context: Option<T::Context>,
}

impl<T> PollingSystem<T>
where
    T: Task,
{
    pub fn new(context: T::Context) -> Self {
        Self {
            timer: tokio::time::sleep(Duration::from_millis(1000)),
            context: Some(context),
        }
    }
}

impl<T> Future for PollingSystem<T>
where
    T: Task,
{
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut me = self.project();

        match me.timer.as_mut().poll(cx) {
            std::task::Poll::Ready(_) => {
                let Some(context) = me.context.take() else {
                    return std::task::Poll::Ready(());
                };

                *me.context = T::execute(context);
                me.timer
                    .as_mut()
                    .reset(Instant::now() + Duration::from_millis(1000));
                return std::task::Poll::Pending;
            }
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Start Polling");
    PollingSystem::<MyTask>::new(0).await;
    println!("End Polling");
}
