use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub struct Memory {
    total_allocated: Arc<AtomicUsize>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            total_allocated: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn allocate(&self, size: usize) {
        self.total_allocated.fetch_add(size, Ordering::SeqCst);
    }

    pub fn deallocate(&self, size: usize) {
        self.total_allocated.fetch_sub(size, Ordering::SeqCst);
    }

    pub fn get_total_allocated(&self) -> usize {
        self.total_allocated.load(Ordering::SeqCst)
    }
}
