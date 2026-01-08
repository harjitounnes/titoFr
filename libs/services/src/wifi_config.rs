use heapless::String;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct WifiConfig {
    pub ssid: String<32>,
    pub password: String<64>,
}

pub fn load_from_fs() -> Result<WifiConfig> {
    // nanti implement LittleFS di sini
    Err(anyhow::anyhow!("wifi config not found"))
}
