//! This entire module is intended to be entirely auto-generated.

pub static NRF91_GNSS: ariel_os_nrf91_gnss::Nrf91Gnss =
    const { ariel_os_nrf91_gnss::Nrf91Gnss::new(Some("Gnss")) };
#[ariel_os::reexports::linkme::distributed_slice(ariel_os::sensors::SENSOR_REFS)]
static NRF91_GNSS_REF: &'static dyn ariel_os::sensors::Sensor = &NRF91_GNSS;

#[ariel_os::task]
pub async fn nrf91_gnss_runner() {
    NRF91_GNSS.run().await
}
