//! This entire module is intended to be entirely auto-generated.

pub static STTS22H_I2C: ariel_os_sensor_stts22h::Stts22hI2c =
    const { ariel_os_sensor_stts22h::Stts22hI2c::new(Some("indoor")) };
#[ariel_os::reexports::linkme::distributed_slice(ariel_os::sensors::SENSOR_REFS)]
static STTS22H_I2C_REF: &'static dyn ariel_os::sensors::Sensor = &STTS22H_I2C;

#[ariel_os::task]
pub async fn stts22h_i2c_runner() {
    STTS22H_I2C.run().await
}
