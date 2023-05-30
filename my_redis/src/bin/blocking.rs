use std::thread;
use std::time::Duration;
use tokio::runtime::Builder;
use tokio::spawn;
use tokio::sync::mpsc;
use tokio::time::sleep;

fn main() {
    let spawner = TaskSpawner::new();

    spawner.spawn_task(Task {
        name: String::from("test1"),
    });

    spawner.spawn_task(Task {
        name: String::from("test2"),
    });

    let runtime = Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();

    let mut handles = Vec::with_capacity(10);
    for i in 0..10 {
        handles.push(runtime.spawn(my_bg_task(i)));
    }

    thread::sleep(Duration::from_millis(750));
    println!("Finished time-consuming task.");

    for handle in handles {
        runtime.block_on(handle).unwrap();
    }
}

async fn my_bg_task(i: u64) {
    let mills = 1000 - 50 * i;
    println!("Task {} sleeping for {} ms", i, mills);
    sleep(Duration::from_millis(mills)).await;
    println!("Task {} stopping", i);
}

pub struct Task {
    name: String,
}

async fn handle_task(task: Task) {
    println!("Got task {:?}", task.name);
}

pub struct TaskSpawner {
    spawn: mpsc::Sender<Task>,
}

impl TaskSpawner {
    pub fn new() -> Self {
        let (send, mut recv) = mpsc::channel::<Task>(16);

        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        thread::spawn(move || {
            rt.block_on(async move {
                while let Some(task) = recv.recv().await {
                    spawn(handle_task(task));
                }
            });
        });

        Self { spawn: send }
    }

    pub fn spawn_task(&self, task: Task) {
        match self.spawn.blocking_send(task) {
            Ok(()) => {}
            Err(_) => panic!("The shared runtime has shut down."),
        }
    }
}
