use arduino_hal::prelude::*;

// https://github.com/Rahix/avr-hal/blob/main/examples/arduino-uno/src/bin/uno-panic.rs
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    avr_device::interrupt::disable();

    let dp = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(dp);
    //let mut serial = arduino_hal::default_serial!(dp, pins, 9600);

    //ufmt::uwriteln!(&mut serial, "Panicked!\r").void_unwrap();
    //if let Some(loc) = info.location() {
        //ufmt::uwriteln!(&mut serial,
            //"At {}:{}:{}\r",
        //loc.file(), loc.line(), loc.column()).void_unwrap();
    //}
    //serial.flush();

    let mut led = pins.d13.into_output();
    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
    }

}