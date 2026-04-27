//! Common types for networking in Ariel OS.

use embassy_net::Stack;

/// Allows to control the state of a network interface.
/// The network interface should be enabled by default at startup.
pub trait InterfaceController: Copy {
    /// Enable a previously disabled network interface.
    fn enable(&self);
    /// Disable this network interface.
    /// Whether the interface is fully powered down or not depends on the implementation.
    fn disable(&self);
}

/// A network interface.
#[derive(Clone, Copy)]
pub struct NetworkInterface<'a, C: InterfaceController> {
    stack: Stack<'a>,
    controller: C,
}
impl<'a, C: InterfaceController> NetworkInterface<'a, C> {
    /// Create a new interface from the stack and interface struct.
    pub fn new(stack: Stack<'a>, controller: C) -> Self {
        Self { stack, controller }
    }

    /// Get the [`embassy_net::Stack`] for this interface.
    pub fn network_stack(&self) -> Stack<'a> {
        self.stack
    }

    /// Enable a previously disabled [`NetworkInterface`].
    pub fn enable(&self) {
        self.controller.enable();
    }
    /// Disable a previously disabled [`NetworkInterface`].
    /// Whether the interface is fully powered down or not depends on the implementation.
    pub fn disable(&self) {
        self.controller.disable();
    }
}
