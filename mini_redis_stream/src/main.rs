use std::future::Future;
use std::pin::{Pin};
use std::task::{Context, Poll};
use std::thread;
use std::time::{Duration, Instant};
use async_stream::stream;
use mini_redis::client;
use tokio::signal;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tokio_stream::{Stream, StreamExt};

async fn publish() -> mini_redis::Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.publish("numbers", "1".into()).await?;
    client.publish("numbers", "two".into()).await?;
    client.publish("numbers", "3".into()).await?;
    client.publish("numbers", "four".into()).await?;
    client.publish("numbers", "five".into()).await?;
    client.publish("numbers", "6".into()).await?;

    Ok(())
}

async fn subscribe() -> mini_redis::Result<()> {
    let client = client::connect("127.0.0.1:6379").await?;
    let subscriber = client.subscribe(vec!["numbers".to_string()]).await?;
    let messages = subscriber
        .into_stream()
        // .filter(|msg| match msg {
        //     Ok(msg) if msg.content.len() == 1 => true,
        //     _ => false,
        // })
        // .map(|msg| msg.unwrap().content)
        .filter_map(|msg| match msg {
            Ok(msg) if msg.content.len() == 1 => Some(msg.content),
            _ => None,
        })
        .take(3);

    tokio::pin!(messages);

    while let Some(msg) = messages.next().await {
        println!("got = {:?}", msg)
    }

    Ok(())
}

async fn publish_subscribe() -> mini_redis::Result<()> {
    tokio::spawn(async {
        publish().await;
    });

    subscribe().await;

    println!("DONE");

    Ok(())
}

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            return Poll::Ready(());
        }

        thread::sleep(self.when - Instant::now());

        cx.waker().wake_by_ref();

        Poll::Pending
    }
}

struct Interval {
    rem: usize,
    delay: Delay,
}

impl Stream for Interval {
    type Item = usize;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.rem == 0 {
            return Poll::Ready(None);
        }

        match Pin::new(&mut self.delay).poll(cx) {
            Poll::Ready(_) => {
                let when = self.delay.when + Duration::from_millis(1000);
                self.delay = Delay { when };
                self.rem -= 1;
                Poll::Ready(Some(self.rem.clone()))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

async fn interval() {
    let mut stream = Interval {
        rem: 3,
        delay: Delay { when: Instant::now() },
    };

    while let Some(v) = stream.next().await {
        println!("GOT = {:?}", v);
    }
}

async fn interval2() {
    let stream = stream! {
        let mut when = Instant::now();
        for i in 0..3 {
            let delay = Delay { when };
            delay.await;
            yield i;
            when += Duration::from_millis(1000);
        }
    };

    tokio::pin!(stream);

    while let Some(v) = stream.next().await {
        println!("GOT = {:?}", v);
    }
}

async fn shutdown() {
    match signal::ctrl_c().await {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
        }
    }

    println!("shutting down");
}

async fn some_operation(i: u64, _sender: mpsc::Sender<()>) {
    sleep(Duration::from_millis(100 * i)).await;
    println!("Task {} shutting down.", i);
}

async fn wait_finish() {
    let (send, mut recv) = mpsc::channel(1);
    for i in 0..10 {
        tokio::spawn(some_operation(i, send.clone()));
    }
    drop(send);
    recv.recv().await;
}

#[tokio::main]
async fn main() {
    wait_finish().await;
}
