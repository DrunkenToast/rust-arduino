use arduino_hal::{I2c, Delay};

use crate::{lcd_i2c::{Lcd, Backlight}, millis::millis};

type LcdType = Lcd<I2c, Delay>;

pub struct Display {
    lcd: LcdType,
    last_received: u32,
}

impl Display {
    pub fn new(lcd: LcdType) -> Self {
        Display { lcd, last_received: 0 }
    }

    pub fn write_message(&mut self, message: &str) {
        self.lcd.clear().unwrap();
        self.last_received = millis();
        self.lcd.set_backlight(Backlight::On);

        if message.len() > 16 {
            self.lcd.write_str(&message[..16]).unwrap();
            self.lcd.set_cursor(1, 0).unwrap();
            self.lcd.write_str(&message[16..]).unwrap();
        }
        else {
            self.lcd.write_str(message).unwrap();
        }
    }

    pub fn check_state(&mut self) {
        if self.lcd.get_backlight() == Backlight::On
        && self.last_received + 5000 < millis() {
            self.lcd.set_backlight(Backlight::Off);
        }
    }
}