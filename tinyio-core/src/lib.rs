mod reexport;
pub mod runtime;
mod util;
pub use reexport::*;
use runtime::RUNTIME;
use runtime::{task::Task, tq::TaskQueue};
use std::{future::Future, sync::Arc};

pub fn init() {
    RUNTIME.set(TaskQueue::new()).unwrap();
}

pub fn spawn(future: impl Future<Output = ()> + 'static + Send) {
    let tq = RUNTIME.get().unwrap();
    let task = Task::new(future);
    tq.push(Arc::new(task));
}
