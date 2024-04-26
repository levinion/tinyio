use std::sync::Arc;

use crossbeam::queue::SegQueue;

use crate::runtime::task::Task;

#[derive(Clone, Debug, Default)]
pub struct TaskQueue {
    queue: Arc<SegQueue<Arc<Task>>>,
}

impl TaskQueue {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn pop(&self) -> Option<Arc<Task>> {
        self.queue.pop()
    }

    pub fn push(&self, task: Arc<Task>) {
        self.queue.push(task);
    }
}
