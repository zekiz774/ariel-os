use ariel_os::hal::{i2c, peripherals};

#[cfg(context = "esp")]
pub type SensorI2c = i2c::controller::I2C0;
#[cfg(context = "esp")]
ariel_os::hal::define_peripherals!(Peripherals {
    i2c_sda: GPIO2,
    i2c_scl: GPIO0,
});

#[cfg(any(context = "nrf52833", context = "nrf52840"))]
pub type SensorI2c = i2c::controller::TWISPI0;
#[cfg(any(context = "nrf5340", context = "nrf9160"))]
pub type SensorI2c = i2c::controller::SERIAL0;
#[cfg(all(context = "nrf", not(context = "bbc-microbit-v2")))]
ariel_os::hal::define_peripherals!(Peripherals {
    i2c_sda: P0_00,
    i2c_scl: P0_01,
});
#[cfg(context = "bbc-microbit-v2")]
ariel_os::hal::define_peripherals!(Peripherals {
    i2c_sda: P0_16,
    i2c_scl: P0_08,
});

#[cfg(context = "rp")]
pub type SensorI2c = i2c::controller::I2C0;
#[cfg(context = "rp")]
ariel_os::hal::define_peripherals!(Peripherals {
    i2c_sda: PIN_12,
    i2c_scl: PIN_13,
});

#[cfg(context = "stm32h755zitx")]
pub type SensorI2c = i2c::controller::I2C1;
#[cfg(context = "stm32h755zitx")]
ariel_os::hal::define_peripherals!(Peripherals {
    i2c_sda: PB9,
    i2c_scl: PB8,
});

#[cfg(context = "stm32wb55rgvx")]
pub type SensorI2c = i2c::controller::I2C1;
#[cfg(context = "stm32wb55rgvx")]
ariel_os::hal::define_peripherals!(Peripherals {
    i2c_sda: PB9,
    i2c_scl: PB8,
});
