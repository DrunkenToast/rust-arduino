#![no_std]
#![no_main]

mod dht;
mod serial;
mod display_i2c;


use arduino_hal::{port::{mode::{Output}, Pin}, hal::{port::{PB5}}, I2c, Delay};
use dht::Dht;
use dht11::Dht11;
use panic_halt as _;
use embedded_hal::{digital::v2::{OutputPin, PinState}};
use display_i2c::Lcd;
use serial::Serial;

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

type Led = Pin<Output, PB5>;
type Display = Lcd<I2c, Delay>;

const LCD_ADDR: u8 = 0x27;

#[arduino_hal::entry]
fn main() -> ! {
    // Init Arduino peripherals
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led: Led = pins.d13.into_output();
    let mut serial: Serial = Serial::new(arduino_hal::default_serial!(dp, pins, 9600));

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

    // DHT11 sensor
    let mut dht = Dht::new(
        Dht11::new(pins.d2.into_opendrain()), 
        arduino_hal::Delay::new()
    );

    // Say wait for connection with master
    loop {
        serial.write_action(Action::Hello);
        
        let b = serial.read();
        if let Ok(Action::Hello) = Action::try_from(b) {
            serial.write_u8(Action::Recv as u8);
            break;
        }
    }

    // Main loop
    loop {
        handle_action(&mut serial, &mut led, &mut display, &mut dht);
    }
    //ufmt::uwrite!(&mut serial, "Hello, world!\r\n").void_unwrap();
}

fn handle_action(serial: &mut Serial, led: &mut Led, display: &mut Display, dht: &mut Dht) {
    let action = Action::try_from(serial.read());

    match action {
        Ok(Action::Hello) => {},
        Ok(Action::SwitchLed) => {
            let state: u8 = serial.read();
            if state != 0 {led.set_state(PinState::High).unwrap();}
            else {led.set_state(PinState::Low).unwrap();}
        },
        Ok(Action::DisplayMessage) => {
            let amt_bytes = serial.read().clamp(0, 32); 
            for _ in 0..amt_bytes {
                led.toggle();
            }
            let mut message: [u8; 32] = [0; 32];
            let message = &mut message[..amt_bytes as usize];
            for i in 0..amt_bytes {
                message[i as usize] = serial.read();
            }
            write_message(core::str::from_utf8(message).unwrap(), display);
        },
        Ok(Action::ReadTemperature) => {
            serial.write(&dht.temperature());
        },
        Ok(Action::ReadHumidity) => {
            serial.write(&dht.humidity())
        },
        Ok(Action::Recv) => {},
        Err(_) => {}
    }

    if action.is_ok() {
        serial.write_action(Action::Recv);
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
        display.write_str(message).unwrap();
    }
}