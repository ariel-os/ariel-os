//! This module is intended to contain the auto-@generated instantiation and registration of sensor
//! drivers.

pub async fn init() {
    #[cfg(any(context = "st-steval-mkboxpro"))]
    lis2du12::init().await;

    #[cfg(any(context = "st-steval-mkboxpro"))]
    lis2mdl::init().await;

    #[cfg(any(context = "st-steval-mkboxpro"))]
    lps22df::init().await;

    #[cfg(any(context = "st-steval-mkboxpro", context = "stm32u083c-dk"))]
    stts22h::init().await;
}

#[cfg(any(context = "st-steval-mkboxpro"))]
mod lis2du12 {
    use ariel_os::i2c::controller::I2cDevice;

    pub static LIS2DU12_I2C: ariel_os_sensor_lis2du12::i2c::Lis2du12<I2cDevice> =
        const { ariel_os_sensor_lis2du12::i2c::Lis2du12::new(Some("onboard")) };
    #[ariel_os::reexports::linkme::distributed_slice(ariel_os::sensors::SENSOR_REFS)]
    #[linkme(crate = ariel_os::reexports::linkme)]
    static LIS2DU12_I2C_REF: &'static dyn ariel_os::sensors::Sensor = &LIS2DU12_I2C;

    #[ariel_os::task(autostart)]
    pub async fn lis2du12_i2c_runner() {
        LIS2DU12_I2C.run().await
    }

    pub(super) async fn init() {
        let mut config = ariel_os_sensor_lis2du12::i2c::Config::default();
        config.address = {
            #[cfg(context = "st-steval-mkboxpro")]
            let address = ariel_os_sensor_lis2du12::i2c::I2cAddress::Sa0Vdd;
            address
        };

        LIS2DU12_I2C
            .init(
                ariel_os_sensor_lis2du12::i2c::Peripherals {},
                I2cDevice::new(crate::i2c_bus::I2C_BUS.get().unwrap()),
                config,
            )
            .await;
    }
}

#[allow(unused, reason = "should be directly accessible without going through the registry")]
#[cfg(any(context = "st-steval-mkboxpro"))]
pub use lis2du12::LIS2DU12_I2C;

#[cfg(any(context = "st-steval-mkboxpro"))]
mod lis2mdl {
    use ariel_os::i2c::controller::I2cDevice;

    pub static LIS2MDL_I2C: ariel_os_sensor_lis2mdl::i2c::Lis2mdl<I2cDevice> =
        const { ariel_os_sensor_lis2mdl::i2c::Lis2mdl::new(Some("onboard")) };
    #[ariel_os::reexports::linkme::distributed_slice(ariel_os::sensors::SENSOR_REFS)]
    #[linkme(crate = ariel_os::reexports::linkme)]
    static LIS2MDL_I2C_REF: &'static dyn ariel_os::sensors::Sensor = &LIS2MDL_I2C;

    #[ariel_os::task(autostart)]
    pub async fn lis2mdl_i2c_runner() {
        LIS2MDL_I2C.run().await
    }

    pub(super) async fn init() {
        let config = ariel_os_sensor_lis2mdl::i2c::Config::default();

        LIS2MDL_I2C
            .init(
                ariel_os_sensor_lis2mdl::i2c::Peripherals {},
                I2cDevice::new(crate::i2c_bus::I2C_BUS.get().unwrap()),
                config,
            )
            .await;
    }
}

#[allow(unused, reason = "should be directly accessible without going through the registry")]
#[cfg(any(context = "st-steval-mkboxpro"))]
pub use lis2mdl::LIS2MDL_I2C;

#[cfg(any(context = "st-steval-mkboxpro"))]
mod lps22df {
    use ariel_os::i2c::controller::I2cDevice;

    pub static LPS22DF_I2C: ariel_os_sensor_lps22df::i2c::Lps22df<I2cDevice> =
        const { ariel_os_sensor_lps22df::i2c::Lps22df::new(Some("onboard")) };
    #[ariel_os::reexports::linkme::distributed_slice(ariel_os::sensors::SENSOR_REFS)]
    #[linkme(crate = ariel_os::reexports::linkme)]
    static LPS22DF_I2C_REF: &'static dyn ariel_os::sensors::Sensor = &LPS22DF_I2C;

    #[ariel_os::task(autostart)]
    pub async fn lps22df_i2c_runner() {
        LPS22DF_I2C.run().await
    }

    pub(super) async fn init() {
        let mut config = ariel_os_sensor_lps22df::i2c::Config::default();
        config.address = {
            #[cfg(context = "st-steval-mkboxpro")]
            let address = ariel_os_sensor_lps22df::i2c::I2cAddress::Sa0Vdd;
            address
        };

        LPS22DF_I2C
            .init(
                ariel_os_sensor_lps22df::i2c::Peripherals {},
                I2cDevice::new(crate::i2c_bus::I2C_BUS.get().unwrap()),
                config,
            )
            .await;
    }
}

#[allow(unused, reason = "should be directly accessible without going through the registry")]
#[cfg(any(context = "st-steval-mkboxpro"))]
pub use lps22df::LPS22DF_I2C;

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
