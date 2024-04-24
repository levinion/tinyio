use crossbeam::channel::{unbounded, Receiver, Sender};
use std::{
    sync::{Arc, Mutex},
    thread,
};

#[derive(Clone)]
struct Runtime {
    rx: Receiver<Task>,
}

impl Runtime {
    pub fn new(rx: Receiver<Task>) -> Self {
        Self { rx }
    }

    fn run(&self) {
        loop {
            if let Ok(task) = self.rx.recv() {
                thread::spawn(move || task.call());
            }
        }
    }
}

pub struct Spawner {
    tx: Sender<Task>,
}

impl Spawner {
    fn new(tx: Sender<Task>) -> Self {
        Self { tx }
    }

    pub fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + Sync + 'static,
    {
        let task = Task::new(f);
        self.tx.send(task).unwrap();
    }
}

type SyncFn = Arc<Mutex<Option<Box<dyn FnOnce() + Send + Sync + 'static>>>>;

struct Task {
    func: SyncFn,
}

impl Task {
    fn new<F>(f: F) -> Self
    where
        F: FnOnce() + Send + Sync + 'static,
    {
        Self {
            func: Arc::new(Mutex::new(Some(Box::new(f)))),
        }
    }

    fn call(&self) {
        let func = self.func.clone();
        let mut func = func.lock().unwrap();
        let f = func.take().unwrap();
        f();
    }
}

pub fn init() -> Spawner {
    let (tx, rx) = unbounded();
    let runtime = Runtime::new(rx);
    thread::spawn({
        let runtime = runtime.clone();
        move || {
            runtime.run();
        }
    });
    Spawner::new(tx)
}
