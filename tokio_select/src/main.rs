use std::future::Future;
use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::task::Poll::{Pending, Ready};
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::select;
use tokio::sync::{mpsc, oneshot};
use tokio_stream::StreamExt;

async fn some_operation() -> String {
    tokio::time::sleep(Duration::from_millis(1000)).await;
    println!("world");
    String::from("hello")
}

async fn tokio_select() {
    let (mut tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        // let _ = tx1.send("one");
        select! {
            val = some_operation() => {
                let _ = tx1.send(val);
            }
            _ = tx1.closed() => {
                println!("closing");
            }
        }
    });

    tokio::spawn(async {
        let _ = tx2.send("two");
    });

    select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        },
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    }
}

struct MySelect {
    rx1: oneshot::Receiver<&'static str>,
    rx2: oneshot::Receiver<&'static str>,
}

impl Future for MySelect {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Ready(val) = Pin::new(&mut self.rx1).poll(cx) {
            println!("rx1 completed first with {:?}", val);
            return Ready(());
        }
        if let Ready(val) = Pin::new(&mut self.rx2).poll(cx) {
            println!("rx2 completed first with {:?}", val);
            return Ready(());
        }
        Pending
    }
}

async fn my_select() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        tx1.send("one");
    });

    tokio::spawn(async {
        tx2.send("two");
    });

    MySelect {
        rx1,
        rx2,
    }.await
}

async fn select_branch() -> io::Result<()> {
    let (tx, rx) = oneshot::channel();
    tokio::spawn(async {
        tx.send(()).unwrap();
    });
    let mut listener = TcpListener::bind("localhost:3465").await?;
    select! {
        res = async {
            loop {
                let (socket, _) = listener.accept().await?;
                tokio::spawn(async move {
                    // process(socket);
                });
            }
            Ok::<_, io::Error>(())
        } => {
            res?;
        },
        _ = rx => {
            println!("terminating accept loop");
        }
    }
    Ok(())
}

async fn select_pattern() {
    let (mut tx1, mut rx1) = mpsc::channel(128);
    let (mut tx2, mut rx2) = mpsc::channel(128);

    tokio::spawn(async move {
        tx1.send("one").await;
    });

    tokio::spawn(async move {
        tx2.send("two").await;
    });

    select! {
        Some(v) = rx1.recv() => {
            println!("Got {:?} from rx1", v);
        },
        Some(v) = rx2.recv() => {
            println!("Got {:?} from rx2", v);
        },
        else => {
            println!("Bot channels closed");
        }
    }
}

async fn race(data: &[u8], addr1: SocketAddr, addr2: SocketAddr) -> io::Result<()> {
    select! {
        Ok(_) = async {
            let mut socket = TcpStream::connect(addr1).await?;
            socket.write_all(data).await?;
            Ok::<_, io::Error>(())
        } => {}
        Ok(_) = async {
            let mut socket = TcpStream::connect(addr2).await?;
            socket.write_all(data).await?;
            Ok::<_, io::Error>(())
        } => {}
        else => {}
    }

    Ok(())
}

async fn select_loop() {
    let (tx1, mut rx1) = mpsc::channel(128);
    let (tx2, mut rx2) = mpsc::channel(128);
    let (tx3, mut rx3) = mpsc::channel(128);

    tokio::spawn(async move {
        tx1.send("one").await;
        tx2.send("two").await;
        tx3.send("three").await;
    });

    loop {
        let msg = select! {
            Some(msg) = rx1.recv() => msg,
            Some(msg) = rx2.recv() => msg,
            Some(msg) = rx3.recv() => msg,
            else => { break },
        };

        println!("Got {}", msg);
    }

    println!("All channels have been closed");
}

async fn action(input: Option<i32>) -> Option<String> {
    let i = match input {
        Some(input) => input,
        None => return None,
    };
    return Some(i.to_string());
}

async fn resume() {
    let (mut tx, mut rx) = mpsc::channel(128);

    let mut done = false;
    let operation = action(None);
    tokio::pin!(operation);

    tokio::spawn(async move {
        let _ = tx.send(1).await;
        let _ = tx.send(3).await;
        let _ = tx.send(2).await;
    });

    loop {
        select! {
            res = &mut operation, if !done => {
                done = true;

                if let Some(v) = res {
                    println!("GOT = {}", v);
                    return;
                }
            },
            Some(v) = rx.recv() => {
                if v % 2 == 0 {
                    operation.set(action(Some(v)));
                    done = false;
                }
            },
        }
    }
}

async fn stream() {
    let mut stream = tokio_stream::iter(&[1, 2, 3]);
    while let Some(v) = stream.next().await {
        println!("GOT = {:?}", v);
    }
}

#[tokio::main]
async fn main() {
    stream().await;
}
