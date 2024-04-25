use std::sync::Arc;

use crossbeam::queue::SegQueue;

use crate::Task;

#[derive(Clone, Debug)]
pub struct TaskQueue(Arc<SegQueue<Arc<Task>>>);

impl TaskQueue {
    pub fn new() -> Self {
        Self(Arc::new(SegQueue::new()))
    }

    pub fn pop(&self) -> Option<Arc<Task>> {
        self.0.pop()
    }

    pub fn push(&self, task: Arc<Task>) {
        self.0.push(task);
    }
}
