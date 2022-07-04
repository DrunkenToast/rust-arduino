use arduino_hal::{Usart, pac::USART0, port::{Pin, mode::{Input, Output}}, hal::port::{PD0, PD1}};
use embedded_hal::prelude::*;
use arduino_hal::prelude::*;
use crate::Action;
type SerialType = Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>>;

pub struct Serial {
    serial: SerialType,
}

impl Serial {
    pub fn new(serial: SerialType) -> Self {
        Self { serial }
    }

    pub fn read(&mut self) -> u8 {
        self.serial.read_byte()
    }

    pub fn read_no_block(&mut self) -> Result<u8, ()> {
        if let Ok(val) = self.serial.read() {
            Ok(val)
        }
        else {
            Err(())
        }
    }

    pub fn write_u8(&mut self, b: u8) {
        self.serial.write_byte(b)
    }

    pub fn write(&mut self, buf: &[u8]) {
        for b in buf {
            self.serial.write_byte(*b);
        }
    }

    pub fn write_action(&mut self, action: Action) {
        self.write_u8(action as u8);
    }
}