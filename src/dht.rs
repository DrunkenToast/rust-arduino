use arduino_hal::{port::{Pin, mode::OpenDrain}, hal::{port::{PD2}, delay::Delay}, clock::MHz16};
use dht11::{Dht11};

type DhtType = Dht11<Pin<OpenDrain, PD2>>;

pub struct Dht {
    dht: DhtType,
    delay: Delay<MHz16>,
}
impl Dht {
    pub fn new(dht: DhtType, delay: Delay<MHz16>) -> Self {
        Self { dht , delay }
    }

    pub fn temperature(&mut self) -> [u8; 2] {
        loop {
            if let Ok(measurement) = self.dht.perform_measurement(&mut self.delay) {
                return measurement.temperature.to_le_bytes()
            }
        }
    }

    pub fn humidity(&mut self) -> [u8; 2] {
        loop {
            if let Ok(measurement) = self.dht.perform_measurement(&mut self.delay) {
                return measurement.humidity.to_le_bytes()
            }
        }
    }
}