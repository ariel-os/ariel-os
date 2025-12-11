//! This module is intended to contain the auto-@generated instantiation and registration of sensor
//! drivers.

pub async fn init() {
    // Sensor driver instances are to be initialized here.
    init_stts22h().await;
}

pub static STTS22H_I2C: ariel_os_sensor_stts22h::i2c::Stts22h<ariel_os::i2c::controller::I2cDevice> =
    const { ariel_os_sensor_stts22h::i2c::Stts22h::new(Some("indoor")) };
#[ariel_os::reexports::linkme::distributed_slice(ariel_os::sensors::SENSOR_REFS)]
#[linkme(crate = ariel_os::reexports::linkme)]
static STTS22H_I2C_REF: &'static dyn ariel_os::sensors::Sensor = &STTS22H_I2C;

#[ariel_os::task(autostart)]
pub async fn stts22h_i2c_runner() {
    STTS22H_I2C.run().await
}

async fn init_stts22h() {
    STTS22H_I2C
        .init(
            ariel_os_sensor_stts22h::i2c::Peripherals {},
            ariel_os::i2c::controller::I2cDevice::new(crate::i2c_bus::I2C_BUS.get().unwrap()),
            ariel_os_sensor_stts22h::i2c::Config::default(),
        )
        .await;
}