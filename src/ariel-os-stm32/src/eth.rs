use embassy_stm32::eth::{Ethernet, GenericPhy};
use embassy_stm32::peripherals::ETH;
use embassy_stm32::{bind_interrupts, eth};
use static_cell::StaticCell;

bind_interrupts!(struct Irqs {
    ETH => eth::InterruptHandler;
});

type MacAddress = [u8; 6];

pub type NetworkDevice = Ethernet<'static, ETH, GenericPhy>;

pub fn device(peripherals: &mut crate::OptionalPeripherals) -> NetworkDevice {
    static PKTS: StaticCell<eth::PacketQueue<4, 4>> = StaticCell::new();

    let mac_addr = generate_random_aai_mac_addr();

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

fn generate_random_aai_mac_addr() -> MacAddress {
    use rand_core::RngCore as _;

    let mut eui48 = [0u8; _];
    ariel_os_random::crypto_rng().fill_bytes(&mut eui48);

    // Enforce the `?2-??-??-??-??-??` pattern of an AAI (Administratively Assigned Identifier).
    eui48[0] &= 0xf0;
    eui48[0] |= 0x02;

    eui48
}
