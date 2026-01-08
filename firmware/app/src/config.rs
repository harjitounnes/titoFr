

use esp_idf_hal::{
    gpio::{PinDriver, Output, Gpio2},
    i2c::I2cDriver,
    delay::FreeRtos,
};
use services::wifi_config::{self, WifiConfig};
use cores::WifiAdapter;

pub type LedPin = PinDriver<'static, Gpio2, Output>;
pub type I2c = I2cDriver<'static>;
pub type Delay = FreeRtos;
pub type Wifi = WifiAdapter;

pub fn default_wifi() -> WifiConfig {
    let mut ssid = heapless::String::new();
    let mut pass = heapless::String::new();

    ssid.push_str("ICON+").ok();
    pass.push_str("M12@n0@mrq@15@r@").ok();

    WifiConfig { ssid, password: pass }
}

pub fn load_wifi() -> WifiConfig {
    match wifi_config::load_from_fs() {
        Ok(cfg) => cfg,
        Err(e) => {
            log::warn!("WiFi config not found, using default: {:?}", e);
            default_wifi()
        }
    }
}