use embassy_stm32::eth::{Ethernet, GenericPhy, Sma};
use embassy_stm32::peripherals::{ETH, ETH_SMA};
use embassy_stm32::{bind_interrupts, eth};
use static_cell::StaticCell;

// Index of the builtin Ethernet MAC, used for generating the MAC address.
const IF_INDEX: u32 = 0;

bind_interrupts!(struct Irqs {
    ETH => eth::InterruptHandler;
});

pub type NetworkDevice = Ethernet<'static, ETH, GenericPhy<Sma<'static, ETH_SMA>>>;

pub fn device(peripherals: &mut crate::OptionalPeripherals) -> NetworkDevice {
    static PKTS: StaticCell<eth::PacketQueue<4, 4>> = StaticCell::new();

    let mac_addr = get_mac_address();

    Ethernet::new(
        PKTS.init(eth::PacketQueue::<4, 4>::new()),
        peripherals.ETH.take().unwrap(),
        Irqs,
        peripherals.PA1.take().unwrap(),
        peripherals.PA7.take().unwrap(),
        peripherals.PC4.take().unwrap(),
        peripherals.PC5.take().unwrap(),
        peripherals.PG13.take().unwrap(),
        peripherals.PB13.take().unwrap(),
        peripherals.PG11.take().unwrap(),
        mac_addr,
        peripherals.ETH_SMA.take().unwrap(),
        peripherals.PA2.take().unwrap(),
        peripherals.PC1.take().unwrap(),
    )
}

/// Returns a stable MAC address based on the device identity.
fn get_mac_address() -> [u8; 6] {
    use ariel_os_embassy_common::identity::DeviceId;

    // NOTE(no-panic): infallible on STM32.
    match crate::identity::DeviceId::get() {
        Ok(device_id) => device_id.interface_eui48(IF_INDEX).0,
        Err(_) => unreachable!(),
    }
}
