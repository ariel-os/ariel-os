//! This module is intended to contain the auto-@generated instantiation and registration of sensor
//! drivers.

pub async fn init() {
    #[cfg(any(context = "st-steval-mkboxpro", context = "stm32u083c-dk"))]
    stts22h::init().await;

    #[cfg(any(context = "st-steval-mkboxpro"))]
    lsm6dsv16x::init().await;
}

#[cfg(any(context = "st-steval-mkboxpro", context = "stm32u083c-dk"))]
mod stts22h {
    use ariel_os::i2c::controller::I2cDevice;

    pub static STTS22H_I2C: ariel_os_sensor_stts22h::i2c::Stts22h<I2cDevice> =
        const { ariel_os_sensor_stts22h::i2c::Stts22h::new(Some("onboard")) };
    #[ariel_os::reexports::linkme::distributed_slice(ariel_os::sensors::SENSOR_REFS)]
    #[linkme(crate = ariel_os::reexports::linkme)]
    static STTS22H_I2C_REF: &'static dyn ariel_os::sensors::Sensor = &STTS22H_I2C;

    #[ariel_os::task(autostart)]
    pub async fn stts22h_i2c_runner() {
        STTS22H_I2C.run().await
    }

    pub(super) async fn init() {
        let mut config = ariel_os_sensor_stts22h::i2c::Config::default();
        config.address = {
            #[cfg(context = "st-steval-mkboxpro")]
            let address = ariel_os_sensor_stts22h::i2c::I2cAddress::AddrVdd;
            #[cfg(context = "stm32u083c-dk")]
            let address = ariel_os_sensor_stts22h::i2c::I2cAddress::AddrGnd;
            address
        };

        STTS22H_I2C
            .init(
                ariel_os_sensor_stts22h::i2c::Peripherals {},
                I2cDevice::new(crate::i2c_bus::I2C_BUS.get().unwrap()),
                config,
            )
            .await;
    }
}

#[allow(unused, reason = "should be directly accessible without going through the registry")]
#[cfg(any(context = "st-steval-mkboxpro", context = "stm32u083c-dk"))]
pub use stts22h::STTS22H_I2C;

#[cfg(any(context = "st-steval-mkboxpro"))]
mod lsm6dsv16x {
    use ariel_os::i2c::controller::I2cDevice;

    pub static LSM6DSV16X_I2C: ariel_os_sensor_lsm6dsv16x::i2c::Lsm6dsv16x<I2cDevice> =
        const { ariel_os_sensor_lsm6dsv16x::i2c::Lsm6dsv16x::new(Some("onboard")) };
    #[ariel_os::reexports::linkme::distributed_slice(ariel_os::sensors::SENSOR_REFS)]
    #[linkme(crate = ariel_os::reexports::linkme)]
    static LSM6DSV16X_I2C_REF: &'static dyn ariel_os::sensors::Sensor = &LSM6DSV16X_I2C;

    #[ariel_os::task(autostart)]
    async fn lsm6dsv16x_i2c_runner() {
        LSM6DSV16X_I2C.run().await
    }

    pub(super) async fn init() {
        let mut config = ariel_os_sensor_lsm6dsv16x::i2c::Config::default();
        #[cfg(any(context = "st-steval-mkboxpro"))]
        let address = ariel_os_sensor_lsm6dsv16x::i2c::I2cAddress::Sa0Vdd;
        config.address = address;
        LSM6DSV16X_I2C
            .init(
                ariel_os_sensor_lsm6dsv16x::i2c::Peripherals {},
                ariel_os::i2c::controller::I2cDevice::new(crate::i2c_bus::I2C_BUS.get().unwrap()),
                config,
            )
            .await;
    }
}

#[allow(unused, reason = "should be directly accessible without going through the registry")]
#[cfg(any(context = "st-steval-mkboxpro"))]
pub use lsm6dsv16x::LSM6DSV16X_I2C;
