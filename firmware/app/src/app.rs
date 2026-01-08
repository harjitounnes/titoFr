use esp_idf_hal::delay::FreeRtos;

use cores::Controller;
use crate::hardware::Hardware;
use drivers::wifi::WifiStatus;

/// Runtime application (logic only)
pub struct App {
    ctrl: Controller,
    hw: Hardware,
}

impl App {
    pub fn new(hw: Hardware) -> Self {
        Self {
            ctrl: Controller::new(),
            hw,
        }
    }

    pub fn run(&mut self) -> ! {
        // ---- boot sequence ----
        self.hw.display.init();
        self.hw.display.boot_screen();
        self.hw.display.show_led(false);
        self.hw.display.show_message(0, "Hello ESP32");

        let _ = self.hw.wifi.start();
        self.hw.display.clear_row(0, 16);
        self.hw.display.clear_row(1, 16);
        self.hw.display.show_message(0, "WiFi status:");
        self.hw.display.show_message(1, "Connecting...");

        // ---- main loop ----
        loop {
            let state = self.ctrl.toggle();
            self.hw.led.set(state);

            let status = self.hw.wifi.poll();
            let text = match status {
                WifiStatus::Connected => "Connected.",
                WifiStatus::Disconnected => "Failed.",
            };

            self.hw.display.clear_row(1, 16);
            self.hw.display.show_message(1, text);

            self.hw.led.set(true);
            FreeRtos::delay_ms(2000);
            self.hw.led.set(false);
            FreeRtos::delay_ms(2000);
            FreeRtos::delay_ms(1000);
        }
    }
}
