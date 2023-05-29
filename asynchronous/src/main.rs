use std::future::Future;
use std::io::Error;
use std::pin::Pin;
use std::rc::Rc;
use futures::channel::mpsc::channel;
use futures::executor::block_on;
use futures::{future, FutureExt, pin_mut, select, SinkExt, Stream, StreamExt, try_join, TryFutureExt, TryStreamExt};
use futures::future::{BoxFuture, Fuse};
use futures::stream::{FusedStream, FuturesUnordered};
use futures::future::FusedFuture;
use async_trait::async_trait;

async fn hello_world() {
    hello_cat().await;
    println!("hello, world!");
}

async fn hello_cat() {
    println!("hello, kitty!");
}

struct Song {
    author: String,
    name: String,
}

async fn learn_song() -> Song {
    Song {
        author: "曲婉婷".to_string(),
        name: String::from("我的歌声里"),
    }
}

async fn sing_song(song: Song) {
    println!(
        "给大家献上一首{}的{} ~ {}",
        song.author, song.name, "你存在我深深的脑海里~ ~"
    )
}

async fn dance() {
    println!("唱到情深处，身体不由自主的动了起来~ ~");
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    futures::join!(f1, f2);
}

trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}

struct Socket {}

impl Socket {
    fn has_data_to_read(&self) -> bool {
        true
    }

    fn read_buf(&self) -> Vec<u8> {
        vec![1, 2, 3]
    }

    fn set_readable_callback(&self, _wake: fn()) {
        // ...
    }
}

pub struct SocketRead<'a> {
    socket: &'a Socket,
}

impl SimpleFuture for SocketRead<'_> {
    type Output = Vec<u8>;

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if self.socket.has_data_to_read() {
            Poll::Ready(self.socket.read_buf())
        } else {
            self.socket.set_readable_callback(wake);
            Poll::Pending
        }
    }
}

pub struct Join<FutureA, FutureB> {
    a: Option<FutureA>,
    b: Option<FutureB>,
}

impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
    where
        FutureA: SimpleFuture<Output=()>,
        FutureB: SimpleFuture<Output=()>,
{
    type Output = ();

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(a) = &mut self.a {
            if let Poll::Ready(()) = a.poll(wake) {
                self.a.take();
            }
        }

        if let Some(b) = &mut self.b {
            if let Poll::Ready(()) = b.poll(wake) {
                self.b.take();
            }
        }

        if self.a.is_none() && self.b.is_none() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

pub struct AndThenFut<FutureA, FutureB> {
    first: Option<FutureA>,
    second: FutureB,
}

impl<FutureA, FutureB> SimpleFuture for AndThenFut<FutureA, FutureB> where
    FutureA: SimpleFuture<Output=()>,
    FutureB: SimpleFuture<Output=()>,
{
    type Output = ();

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(first) = &mut self.first {
            match first.poll(wake) {
                Poll::Ready(()) => self.first.take(),
                Poll::Pending => return Poll::Pending,
            };
        }

        self.second.poll(wake)
    }
}

async fn foo() -> u8 {
    5
}

fn bar() -> impl Future<Output=u8> {
    async {
        let x: u8 = foo().await;
        x + 5
    }
}

async fn borrow_x(x: &u8) -> u8 {
    *x
}

fn good() -> impl Future<Output=u8> {
    async {
        let x = 5;
        borrow_x(&x).await
    }
}

async fn send_recv() {
    const BUFFER_SIZE: usize = 10;
    let (mut tx, mut rx) = channel::<i32>(BUFFER_SIZE);
    tx.send(1).await.unwrap();
    tx.send(2).await.unwrap();
    drop(tx);

    assert_eq!(Some(1), rx.next().await);
    assert_eq!(Some(2), rx.next().await);
    assert_eq!(None, rx.next().await);
}

async fn sum_with_next(mut stream: Pin<&mut dyn Stream<Item=i32>>) -> i32 {
    let mut sum = 0;
    while let Some(item) = stream.next().await {
        sum += item;
    }
    sum
}

async fn sum_with_try_next(mut stream: Pin<&mut dyn Stream<Item=Result<i32, Error>>>) -> Result<i32, Error> {
    let mut sum = 0;
    while let Some(item) = stream.try_next().await? {
        sum += item;
    }
    Ok(sum)
}

async fn jump_around(mut stream: Pin<&mut dyn Stream<Item=Result<u8, Error>>>) -> Result<(), Error> {
    const MAX_CONCURRENT_JUMPERS: usize = 100;
    stream.try_for_each_concurrent(MAX_CONCURRENT_JUMPERS, |num| async move {
        Ok(())
    }).await?;
    Ok(())
}

struct Book;

struct Music;

async fn get_book() -> Result<Book, ()> {
    Ok(Book)
}

async fn get_music() -> Result<Music, String> {
    Ok(Music)
}

async fn get_book_and_music() -> Result<(Book, Music), String> {
    let book_fut = get_book().map_err(|()| "Unable to get book".to_string());
    let music_fut = get_music();
    try_join!(book_fut, music_fut)
}

async fn task_one() {}

async fn task_two() {}

async fn race_tasks() {
    let t1 = task_one().fuse();
    let t2 = task_two().fuse();

    pin_mut!(t1, t2);

    select! {
        () = t1 => println!("Task one finished first"),
        () = t2 => println!("Task two finished first"),
    }
}

async fn select() {
    let mut a_fut = future::ready(4);
    let mut b_fut = future::ready(6);
    let mut total = 0;

    loop {
        select! {
            a = a_fut => total += a,
            b = b_fut => total += b,
            complete => break,
            default => panic!(),
        }
    }

    assert_eq!(total, 10);
}

async fn add_two_streams(
    mut s1: impl Stream<Item=u8> + FusedStream + Unpin,
    mut s2: impl Stream<Item=u8> + FusedStream + Unpin,
) -> u8 {
    let mut total = 0;

    loop {
        let item = select! {
            x = s1.next() => x,
            x = s2.next() => x,
            complete => break,
        };
        if let Some(next_num) = item {
            total += next_num;
        }
    }

    total
}

async fn get_new_num() -> u8 { 5 }

async fn run_on_new_num(_: u8) {}

async fn run_on_new_num2(_: u8) -> u8 { 5 }

async fn run_loop(
    mut interval_timer: impl Stream<Item=()> + FusedStream + Unpin,
    starting_num: u8,
) {
    let run_on_new_num_fut = run_on_new_num(starting_num).fuse();
    let get_new_num_fut = Fuse::terminated();
    pin_mut!(run_on_new_num_fut, get_new_num_fut);

    loop {
        select! {
            () = interval_timer.select_next_some() => {
                if get_new_num_fut.is_terminated() {
                    get_new_num_fut.set(get_new_num().fuse());
                }
            },
            new_num = get_new_num_fut => {
                run_on_new_num_fut.set(run_on_new_num(new_num).fuse());
            },
            () = run_on_new_num_fut => {},
            complete => panic!("`interval_timer` completed unexpectedly"),
        }
    }
}

async fn run_loop2(
    mut interval_timer: impl Stream<Item=()> + FusedStream + Unpin,
    starting_num: u8,
) {
    let mut run_on_new_num_futs = FuturesUnordered::new();
    run_on_new_num_futs.push(run_on_new_num2(starting_num));
    let get_new_num_fut = Fuse::terminated();
    pin_mut!(get_new_num_fut);

    loop {
        select! {
            () = interval_timer.select_next_some() => {
                if get_new_num_fut.is_terminated() {
                    get_new_num_fut.set(get_new_num().fuse());
                }
            },
            new_num = get_new_num_fut => {
                run_on_new_num_futs.push(run_on_new_num2(new_num));
            },
            res = run_on_new_num_futs.select_next_some() => {
                println!("run_on_new_num_fut returned {:?}", res);
            },
            complete => panic!("`interval_timer` completed unexpectedly"),
        }
    }
}

async fn foo2() -> Result<(), String> {
    Ok(())
}

async fn bar2() -> Result<(), String> {
    Ok(())
}

fn foo_bar() {
    let fut = async {
        foo2().await?;
        bar2().await?;
        Ok::<(), String>(())
    };
}

#[derive(Default)]
struct NotSend(Rc<()>);

async fn bar3() {}

async fn foo3() {
    { let x = NotSend::default(); }
    bar3().await;
}

fn require_send(_: impl Send) {}

fn run() {
    require_send(foo3());
}

fn recursive() -> BoxFuture<'static, ()> {
    async {
        recursive().await;
        recursive().await;
    }.boxed()
}

#[async_trait]
trait Advertisement {
    async fn run(&self);
}

struct Modal;

#[async_trait]
impl Advertisement for Modal {
    async fn run(&self) {}
}

fn main() {
    let future = select();
    block_on(future);
}
