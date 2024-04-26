use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Wake, Waker},
};

use crate::util::signal::{Counter, Signal};

use super::RUNTIME;

pub struct Task {
    future: Mutex<Option<Pin<Box<dyn Future<Output = ()> + 'static + Send>>>>,
    mark: Signal,
}

impl Wake for Task {
    fn wake_by_ref(self: &Arc<Self>) {
        RUNTIME.get().unwrap().push(self.clone());
    }

    fn wake(self: Arc<Self>) {
        RUNTIME.get().unwrap().push(self.clone());
    }
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static + Send) -> Self {
        Self {
            future: Mutex::new(Some(Box::pin(future))),
            mark: Signal::new(false),
        }
    }

    /// Increase the counter if the task is not marked, then mark the task.
    pub fn mark(self: &Arc<Self>, counter: &Counter) {
        if !self.mark.value() {
            counter.add(1);
            self.mark.set(true);
        }
    }

    /// Poll a task, if pending, send it back.
    pub fn try_poll(self: &Arc<Self>, counter: &Counter) {
        let mut future_slot = self.future.lock().unwrap();
        if let Some(mut future) = future_slot.take() {
            let waker = Waker::from(self.clone());
            let mut cx = Context::from_waker(&waker);
            if future.as_mut().poll(&mut cx).is_pending() {
                *future_slot = Some(future);
            } else {
                counter.sub(1);
            }
        }
    }
}
