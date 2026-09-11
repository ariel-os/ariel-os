use embassy_stm32::eth::{Ethernet, GenericPhy};
use embassy_stm32::peripherals::ETH;
use embassy_stm32::{bind_interrupts, eth};
use static_cell::StaticCell;

use ariel_os_embassy_common::ethernet::MAC_ADDRESS_DEVICE_ID_IF_INDEX;

bind_interrupts!(struct Irqs {
    ETH => eth::InterruptHandler;
});

pub type NetworkDevice = Ethernet<'static, ETH, GenericPhy>;

pub fn device(peripherals: &mut crate::OptionalPeripherals) -> NetworkDevice {
    static PKTS: StaticCell<eth::PacketQueue<4, 4>> = StaticCell::new();

    let mac_addr = get_mac_address();

    Ethernet::new(
        PKTS.init(eth::PacketQueue::<4, 4>::new()),
        peripherals.ETH.take().unwrap(),
        Irqs,
        peripherals.PA1.take().unwrap(),
        peripherals.PA2.take().unwrap(),
        peripherals.PC1.take().unwrap(),
        peripherals.PA7.take().unwrap(),
        peripherals.PC4.take().unwrap(),
        peripherals.PC5.take().unwrap(),
        peripherals.PG13.take().unwrap(),
        peripherals.PB13.take().unwrap(),
        peripherals.PG11.take().unwrap(),
        GenericPhy::new(0),
        mac_addr,
    )
}

/// Returns a stable MAC address based on the device identity.
// NOTE: Keep in sync with the general implementation in `ariel-os-embassy`.
fn get_mac_address() -> [u8; 6] {
    use ariel_os_embassy_common::identity::DeviceId as _;

    // NOTE(no-panic): infallible on STM32.
    match crate::identity::DeviceId::get() {
        Ok(device_id) => device_id.interface_eui48(MAC_ADDRESS_DEVICE_ID_IF_INDEX).0,
        Err(_) => unreachable!(),
    }
}
