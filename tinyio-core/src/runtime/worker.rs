use std::sync::Arc;

use crossbeam::utils::Backoff;

use crate::util::signal::Counter;

use super::{task::Task, tq::TaskQueue};

#[derive(Clone, Debug)]
pub struct Worker {
    queue: TaskQueue,
    counter: Counter,
}

impl Worker {
    pub fn new(counter: Counter) -> Self {
        let queue = TaskQueue::new();
        Self { queue, counter }
    }

    pub fn spawn(&self) {
        std::thread::spawn({
            let counter = self.counter.clone();
            let queue = self.queue.clone();
            move || {
                let backoff = Backoff::new();
                loop {
                    match queue.pop() {
                        Some(task) => {
                            backoff.reset();
                            task.try_poll(&counter);
                        }
                        None => backoff.snooze(),
                    }
                }
            }
        });
    }

    pub fn send(&self, task: Arc<Task>) {
        self.queue.push(task);
    }
}
