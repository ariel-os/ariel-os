use embassy_stm32::eth::Ethernet;
use embassy_stm32::eth::generic_smi::GenericSMI;
use embassy_stm32::peripherals::ETH;
use embassy_stm32::{bind_interrupts, eth};
use static_cell::StaticCell;

bind_interrupts!(struct Irqs {
    ETH => eth::InterruptHandler;
});

pub type NetworkDevice = Ethernet<'static, ETH, GenericSMI>;

pub fn device(peripherals: &mut crate::OptionalPeripherals) -> NetworkDevice {
    static PKTS: StaticCell<eth::PacketQueue<4, 4>> = StaticCell::new();

    let mac_addr = [0xCA, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC];

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
        eth::generic_smi::GenericSMI::new(0),
        mac_addr,
    )
}
