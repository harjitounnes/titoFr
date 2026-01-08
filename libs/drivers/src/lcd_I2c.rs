#![allow(dead_code)]

use embedded_hal::i2c::I2c;
use embedded_hal::delay::DelayNs;

pub struct LcdI2c<I2C> {
    i2c: I2C,
    addr: u8,
    backlight: bool,
}

impl<I2C> LcdI2c<I2C>
where
    I2C: I2c,
{
    pub fn new<D>(i2c: I2C, addr: u8, delay: &mut D) -> Result<Self, ()>
    where
        D: DelayNs,
    {
        let mut lcd = Self {
            i2c,
            addr,
            backlight: true,
        };

        // Init sequence (4-bit mode)
        delay.delay_ms(50);

        lcd.write_nibble(0x03, false, delay)?;
        delay.delay_ms(5);

        lcd.write_nibble(0x03, false, delay)?;
        delay.delay_us(150);

        lcd.write_nibble(0x03, false, delay)?;
        delay.delay_us(150);

        lcd.write_nibble(0x02, false, delay)?;
        delay.delay_us(150);

        lcd.command(0x28, delay)?;
        lcd.command(0x0C, delay)?;
        lcd.command(0x06, delay)?;
        lcd.clear(delay)?;

        Ok(lcd)
    }

    pub fn clear<D: DelayNs>(&mut self, delay: &mut D) -> Result<(), ()> {
        self.command(0x01, delay)?;
        delay.delay_ns(2_000_000);
        Ok(())
    }

    pub fn set_cursor<D: DelayNs>(&mut self, col: u8, row: u8, delay: &mut D) -> Result<(), ()> {
        let row_offset = if row == 0 { 0x00 } else { 0x40 };
        self.command(0x80 | (col + row_offset), delay)
    }

    pub fn print<D: DelayNs>(&mut self, text: &str, delay: &mut D) -> Result<(), ()> {
        for b in text.bytes() {
            self.data(b, delay)?;
        }
        Ok(())
    }

    // ───────── low-level ─────────

    fn command<D: DelayNs>(&mut self, cmd: u8, delay: &mut D) -> Result<(), ()> {
        self.write_byte(cmd, false, delay)
    }

    fn data<D: DelayNs>(&mut self, data: u8, delay: &mut D) -> Result<(), ()> {
        self.write_byte(data, true, delay)
    }

    fn write_byte<D: DelayNs>(&mut self, val: u8, rs: bool, delay: &mut D) -> Result<(), ()> {
        self.write_nibble(val >> 4, rs, delay)?;
        self.write_nibble(val & 0x0F, rs, delay)?;
        Ok(())
    }

    fn write_nibble<D: DelayNs>(&mut self, nibble: u8, rs: bool, delay: &mut D) -> Result<(), ()> {
        let mut data = (nibble & 0x0F) << 4;

        if rs {
            data |= 1 << 0;
        }
        if self.backlight {
            data |= 1 << 3;
        }

        self.pulse_enable(data, delay)
    }

    fn pulse_enable<D: DelayNs>(&mut self, data: u8, delay: &mut D) -> Result<(), ()> {
        self.write_raw(data | (1 << 2))?;
        delay.delay_ns(1_000);
        self.write_raw(data & !(1 << 2))?;
        delay.delay_ns(50_000);
        Ok(())
    }

    fn write_raw(&mut self, data: u8) -> Result<(), ()> {
        self.i2c.write(self.addr, &[data]).map_err(|_| ())
    }
}