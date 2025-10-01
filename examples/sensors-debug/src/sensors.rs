//! This module is intended to contain the auto-@generated instantiation and registration of sensor
//! drivers.

// This example does not currently register any sensor drivers, they will be added later.

pub async fn init() {
    // Sensor driver instances are to be initialized here.
    init_lsm6dsv16x().await;
}

pub static LSM6DSV16X_I2C: ariel_os_sensor_lsm6dsv16x::i2c::Lsm6dsv16x =
    const { ariel_os_sensor_lsm6dsv16x::i2c::Lsm6dsv16x::new(Some("onboard")) };
#[ariel_os::reexports::linkme::distributed_slice(ariel_os::sensors::SENSOR_REFS)]
#[linkme(crate = ariel_os::reexports::linkme)]
static LSM6DSV16X_I2C_REF: &'static dyn ariel_os::sensors::Sensor = &LSM6DSV16X_I2C;

#[ariel_os::task(autostart)]
async fn lsm6dsv16x_i2c_runner() {
    LSM6DSV16X_I2C.run().await
}

pub async fn init_lsm6dsv16x() {
    let mut config = ariel_os_sensor_lsm6dsv16x::i2c::Config::default();
    config.address = ariel_os_sensor_lsm6dsv16x::i2c::I2cAddress::Sa0Vdd;
    LSM6DSV16X_I2C
        .init(
            ariel_os_sensor_lsm6dsv16x::i2c::Peripherals {},
            ariel_os::i2c::controller::I2cDevice::new(crate::i2c_bus::I2C_BUS.get().unwrap()),
            config,
        )
        .await;
}
