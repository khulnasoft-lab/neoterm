use std::sync::atomic::{AtomicUsize, Ordering};

// A unique ID for each block
static BLOCK_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

// Represents a single command and its output
#[derive(Debug, Clone)]
pub struct Block {
    pub id: usize,
    pub command: String,
    pub output: String,
    pub status: BlockStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockStatus {
    Running,
    Finished,
}

impl Block {
    pub fn new(command: String) -> Self {
        Self {
            id: BLOCK_ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            command,
            output: String::new(),
            status: BlockStatus::Running,
        }
    }
} 