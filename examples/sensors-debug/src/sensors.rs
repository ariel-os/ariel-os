//! This module is intended to contain the auto-@generated instantiation and registration of sensor
//! drivers.

pub static LSM6DSV16X_I2C: ariel_os_sensor_lsm6dsv16x::i2c::Lsm6dsv16x =
    const { ariel_os_sensor_lsm6dsv16x::i2c::Lsm6dsv16x::new(Some("onboard")) };
#[ariel_os::reexports::linkme::distributed_slice(ariel_os::sensors::SENSOR_REFS)]
#[linkme(crate = ariel_os::reexports::linkme)]
static LSM6DSV16X_I2C_REF: &'static dyn ariel_os::sensors::Sensor = &LSM6DSV16X_I2C;

#[ariel_os::task(autostart)]
async fn lsm6dsv16x_i2c_runner() {
    LSM6DSV16X_I2C.run().await
}
