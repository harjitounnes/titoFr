#![cfg_attr(not(test), no_std)]

#[cfg(test)]
extern crate std;

pub mod display;
pub mod wifi;
pub mod wifi_config;

pub use display::LcdDisplay;
pub use wifi::WifiService;
pub use wifi_config::load_from_fs;
// ================= UNIT TESTS =================

#[cfg(test)]
mod tests {
    use super::LcdDisplay;
    use drivers::LcdI2c;

    use embedded_hal::i2c::{
        I2c, SevenBitAddress, ErrorType as I2cErrorType, Operation,
    };
    use embedded_hal::delay::DelayNs;

    use core::convert::Infallible;
    use std::cell::RefCell;
    use std::rc::Rc;

    // ===== mock delay =====
    struct TestDelay;
    impl DelayNs for TestDelay {
        fn delay_ns(&mut self, _ns: u32) {}
    }

    // ===== mock i2c =====
    #[derive(Clone)]
    struct TestI2c {
        writes: Rc<RefCell<u32>>,
    }

    impl TestI2c {
        fn new() -> Self {
            Self {
                writes: Rc::new(RefCell::new(0)),
            }
        }

        fn count(&self) -> u32 {
            *self.writes.borrow()
        }
    }

    impl I2cErrorType for TestI2c {
        type Error = Infallible;
    }

    impl I2c<SevenBitAddress> for TestI2c {
        fn write(
            &mut self,
            _addr: SevenBitAddress,
            _bytes: &[u8],
        ) -> Result<(), Self::Error> {
            *self.writes.borrow_mut() += 1;
            Ok(())
        }

        fn read(
            &mut self,
            _addr: SevenBitAddress,
            _buf: &mut [u8],
        ) -> Result<(), Self::Error> {
            Ok(())
        }

        fn write_read(
            &mut self,
            _addr: SevenBitAddress,
            _bytes: &[u8],
            _buf: &mut [u8],
        ) -> Result<(), Self::Error> {
            Ok(())
        }

        fn transaction(
            &mut self,
            _addr: SevenBitAddress,
            ops: &mut [Operation<'_>],
        ) -> Result<(), Self::Error> {
            for op in ops {
                if let Operation::Write(_) = op {
                    *self.writes.borrow_mut() += 1;
                }
            }
            Ok(())
        }
    }

    // ================= TEST DISPLAY =================

    #[test]
    fn display_boot_screen_writes_i2c() {
        let i2c = TestI2c::new();
        let mut delay = TestDelay;

        let lcd = LcdI2c::new(i2c.clone(), 0x27, &mut delay)
            .expect("lcd init failed");

        let mut display = LcdDisplay::new(lcd, delay);

        let before = i2c.count();
        display.boot_screen();
        let after = i2c.count();

        assert!(after > before);
    }

    #[test]
    fn display_show_message_writes_i2c() {
        let i2c = TestI2c::new();
        let mut delay = TestDelay;

        let lcd = LcdI2c::new(i2c.clone(), 0x27, &mut delay)
            .expect("lcd init failed");

        let mut display = LcdDisplay::new(lcd, delay);

        let before = i2c.count();
        display.show_message(0, "Hello");
        display.show_message(1, "ESP32");

        let after = i2c.count();

        assert!(after > before);
    }


    // // ================= TEST WIFI =================

    // #[derive(Default)]
    // struct MockWifi {
    //     connected: bool,
    //     last_ssid: Option<String>,
    // }

    // impl Wifi for MockWifi {
    //     fn connect(&mut self, ssid: &str, _password: &str) {
    //         self.connected = true;
    //         self.last_ssid = Some(ssid.to_string());
    //     }

    //     fn is_connected(&self) -> bool {
    //         self.connected
    //     }
    // }

    // #[test]
    // fn wifi_connect_sets_connected_true() {
    //     let mock = MockWifi::default();
    //     let mut service = WifiService::new(mock);

    //     service.connect("MySSID", "secret");

    //     assert!(service.is_connected());
    // }

    // #[test]
    // fn wifi_connect_stores_ssid() {
    //     let mock = MockWifi::default();
    //     let mut service = WifiService::new(mock);

    //     service.connect("OfficeWiFi", "12345678");

    //     // akses ke mock via service (pattern: destructure)
    //     let WifiService { wifi } = service;
    //     assert_eq!(wifi.last_ssid.as_deref(), Some("OfficeWiFi"));
    // }
}

