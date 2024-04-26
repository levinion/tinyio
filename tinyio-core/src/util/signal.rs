use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc,
};

#[derive(Debug, Default)]
pub struct Counter(Arc<AtomicUsize>);

impl Counter {
    pub fn new(n: usize) -> Self {
        Self(Arc::new(AtomicUsize::new(n)))
    }

    pub fn add(&self, n: usize) {
        self.0.fetch_add(n, Ordering::AcqRel);
    }

    pub fn sub(&self, n: usize) {
        self.0.fetch_sub(n, Ordering::AcqRel);
    }

    pub fn value(&self) -> usize {
        self.0.load(Ordering::Acquire)
    }
}

impl Clone for Counter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[derive(Debug)]
pub struct Signal(AtomicBool);

impl Signal {
    pub fn new(value: bool) -> Self {
        Self(AtomicBool::new(value))
    }

    pub fn value(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }

    pub fn set(&self, value: bool) {
        self.0.store(value, Ordering::Release);
    }
}
