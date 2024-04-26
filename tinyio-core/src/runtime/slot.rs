use super::worker::Worker;
use crate::util::signal::{Counter, Signal};

#[derive(Debug, Clone)]
pub struct ThreadSlot {
    #[allow(unused)]
    pub index: usize,
    pub worker: Option<Worker>,
}

impl ThreadSlot {
    pub fn make_worker(&mut self, counter: Counter) -> &Worker {
        self.worker = Some(Worker::new(counter));
        let worker = self.worker.as_ref().unwrap();
        worker.spawn();
        worker
    }
}

#[derive(Debug)]
pub struct ThreadSlots {
    capacity: usize,
    slots: Vec<ThreadSlot>,
    full: Signal,
}

impl ThreadSlots {
    pub fn new(capacity: usize) -> Self {
        let slots = (0..capacity)
            .map(|index| ThreadSlot {
                index,
                worker: None,
            })
            .collect();
        Self {
            capacity,
            slots,
            full: Signal::new(false),
        }
    }

    pub fn allocate(&mut self) -> &mut ThreadSlot {
        let capacity = self.capacity;
        let mut slots = self.slots.iter_mut();
        let full = &self.full;
        // if slots are full, then give a slot back
        if full.value() {
            slots.find(|slot| slot.worker.is_some()).unwrap()
        } else {
            // if slots are not full, give a slot back, and check if slots are full
            let slot = slots.find(|slot| slot.worker.is_none()).unwrap();
            if slot.index + 1 == capacity {
                full.set(true)
            }
            slot
        }
    }
}
