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

    pub fn measure(&mut self) -> [u8; 4] {
        loop {
            if let Ok(measurement) = self.dht.perform_measurement(&mut self.delay) {
                let mut total: [u8; 4] = [0;4];
                let (t, h) = total.split_at_mut(2);
                t.copy_from_slice(&measurement.temperature.to_le_bytes());
                h.copy_from_slice(&measurement.humidity.to_le_bytes());
                return total;
            }
        }
    }
}