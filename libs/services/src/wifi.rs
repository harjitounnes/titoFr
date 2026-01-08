/// Abstraksi WiFi (kontrak)
use drivers::wifi::WifiStatus;

pub trait Wifi {
    fn connect(&mut self, ssid: &str, password: &str) -> WifiStatus;
    fn is_connected(&self) -> bool;
}

/// Service WiFi (logic only)
pub struct WifiService<W: Wifi> {
    wifi: W,
    ssid: heapless::String<32>,
    pass: heapless::String<64>,
}

impl<W: Wifi> WifiService<W> {
    pub fn new(wifi: W, ssid: &str, pass: &str) -> Self {
        let mut s = heapless::String::new();
        let mut p = heapless::String::new();
        s.push_str(ssid).ok();
        p.push_str(pass).ok();
        Self { wifi, ssid: s, pass: p }
    }

    pub fn start(&mut self) -> WifiStatus {
        self.wifi.connect(&self.ssid, &self.pass)
    }

    pub fn poll(&mut self) -> WifiStatus{
        if !self.wifi.is_connected() {
            log::warn!("WiFi disconnected, reconnecting...");
            self.wifi.connect(&self.ssid, &self.pass);

            if self.wifi.is_connected() {
                WifiStatus::Connected
            } else {
                WifiStatus::Disconnected
            }
        } else {
            WifiStatus::Connected
        }
    }
}
