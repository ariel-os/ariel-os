#![expect(unsafe_code)]

use std::cell::Cell;

use std::sync::atomic::{AtomicU32, Ordering};

use crate::{Arch, SCHEDULER, ThreadId, ThreadState, thread::Thread};

pub struct Cpu;

mod critical_section;

static THREAD_RUNNABLE: [AtomicU32; crate::THREAD_COUNT] =
    [const { AtomicU32::new(0) }; crate::THREAD_COUNT];

#[derive(Debug)]
pub struct ThreadData {
    thread: Option<std::thread::Thread>,
}

impl ThreadData {
    pub const fn new() -> Self {
        Self { thread: None }
    }
}

impl ThreadData {
    thread_local! {
        pub static ID: Cell<Option<ThreadId>> = const { Cell::new(None) };
    }
}

impl Arch for Cpu {
    type ThreadData = ThreadData;
    const DEFAULT_THREAD_DATA: Self::ThreadData = ThreadData::new();

    fn setup_stack(thread: &mut Thread, _stack: &mut [u8], func: fn(), arg: Option<usize>) {
        let thread_id = thread.tid;

        let handle = std::thread::spawn(move || {
            ThreadData::ID.with(|x| x.set(Some(thread_id)));
            atomic_wait::wait(&THREAD_RUNNABLE[usize::from(thread_id)], 0);

            // We use catch_unwind here to catch if a thread panics.
            // In that case, we abort the process, which is as close as it gets
            // to an actual MCU target panicking.
            let res = std::panic::catch_unwind(|| {
                if let Some(arg) = arg {
                    // SAFETY:
                    // Going through *const(), transmuting (itself) between any two pointer types is safe.
                    //
                    // Whether this results in a function pointer that is safe to call:
                    //
                    // If `arg.is_some()`, `func` has been transmuted from `fn(T: Arguable + Send)`,
                    // here we transmute it again into a function pointer with one `usize` argument.
                    //
                    // According to
                    // https://doc.rust-lang.org/stable/std/primitive.fn.html#abi-compatibility,
                    // this "function call behaves as if every argument was transmuted from the
                    // type in the function pointer to the type at the function declaration".
                    //
                    // The safety of this transmute is enforced by the `Arguable` trait.
                    let func: fn(usize) = unsafe { core::mem::transmute(func as *const ()) };

                    func(arg)
                } else {
                    func()
                }
            });

            if res.is_err() {
                ariel_os_debug::log::error!("thread {:?} panicked, aborting.", thread_id);
                std::process::abort();
            }

            SCHEDULER.with_mut(|mut scheduler| {
                scheduler.set_state(thread_id, ThreadState::Invalid);
            });
        });

        thread.data.thread = Some(handle.thread().clone());
    }

    fn start_threading() {
        loop {
            SCHEDULER.with(|scheduler| {
                for (n, thread) in scheduler.threads.iter().enumerate() {
                    if thread.state == ThreadState::Running {
                        if THREAD_RUNNABLE[n].swap(1, Ordering::Acquire) == 0 {
                            atomic_wait::wake_one(&THREAD_RUNNABLE[n]);
                        }
                    }
                }
            });

            std::thread::park();
        }
    }

    fn schedule() {}

    fn wfi() {
        unimplemented!()
    }

    fn set_running(thread_id: ThreadId) {
        let n = usize::from(thread_id);
        if THREAD_RUNNABLE[n].swap(1, Ordering::Acquire) == 0 {
            atomic_wait::wake_one(&THREAD_RUNNABLE[n]);
        }
    }

    fn set_stopped(thread_id: ThreadId) {
        THREAD_RUNNABLE[usize::from(thread_id)].store(0, Ordering::Release);
    }
}
