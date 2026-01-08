use embedded_hal::digital::OutputPin;

pub struct Led<PIN>
where
    PIN: OutputPin,
{
    pin: PIN,
}

impl<PIN> Led<PIN>
where
    PIN: OutputPin,
{
    pub fn new(pin: PIN) -> Self {
        Self { pin }
    }

    pub fn set(&mut self, on: bool) {
        if on {
            let _ =self.pin.set_high();
        } else {
            let _ = self.pin.set_low();
        }
    }
}
