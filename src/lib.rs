use std::pin::Pin;
use std::sync::Mutex;
use std::task::{Context, Wake, Waker};
use std::{future::Future, sync::Arc};

use crossbeam::channel::{unbounded, Receiver, Sender};

pub struct Executor {
    rx: Receiver<Arc<Task>>,
}

impl Executor {
    pub fn run(&self) {
        while let Ok(task) = self.rx.recv() {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = Waker::from(task.clone());
                let mut cx = Context::from_waker(&waker);
                if future.as_mut().poll(&mut cx).is_pending() {
                    *future_slot = Some(future);
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Spawner {
    tx: Sender<Arc<Task>>,
}

impl Spawner {
    pub fn spawn(&self, task: impl Future<Output = ()> + 'static + Send) {
        let task = Task {
            future: Mutex::new(Some(Box::pin(task))),
            tx: self.tx.clone(),
        };
        self.tx.send(task.into()).unwrap();
    }
}

struct Task {
    future: Mutex<Option<Pin<Box<dyn Future<Output = ()> + 'static + Send>>>>,
    tx: Sender<Arc<Task>>,
}

impl Wake for Task {
    fn wake_by_ref(self: &Arc<Self>) {
        self.tx.send(self.clone()).unwrap();
    }

    fn wake(self: Arc<Self>) {
        self.tx.send(self.clone()).unwrap();
    }
}

pub fn init() -> (Executor, Spawner) {
    let (tx, rx) = unbounded();
    let executor = Executor { rx };
    let spawner = Spawner { tx };
    (executor, spawner)
}
