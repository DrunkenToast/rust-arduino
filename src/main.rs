#![no_std]
#![no_main]

use core::time::Duration;

use arduino_hal::{prelude::*, port::{mode::{Output, Input}, Pin}, hal::{port::{PB5, PD0, PD1}, Atmega}, pac::USART0, clock::MHz16, Peripherals, I2c, Delay};
use arduino_hal::Usart;
use panic_halt as _;
use embedded_hal::{serial::Read, digital::v2::{OutputPin, PinState}};
use display_i2c::Lcd;

mod display_i2c;

#[repr(u8)]
enum Action {
    Hello = 0,
    SwitchLed = 1,
    DisplayMessage = 2,
    ReadTemperature = 3,
    ReadHumidity = 4,
    Recv = 5,
}

impl TryFrom<u8> for Action {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Action::Hello),
            1 => Ok(Action::SwitchLed),
            2 => Ok(Action::DisplayMessage),
            3 => Ok(Action::ReadTemperature),
            4 => Ok(Action::ReadHumidity),
            5 => Ok(Action::Recv),
            _ => Err(()),
        }
    }
}

type Serial = Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>>;
type Led = Pin<Output, PB5>;
type Display = Lcd<I2c, Delay>;

const LCD_ADDR: u8 = 0x27;

#[arduino_hal::entry]
fn main() -> ! {
    // Init Arduino peripherals
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led: Led = pins.d13.into_output();
    let mut serial: Serial = arduino_hal::default_serial!(dp, pins, 115200);

    let i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50_000,
    );

    let delay_lcd = arduino_hal::Delay::new();

    let mut display: Display = Lcd::new(i2c, delay_lcd)
        .address(LCD_ADDR)
        .cursor_on(false)
        .rows(2)
        .init().unwrap();

    
    // Say wait for connection with master
    loop {
        serial.write_byte(Action::Hello as u8);
        
        match serial.read() {
            Ok(b) => {
                if let Ok(Action::Hello) = Action::try_from(b) {
                    serial.write_byte(Action::Recv as u8);
                    break;
                }
            }
            Err(_) => {}
        }
    }
    // Main loop
    loop {
        handle_action(&mut serial, &mut led, &mut display);
    }


    //ufmt::uwrite!(&mut serial, "Hello, world!\r\n").void_unwrap();
}

fn handle_action(serial: &mut Serial, led: &mut Led, display: &mut Display) {
    match Action::try_from(serial.read_byte()) {
        Ok(Action::Hello) => {},
        Ok(Action::SwitchLed) => {
            let state: u8 = serial.read_byte();
            if state != 0 {led.set_state(PinState::High).unwrap();}
            else {led.set_state(PinState::Low).unwrap();}
            serial.write_byte(Action::Recv as u8);
        },
        Ok(Action::DisplayMessage) => {
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
        Ok(Action::ReadTemperature) => {
           
        },
        Ok(Action::ReadHumidity) => {
            
        },
        Ok(Action::Recv) => {},
        Err(_) => {}
    }
}

pub fn write_message(message: &str, display: &mut Display) {
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