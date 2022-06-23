#![no_std]
#![no_main]

use arduino_hal::{prelude::*, port::{mode::{Output, Input}, Pin}, hal::{port::{PB5, PD0, PD1}, Atmega}, pac::USART0, clock::MHz16, Peripherals, I2c, Delay};
use arduino_hal::Usart;
use arduino_hal::Pins;
use panic_halt as _;
use embedded_hal::{serial::Read, digital::v2::{OutputPin, PinState}};
use display_i2c::Lcd;

mod display_i2c;

#[repr(u8)]
enum Action {
    SwitchLed,
    DisplayMessage,
    ReadTemperature,
    ReadHumidity,
}

impl TryFrom<u8> for Action {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= Action::ReadHumidity as u8 {
            unsafe {
                Ok(core::mem::transmute(value))
            }
        } else {
            Err(())
        }
    }
}
type Serial = Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>>;
type Led = Pin<Output, PB5>;
type Display = Lcd<I2c, Delay>;

struct Arduino {
   serial: Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>>,
   led: Pin<Output, PB5>,
   lcd: Lcd<I2c, Delay>,
}

impl Arduino {
    fn new() -> Self {
        const LCD_ADDR: u8 = 0x27;
        let dp = arduino_hal::Peripherals::take().unwrap();
        let pins = arduino_hal::pins!(dp);
        let led = pins.d13.into_output();
        let serial = arduino_hal::default_serial!(dp, pins, 115200);

        let i2c = arduino_hal::I2c::new(
            dp.TWI,
            pins.a4.into_pull_up_input(),
            pins.a5.into_pull_up_input(),
            50_000,
        );

        let delay = arduino_hal::Delay::new();

        let lcd = Lcd::new(i2c, delay)
            .address(LCD_ADDR)
            .cursor_on(false)
            .rows(2)
            .init().unwrap();

        Self {
            led,
            serial,
            lcd, 
        }
    }

    pub fn write_message(&mut self, message: &str) {
        self.lcd.clear().unwrap();
        if message.len() > 16 {
            self.lcd.write_str(&message[..16]).unwrap();
            self.lcd.set_cursor(1, 0).unwrap();
            self.lcd.write_str(&message[16..]).unwrap();
        }
        else {
            self.lcd.write_str(&message).unwrap();
        }
    }
}

const LCD_ADDR: u8 = 0x27;

#[arduino_hal::entry]
fn main() -> ! {
    // Init Arduino peripherals
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let led: Led = pins.d13.into_output();
    let serial: Serial = arduino_hal::default_serial!(dp, pins, 115200);

    let i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50_000,
    );

    let delay = arduino_hal::Delay::new();

    let lcd: Display = Lcd::new(i2c, delay)
        .address(LCD_ADDR)
        .cursor_on(false)
        .rows(2)
        .init().unwrap();

    // Main loop
    //arduino.write_message("Hello, world! This is a pretty long message");
    //ufmt::uwrite!(&mut serial, "Hello, world!\r\n").void_unwrap();
    loop {
        handle_action(read_action(&mut arduino).unwrap(), &mut arduino);
    }
}

fn read_action(serial: Serial) -> Result<Action, ()> {
    let b: u8 = serial.read_byte();
    Action::try_from(b)
}

fn handle_action(action: Action, serial: Serial, led: Led, display: Display) {
    match action {
        Action::SwitchLed => {
            let state: u8 = serial.read_byte();
            if state != 0 {led.set_state(PinState::High).unwrap();}
            else {led.set_state(PinState::Low).unwrap();}
        },
        Action::DisplayMessage => {
            let amt_bytes = serial.read_byte().clamp(0, 32); 
            for _ in 0..amt_bytes {
                led.toggle();
            }
            let mut message: [u8; 32] = ['\0' as u8; 32];
            let message = &mut message[..amt_bytes as usize];
            for i in 0..amt_bytes {
                message[i as usize] = serial.read_byte();
            }
            write_message(core::str::from_utf8(&message).unwrap(), display);
        },
        Action::ReadTemperature => {
            
        },
        Action::ReadHumidity => {
            
        }
    }
}

pub fn write_message(message: &str, display: Display) {
    display.clear().unwrap();
    if message.len() > 16 {
        display.write_str(&message[..16]).unwrap();
        display.set_cursor(1, 0).unwrap();
        display.write_str(&message[16..]).unwrap();
    }
    else {
        display.write_str(&message).unwrap();
    }
}