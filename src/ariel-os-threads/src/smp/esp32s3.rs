#![expect(unsafe_code)]

use esp_hal::{
    interrupt,
    peripherals::{CPU_CTRL, Interrupt},
    system::{Cpu, CpuControl, Stack},
};

use super::{CoreId, ISR_STACKSIZE_CORE1, Multicore, StackLimits};

impl From<Cpu> for CoreId {
    fn from(value: Cpu) -> Self {
        match value {
            Cpu::ProCpu => CoreId(0),
            Cpu::AppCpu => CoreId(1),
        }
    }
}

pub struct Chip;

impl Multicore for Chip {
    const CORES: u32 = 2;
    const IDLE_THREAD_STACK_SIZE: usize = 2048;
    type Stack = Stack<ISR_STACKSIZE_CORE1>;

    fn core_id() -> CoreId {
        Cpu::current().into()
    }

    fn startup_other_cores(stack: &'static mut Self::Stack) {
        // Trigger scheduler.
        let start_threading = move || {
            // Use `CPU_INTR1` to trigger the scheduler on our second core.
            // We need to use a different interrupt here than on the first core so that
            // we specifically trigger the scheduler on one or the other core.
            interrupt::disable(Cpu::ProCpu, Interrupt::FROM_CPU_INTR1);
            Self::schedule_on_core(Self::core_id());
            // Panics if `FROM_CPU_INTR1` is among `esp_hal::interrupt::RESERVED_INTERRUPTS`,
            // which isn't the case.
            interrupt::enable(Interrupt::FROM_CPU_INTR1, interrupt::Priority::min()).unwrap();

            unreachable!()
        };

        let mut cpu_ctrl = unsafe { CpuControl::new(CPU_CTRL::steal()) };
        let guard = cpu_ctrl.start_app_core(stack, start_threading);

        // Dropping the guard would park the other core.
        core::mem::forget(guard)
    }

    fn schedule_on_core(id: CoreId) {
        let ptr = esp_hal::peripherals::SYSTEM::regs();
        let mut id = id.0;
        let already_set = ptr
            .cpu_intr_from_cpu(id.into())
            .read()
            .cpu_intr()
            .bit_is_set();

        if already_set {
            // If a scheduling attempt is already pending, there must have been multiple
            // changes in the runqueue at the same time.
            // Trigger the scheduler on the other core as well to make sure that both schedulers
            // have the most recent runqueue state.
            id ^= 1;
        }
        ptr.cpu_intr_from_cpu(id.into())
            .write(|w| w.cpu_intr().set_bit());
    }
}

impl<const SIZE: usize> StackLimits for Stack<SIZE> {
    fn limits(&self) -> (usize, usize) {
        let lowest = self.mem.as_ptr() as usize;
        let highest = lowest + self.len();
        (lowest, highest)
    }
}
