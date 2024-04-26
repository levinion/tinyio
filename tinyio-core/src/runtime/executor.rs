use super::{scheduler::Scheduler, RUNTIME};

#[derive(Debug, Default)]
pub struct Executor {
    scheduler: Scheduler,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            scheduler: Scheduler::default(),
        }
    }

    pub fn run(&mut self) {
        let backoff = crossbeam::utils::Backoff::new();
        loop {
            match RUNTIME.get().unwrap().pop() {
                Some(task) => {
                    backoff.reset();
                    self.scheduler.try_schedule(task);
                }
                None => {
                    if self.scheduler.done() {
                        break;
                    } else {
                        backoff.snooze();
                    }
                }
            }
        }
    }
}
