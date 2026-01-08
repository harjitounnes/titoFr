use esp_idf_svc::wifi::{EspWifi, ClientConfiguration, Configuration};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_hal::modem::Modem;
use esp_idf_svc::sys::EspError;
use heapless::String as HString;

pub struct WifiDriver {
    wifi: EspWifi<'static>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WifiStatus {
    Connected,
    Disconnected,
}

impl WifiDriver {
    pub fn new(
        modem: Modem,
        sysloop: EspSystemEventLoop,
        nvs: EspDefaultNvsPartition,
    ) -> Result<Self, EspError> {
        let wifi = EspWifi::new(modem, sysloop, Some(nvs))?;
        Ok(Self { wifi })
    }

    pub fn connect(&mut self, ssid: &str, password: &str) -> Result<WifiStatus, EspError> {
        let mut ssid_buf: HString<32> = HString::new();
        let mut pass_buf: HString<64> = HString::new();

        let _ = ssid_buf
            .push_str(ssid);

        let _ = pass_buf
            .push_str(password);

        let cfg = Configuration::Client(ClientConfiguration {
            ssid: ssid_buf,
            password: pass_buf,
            ..Default::default()
        });

        self.wifi.set_configuration(&cfg)?;
        self.wifi.start()?;
        self.wifi.connect()?;

        // Ok(())
        if self.wifi.is_connected().unwrap_or(false) {
            Ok(WifiStatus::Connected)
        } else {
            Ok(WifiStatus::Disconnected)
        }
    }

    pub fn is_connected(&self) -> bool {
        self.wifi.is_connected().unwrap_or(false)
    }

    pub fn disconnect(&mut self) -> Result<(), EspError> {
        self.wifi.disconnect()?;
        Ok(())
    }
}
