/// Module providing core affinity (thread pinning) functionality.
use crate::CoreId;

/// Affinity mask that defines on what cores a thread can be scheduled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CoreAffinity(u8);

impl CoreAffinity {
    /// Allows a thread to be scheduled on any core and to migrate
    /// from one core to another between executions.
    #[cfg(feature = "multi-core")]
    #[must_use]
    pub const fn no_affinity() -> Self {
        use crate::smp::Chip;
        use crate::smp::Multicore;
        Self(2u8.pow(Chip::CORES) - 1)
    }

    /// Allows a thread to be scheduled on any core and to migrate
    /// from one core to another between executions.
    #[cfg(not(feature = "multi-core"))]
    #[must_use]
    pub const fn no_affinity() -> Self {
        Self(2u8.pow(1) - 1)
    }

    /// Restricts the thread execution to a specific core.
    ///
    /// The thread can only be scheduled on this core, even
    /// if other cores are idle or execute a lower priority thread.
    #[must_use]
    pub fn one(core: CoreId) -> Self {
        Self(1 << core.0)
    }

    /// Checks if the affinity mask "allows" this `core`.
    #[must_use]
    pub fn contains(&self, core: CoreId) -> bool {
        self.0 & (1 << core.0) > 0
    }
}

impl Default for CoreAffinity {
    fn default() -> Self {
        Self::no_affinity()
    }
}
