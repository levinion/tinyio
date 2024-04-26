use std::sync::Arc;

use crate::util::signal::Counter;

use super::{slot::ThreadSlots, task::Task};

#[derive(Debug)]
pub struct Scheduler {
    #[allow(unused)]
    capacity: usize,
    slots: ThreadSlots,
    counter: Counter,
}

impl Scheduler {
    pub fn new(capacity: usize) -> Self {
        let slots = ThreadSlots::new(capacity);
        let counter = Counter::new(0);
        Self {
            capacity,
            slots,
            counter,
        }
    }

    pub fn try_schedule(&mut self, task: Arc<Task>) -> Option<()> {
        task.mark(&self.counter);
        let slot = self.slots.allocate();
        match &slot.worker {
            Some(worker) => {
                worker.send(task);
            }
            None => {
                let worker = slot.make_worker(self.counter.clone());
                worker.send(task);
            }
        }
        Some(())
    }

    pub fn done(&self) -> bool {
        self.counter.value() == 0
    }

    pub fn task_number(&self) -> usize {
        self.counter.value()
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        let capacity = num_cpus::get() / 2;
        Self::new(capacity)
    }
}
