use arduino_hal::{Usart, pac::USART0, port::{Pin, mode::{Input, Output}}, hal::port::{PD0, PD1}};
type SerialType = Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>>;

pub struct Serial {
    serial: SerialType,
}

impl Serial {
    pub fn new(&mut self, serial: SerialType) -> Self {
        Self { serial }
    }

    pub fn read_u8(&mut self) -> u8 {
        self.serial.read_byte()
    }

    pub fn write_u8(&mut self, b: u8) {
        self.serial.write_byte(b)
    }

    pub fn write(&mut self, buf: &[u8]) {
        for b in buf {
            self.serial.write_byte(*b);
        }
    }
}