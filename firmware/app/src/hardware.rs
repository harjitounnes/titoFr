use anyhow::Result;

use esp_idf_hal::{
    peripherals::Peripherals,
    gpio::PinDriver,
    i2c::*,
    delay::FreeRtos,
    prelude::FromValueType,
};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
};

use drivers::{Led, LcdI2c, WifiDriver};
use services::{LcdDisplay, WifiService};
use cores::{WifiAdapter};

use crate::config::{self, LedPin, I2c, Delay, Wifi};

pub struct Hardware {
    pub led: Led<LedPin>,
    pub display: LcdDisplay<I2c, Delay>,
    pub wifi: WifiService<Wifi>,
}

pub fn init() -> Result<Hardware> {
    let peripherals = Peripherals::take()?;

    // ===== LED =====
    let gpio_led = PinDriver::output(peripherals.pins.gpio2)?;
    let led = Led::new(gpio_led);

    // ===== I2C + LCD =====
    let i2c = I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio8,
        peripherals.pins.gpio9,
        &I2cConfig::new().baudrate(100_u32.kHz().into()),
    )?;

    let mut delay = FreeRtos;
    let lcd = match LcdI2c::new(i2c, 0x27, &mut delay) {
        Ok(lcd) => Some(lcd),
        Err(_) => {
            println!("LCD init failed, disabled");
            None
        }
    };

    let display = LcdDisplay::new(lcd, delay);

    // ===== WiFi =====
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let wifi_driver = WifiDriver::new(
        peripherals.modem,
        sysloop,
        nvs,
    )?;

    let wifi = WifiAdapter(wifi_driver);
    let wifi_cfg = config::load_wifi();
    let wifi = WifiService::new(
        wifi, 
        &wifi_cfg.ssid, 
        &wifi_cfg.password,
    );

    Ok(Hardware { led, display, wifi })
}
