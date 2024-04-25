mod reexport;
mod tq;
pub use reexport::*;
use std::pin::Pin;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::task::{Context, Wake, Waker};
use std::{future::Future, sync::Arc};
use tq::TaskQueue;

static RUNTIME: OnceLock<TaskQueue> = OnceLock::new();

pub fn run() {
    let task_number = Arc::new(AtomicUsize::new(0));
    loop {
        match RUNTIME.get().unwrap().pop() {
            Some(task) => {
                {
                    let mut first = task.first.lock().unwrap();
                    if *first {
                        task_number.fetch_add(1, Ordering::AcqRel);
                        *first = false;
                    }
                }
                let task_number = task_number.clone();
                std::thread::spawn(move || {
                    let mut future_slot = task.future.lock().unwrap();
                    if let Some(mut future) = future_slot.take() {
                        let waker = Waker::from(task.clone());
                        let mut cx = Context::from_waker(&waker);
                        if future.as_mut().poll(&mut cx).is_pending() {
                            *future_slot = Some(future);
                        } else {
                            task_number.fetch_sub(1, Ordering::AcqRel);
                        }
                    }
                });
            }
            None => {
                if task_number.load(Ordering::Acquire) == 0 {
                    break;
                }
            }
        }
    }
}

struct Task {
    future: Mutex<Option<Pin<Box<dyn Future<Output = ()> + 'static + Send>>>>,
    first: Mutex<bool>,
}

impl Wake for Task {
    fn wake_by_ref(self: &Arc<Self>) {
        RUNTIME.get().unwrap().push(self.clone());
    }

    fn wake(self: Arc<Self>) {
        RUNTIME.get().unwrap().push(self.clone());
    }
}

pub fn init() {
    RUNTIME.set(TaskQueue::new()).unwrap();
}

pub fn spawn(task: impl Future<Output = ()> + 'static + Send) {
    let tq = RUNTIME.get().unwrap();
    let task = Task {
        future: Mutex::new(Some(Box::pin(task))),
        first: Mutex::new(true),
    };
    tq.push(Arc::new(task));
}
