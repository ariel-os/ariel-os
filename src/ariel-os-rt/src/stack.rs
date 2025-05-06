//! Stack usage helpers.
use core::{marker::PhantomData, ptr::write_volatile};

use crate::arch::sp;

/// Struct representing the currently active stack.
///
/// # Stack painting
///
/// [`Stack`] allows to measure the amount of stack effectively used through a technique called
/// stack painting:
///
/// 1. When initializing the memory stack, it is filled with a sequence of bytes of known values:
///    the paint.
/// 2. This paint gets covered during execution with the values stored on stack.
/// 3. When requested, the amount of covered paint is measured to estimate the amount of stack used
///    during execution.
///
/// Note that this technique only provides a lower bound of stack usage, as the values stored in
/// the stack may "collide" with the paint values.
/// In the current implementation, and assuming the stack data follows a uniform distribution, this
/// is unlikely to result in an underestimation of more than one byte.
///
/// # Note
///
/// The machinery for stack painting has a couple of assumptions:
///
/// 1. It is safe for an active stack to *overwrite* unused stack space from its limit (bottom,
///    including) to its stack pointer (not including) through raw pointers.
/// 2. It is safe to *read* unused stack space below the raw stack pointer down to its limit (bottom).
/// 3. The limits of an active stack never change.
/// 4. It is fine to specify zero for both `bottom` and `top`, in which case usage data is invalid
///    (always zero), but no unsafety arises.
/// 5. `!Send + !Sync` keeps `Stack` on the stack it was created for.
///
/// Both 1. and 2. are the case on cortex-m, risc-v and xtensa, as an ISR could technically do so at any time
///    anyways.
/// 3. is true on Ariel.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Stack {
    /// Lowest stack address
    bottom: usize,
    /// Highest stack address
    top: usize,

    /// Basically we need to ensure that `bottom` and `top` precisely correspond
    /// to the currently active stack. `!Send+!Sync` ensures that the object will
    /// be sent to another thread (or ISR), which implies it stays on the same stack.
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

    #[allow(dead_code, reason = "not always used due to conditional compilation")]
    pub(crate) const fn default() -> Self {
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
            // See assumptions in Struct level documentation.
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

        // sanity check, should never happen with `Stack` being `!Send`.
        // (This assert would not catch the case where a thread stack is created
        // on another thread's stack. `!Send+!Sync` still prevents this.)
        assert!(self.bottom <= sp && sp <= self.top);

        // Safety: `Stack` being `!Send+!Sync` should ensure that `repaint()` is only ever called
        // from the stack `self` was created on and belongs to. The assert above double-checks
        // this.
        // Given that `bottom` doesn't change (which it never does in Ariel OS while a stack is
        // in use), overwriting `bottom..sp` is safe on all our platforms, when `sp` points to the
        // current stack frame's stack pointer.
        // This does not prevent this from being interrupted by an ISR, in which case
        // the stack is dirtied again, but that doesn't cause any unsafety and just
        // makes any following `used_max()` call include whatever the ISR wrote on this stack.
        unsafe {
            for pos in self.bottom..sp {
                write_volatile(pos as *mut u8, 0xCC);
            }
        }
    }
}
