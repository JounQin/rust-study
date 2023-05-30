use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::{Duration, Instant};
use crossbeam::channel;
use futures::task;
use futures::task::ArcWake;
use tokio::sync::Notify;

struct Delay {
    when: Instant,
    waker: Option<Arc<Mutex<Waker>>>,
}

impl Future for Delay {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(waker) = &self.waker {
            let mut waker = waker.lock().unwrap();
            if !waker.will_wake(cx.waker()) {
                *waker = cx.waker().clone();
            }
        } else {
            let when = self.when;
            let waker = Arc::new(Mutex::new(cx.waker().clone()));
            self.waker = Some(waker.clone());
            thread::spawn(move || {
                let now = Instant::now();
                if now < when {
                    thread::sleep(when - now);
                }
                let waker = waker.lock().unwrap();
                waker.wake_by_ref();
            });
        }

        if Instant::now() >= self.when {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

fn main() {
    let mini_tokio = MiniTokio::new();

    mini_tokio.spawn(async {
        let when = Instant::now() + Duration::from_millis(1000 * 2);
        let future = Delay { when, waker: None };
        future.await;

        delay(Duration::from_millis(3000)).await;
        println!("delay fn");
    });

    mini_tokio.run();
}

struct MiniTokio {
    scheduled: channel::Receiver<Arc<Task>>,
    sender: channel::Sender<Arc<Task>>,
}

struct Task {
    future: Mutex<Pin<Box<dyn Future<Output=()> + Send>>>,
    executor: channel::Sender<Arc<Task>>,
}

impl Task {
    fn schedule(self: &Arc<Self>) {
        self.executor.send(self.clone()).unwrap();
    }

    fn poll(self: Arc<Self>) -> Poll<()> {
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);
        let mut future = self.future.try_lock().unwrap();
        future.as_mut().poll(&mut cx)
    }

    fn spawn<F>(future: F, sender: &channel::Sender<Arc<Task>>)
        where F: Future<Output=()> + Send + 'static {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            executor: sender.clone(),
        });
        let _ = sender.send(task);
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule()
    }
}

impl MiniTokio {
    fn new() -> MiniTokio {
        let (sender, scheduled) = channel::unbounded();
        MiniTokio { scheduled, sender }
    }

    fn spawn<F>(&self, future: F)
        where F: Future<Output=()> + Send + 'static {
        Task::spawn(future, &self.sender);
    }

    fn run(&self) {
        while let Ok(task) = self.scheduled.recv() {
            println!("poll");
            if task.poll().is_ready() {
                break;
            }
        }
    }
}

async fn delay(dur: Duration) {
    let when = Instant::now() + dur;
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();

    thread::spawn(move || {
        let now = Instant::now();
        if now < when {
            thread::sleep(when - now);
        }
        notify2.notify_one();
    });

    notify.notified().await;
}
