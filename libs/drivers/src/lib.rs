#![cfg_attr(not(test), no_std)]

#[cfg(test)]
extern crate std;

pub mod led;
pub mod lcd_i2c;
pub mod wifi;
// pub mod ble;

pub use led::Led;
pub use lcd_i2c::LcdI2c;
pub use wifi::WifiDriver;
// pub use ble::BleDriver;

// ================= UNIT TESTS =================

#[cfg(test)]
mod tests {
    use super::{Led, LcdI2c}; // ✅ FIX #1

    use embedded_hal::digital::{OutputPin, ErrorType as DigitalErrorType};
    use embedded_hal::i2c::{I2c, SevenBitAddress, ErrorType as I2cErrorType, Operation};
    use embedded_hal::delay::DelayNs;

    use core::convert::Infallible;
    
    use std::sync::{Arc, Mutex};
    use std::cell::RefCell;
    use std::rc::Rc;

    // ===== LED TEST (STYLE MINI) =====

    #[derive(Clone)]
    struct TestPin {
        state: Arc<Mutex<bool>>,
    }

    impl DigitalErrorType for TestPin {
        type Error = Infallible;
    }

    impl OutputPin for TestPin {
        fn set_high(&mut self) -> Result<(), Self::Error> {
            *self.state.lock().unwrap() = true;
            Ok(())
        }

        fn set_low(&mut self) -> Result<(), Self::Error> {
            *self.state.lock().unwrap() = false;
            Ok(())
        }
    }

    #[test]
    fn led_on_sets_pin_high() {
        let state = Arc::new(Mutex::new(false));
        let pin = TestPin { state: state.clone() };

        let mut led = Led::new(pin);
        led.set(true);

        assert_eq!(*state.lock().unwrap(), true);
    }

    #[test]
    fn led_off_sets_pin_low() {
        let state = Arc::new(Mutex::new(true));
        let pin = TestPin { state: state.clone() };

        let mut led = Led::new(pin);
        led.set(false);

        assert_eq!(*state.lock().unwrap(), false);
    }

    // ===== LCD TEST (STYLE MINI) =====

    struct TestDelay;

    impl DelayNs for TestDelay {
        fn delay_ns(&mut self, _ns: u32) {}
    }

    #[derive(Clone)]
    struct TestI2c {
        writes: Rc<RefCell<u32>>,
    }

    impl I2cErrorType for TestI2c {
        type Error = Infallible;
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
            operations: &mut [Operation<'_>],
        ) -> Result<(), Self::Error> {
            // hitung setiap Write sebagai aktivitas I2C
            for op in operations {
                if let Operation::Write(_) = op {
                    *self.writes.borrow_mut() += 1;
                }
            }
            Ok(())
        }
    }

    #[test]
    fn lcd_init_writes_i2c() {
        let i2c = TestI2c::new();
        let mut delay = TestDelay;

        let _lcd = LcdI2c::new(i2c.clone(), 0x27, &mut delay)
            .expect("lcd init failed");

        assert!(i2c.count() > 0);
    }

    #[test]
    fn lcd_print_writes_data() {
        let i2c = TestI2c::new();
        let mut delay = TestDelay;

        let mut lcd = LcdI2c::new(i2c.clone(), 0x27, &mut delay)
            .unwrap();

        let before = i2c.count();
        let _ = lcd.print("Hi", &mut delay);
        let after = i2c.count();

        assert!(after > before);
    }
}
