use drivers::LcdI2c;
use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::{I2c, SevenBitAddress};

/// Service level abstraction (APP LOGIC)
pub struct LcdDisplay<I2C, D> {
    lcd: Option<LcdI2c<I2C>>,
    delay: D,
    error: bool,
}

impl<I2C, D> LcdDisplay<I2C, D>
where
    I2C: I2c<SevenBitAddress>,
    D: DelayNs,
{
    pub fn new(lcd: Option<LcdI2c<I2C>>, delay: D) -> Self {
        let error = lcd.is_none();
        Self { lcd, delay, error }
    }

    pub fn init(&mut self) {
        if let Some(lcd) = self.lcd.as_mut() {
            if lcd.clear(&mut self.delay).is_err() {
                self.lcd = None;
                self.error = true;
            }
        }
    }

    pub fn boot_screen(&mut self) {
        if let Some(lcd) = self.lcd.as_mut() {
            let _ = lcd.clear(&mut self.delay);
            let _ = lcd.set_cursor(0, 0, &mut self.delay);
            let _ = lcd.print("ESP32-C3 Mini", &mut self.delay);

            let _ = lcd.set_cursor(0, 1, &mut self.delay);
            let _ = lcd.print("System Ready", &mut self.delay);
        }
    }

    pub fn show_led(&mut self, on: bool) {
        if let Some(lcd) = self.lcd.as_mut() {
            let _ = lcd.set_cursor(0, 1, &mut self.delay);
            let _ = lcd.print(
            if on { "LED: ON " } else { "LED: OFF" },
            &mut self.delay);
        }
    }

    pub fn show_message(&mut self, line: u8, msg: &str) {
        if let Some(lcd) = self.lcd.as_mut() {
            let _ = lcd.set_cursor(0, line, &mut self.delay);
            let _ = lcd.print(msg, &mut self.delay);
        }
    }

    pub fn clear_row(&mut self, row: u8, cols: usize) {
        let blank = " ".repeat(cols);
        self.show_message(row, &blank);
    }

}
