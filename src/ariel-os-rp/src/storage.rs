use embassy_rp::{
    bind_interrupts,
    flash::Async,
    peripherals::{DMA_CH1, FLASH},
};

pub type Flash = embassy_rp::flash::Flash<'static, FLASH, Async, FLASH_SIZE>;
pub type FlashError = embassy_rp::flash::Error;

const FLASH_SIZE: usize = 2 * 1024 * 1024;

bind_interrupts!(struct Irqs {
    DMA_IRQ_0 => embassy_rp::dma::InterruptHandler<DMA_CH1>;
});

pub fn init(p: &mut crate::OptionalPeripherals) -> Flash {
    embassy_rp::flash::Flash::<_, Async, FLASH_SIZE>::new(
        p.FLASH.take().unwrap(),
        p.DMA_CH1.take().unwrap(),
        Irqs,
    )
}
