//! Synchronization primitives.
mod channel;
mod event;
mod lock;
mod mutex;
mod wait_queue;

pub use channel::Channel;
pub use event::Event;
pub use lock::Lock;
pub use mutex::{Mutex, MutexGuard};
pub use wait_queue::WaitQueue;
