use critical_section::CriticalSection;

use crate::{RunqueueId, SCHEDULER, Scheduler, ThreadId, ThreadState, thread::Thread};

/// Manages blocked [`super::Thread`]s for a resource, and triggering the scheduler when needed.
#[derive(Debug, Default)]
pub struct ThreadList {
    /// Next thread to run once the resource is available.
    head: Option<ThreadId>,
}

impl ThreadList {
    /// Creates a new empty [`ThreadList`].
    pub const fn new() -> Self {
        Self { head: None }
    }

    /// Puts the current (blocked) thread into this [`ThreadList`] and triggers the scheduler.
    ///
    /// Returns a `RunqueueId` if the highest priority among the waiters in the list has changed.
    ///
    /// # Panics
    ///
    /// Panics if this is called outside of a thread context.
    pub fn put_current(
        &mut self,
        cs: CriticalSection<'_>,
        state: ThreadState,
    ) -> Option<RunqueueId> {
        SCHEDULER.with_mut_cs(cs, |mut scheduler| {
            let &mut Thread { tid, prio, .. } = scheduler
                .current()
                .expect("Function should be called inside a thread context.");
            let mut curr = None;
            let mut next = self.head;
            while let Some(n) = next {
                if scheduler.get_unchecked_mut(n).prio < prio {
                    break;
                }
                curr = next;
                next = scheduler.thread_blocklist[usize::from(n)];
            }
            scheduler.thread_blocklist[usize::from(tid)] = next;
            let inherit_priority = if let Some(curr) = curr {
                scheduler.thread_blocklist[usize::from(curr)] = Some(tid);
                None
            } else {
                self.head = Some(tid);
                Some(prio)
            };
            scheduler.set_state(tid, state);
            inherit_priority
        })
    }

    /// Removes the head from this [`ThreadList`].
    ///
    /// Sets the thread's [`ThreadState`] to [`ThreadState::Running`] and triggers
    /// the scheduler.
    ///
    /// Returns the thread's [`ThreadId`] and its previous [`ThreadState`].
    pub fn pop(&mut self, cs: CriticalSection<'_>) -> Option<(ThreadId, ThreadState)> {
        let head = self.head?;
        SCHEDULER.with_mut_cs(cs, |mut scheduler| {
            self.head = scheduler.thread_blocklist[usize::from(head)].take();
            let old_state = scheduler.set_state(head, ThreadState::Running);
            Some((head, old_state))
        })
    }

    fn remove_inner(&mut self, scheduler: &mut Scheduler, thread_id: ThreadId) -> bool {
        ariel_os_debug::log::trace!("remove_current() {:?}", thread_id);
        if let Some(head) = self.head {
            if head == thread_id {
                self.head = scheduler.thread_blocklist[usize::from(head)].take();
                scheduler.set_state(head, ThreadState::Running);
                return true;
            }
            let mut cur = head;
            while let Some(next) = scheduler.thread_blocklist[usize::from(cur)] {
                if next == thread_id {
                    scheduler.thread_blocklist[usize::from(cur)] =
                        scheduler.thread_blocklist[usize::from(next)].take();
                    scheduler.set_state(next, ThreadState::Running);
                    return true;
                }
                cur = next;
            }
        }
        false
    }

    /// Removes the current thread from this [`ThreadList`].
    ///
    /// ## Panics
    /// Panics if this is called outside of a thread context.
    pub(crate) fn remove_current(&mut self, cs: CriticalSection<'_>) -> bool {
        SCHEDULER.with_mut_cs(cs, |mut scheduler| {
            let thread_id = scheduler
                .current_tid()
                .expect("Function should be called inside a thread context.");

            self.remove_inner(&mut scheduler, thread_id)
        })
    }

    /// Determines if this [`ThreadList`] is empty.
    pub fn is_empty(&self, _cs: CriticalSection<'_>) -> bool {
        self.head.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_type_sizes() {
        assert_eq!(size_of::<ThreadId>(), 1);
        assert_eq!(size_of::<ThreadList>(), 2);
    }
}
