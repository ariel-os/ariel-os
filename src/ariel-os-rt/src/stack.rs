//! Stack usage helpers.
use core::{marker::PhantomData, ptr::write_volatile};

use crate::arch::sp;

/// Struct representing the currently active stack.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Stack {
    /// Lowest stack address
    pub bottom: usize,
    /// Highest stack address
    pub top: usize,

    /// We'll be implementing nasty stuff on this Struct that requires it to
    /// not get sent across stacks.
    _not_send: PhantomData<*const ()>,
}

impl Stack {
    /// Gets a handle for the currently active stack.
    pub fn get() -> Self {
        let sp = sp();
        let stack = crate::arch::stack();
        if stack.size() > 0 {
            assert!(stack.top >= stack.bottom);

            // TODO: verify bounds (are they inclusive?)
            assert!(stack.bottom <= sp && stack.top >= sp);
        }
        stack
    }

    pub const fn default() -> Self {
        Self {
            bottom: 0,
            top: 0,
            _not_send: PhantomData,
        }
    }

    pub(crate) const fn new(bottom: usize, top: usize) -> Self {
        Self {
            bottom,
            top,
            _not_send: PhantomData,
        }
    }

    /// Returns the total size of the current stack.
    pub fn size(&self) -> usize {
        self.top - self.bottom
    }

    /// Returns the amount of currently free stack space.
    pub fn free(&self) -> usize {
        self.size() - self.used()
    }

    /// Returns the amount of currently used stack space.
    pub fn used(&self) -> usize {
        self.top - sp()
    }

    /// Returns the minimum free stack space since last repaint.
    ///
    /// This re-calculates and thus runs in `O(n)`!
    pub fn free_min(&self) -> usize {
        let mut free = 0usize;
        for pos in self.bottom..self.top {
            // Safety: dereferencing ptr to valid memory, read only
            if unsafe { *(pos as *const u8) } == 0xCC {
                free += 1;
            }
        }
        free
    }

    /// Returns the maximum stack space used since last repaint.
    ///
    /// Equivalent to `size() - free_min()`.
    ///
    /// This re-calculates and thus runs in `O(n)`!
    pub fn used_max(&self) -> usize {
        self.size() - self.free_min()
    }

    /// Repaints the stack.
    pub fn repaint(&self) {
        let sp = crate::arch::sp();
        if self.size() == 0 {
            return;
        }

        // sanity check
        assert!(self.bottom <= sp && sp <= self.top);

        // Safety: writing to inactive part of active is fine.
        unsafe {
            for pos in self.bottom..sp {
                write_volatile(pos as *mut u8, 0xCC);
            }
        }
    }
}
