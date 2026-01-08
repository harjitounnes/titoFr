use services::wifi::Wifi;
use drivers::WifiDriver;
use drivers::wifi::WifiStatus;

pub struct WifiAdapter(pub WifiDriver);

impl Wifi for WifiAdapter{
    fn connect(&mut self, ssid: &str, password: &str)  -> WifiStatus {
        let _ = self.0.connect(ssid, password);

        if self.0.is_connected() {
            WifiStatus::Connected
        } else {
            WifiStatus::Disconnected
        }
    }
    
    fn is_connected(&self) -> bool {
        self.0.is_connected()
    }
}
