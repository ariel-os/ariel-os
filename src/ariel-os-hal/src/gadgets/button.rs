//! Push button

use crate::gadgets::switch::IntEnabledSwitchPin;

use embassy_time::Timer;
use embassy_futures::select::{Either, select};

pub struct Button<'a> {
    switch: IntEnabledSwitchPin<'a>,
}

impl<'a> Button<'a> {
    #[must_use]
    pub fn new(switch: IntEnabledSwitchPin<'a>) -> Self {
        Self { switch }
    }

    pub async fn wait_for_double_click(&mut self, max_gap_ms: u64) {
        loop {
            self.switch.wait_for_activation_edge().await;
            match select(self.switch.wait_for_activation_edge(), Timer::after_millis(max_gap_ms)).await {
                Either::First(_) => break,
                Either::Second(_) => {}
            }
        }
    }

    crate::impl_intswitch_inner!(switch);
}

#[macro_export]
macro_rules! impl_intbutton_inner {
    ($inner:ident) => {
        pub async fn wait_for_double_click(&mut self, max_gap_ms: u64) {
            self.$inner.wait_for_double_click(max_gap_ms).await
        }
    };
}
