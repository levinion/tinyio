pub mod executor;
pub mod scheduler;
mod slot;
pub mod task;
pub mod tq;
mod worker;

use std::sync::OnceLock;

use self::tq::TaskQueue;

pub static RUNTIME: OnceLock<TaskQueue> = OnceLock::new();
