#![no_std]
#![no_main]

use arduino_hal::delay_ms;
use arduino_hal::pac::TC1;
use bitbang_hal::serial::Serial;
use embedded_hal::timer::{CountDown, Periodic};
use embedded_hal::serial::Write;
use panic_halt as _;

// For now, hard code the timer frequency.
struct Timer {
    inner: TC1
}

impl CountDown for Timer {
    type Time = ();

    fn start<T>(&mut self, _: T)
    where
        T: Into<Self::Time>
    {
        self.inner.tccr1b.write(|w| w.cs1().direct());
        self.inner.tcnt1.write(|w| unsafe { w.bits(0) });
    }

    fn wait(&mut self) -> nb::Result<(), void::Void> {
        let bits = self.inner.tcnt1.read().bits();
        // Timer runs at 16MHz (direct), so
        // > 1/(57600Hz) * (16MHz)
        // 2500/9, approx. 277.7777  (dimensionless)
        // (Round up)
        if bits < 256 {
            Err(nb::Error::WouldBlock)
        } else {
            self.inner.tcnt1.write(|w| unsafe { w.bits(0) });
            Ok(())
        }
    }
}

impl Periodic for Timer {}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    // let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut timer = Timer { inner: dp.TC1 };
    timer.start(());
    let mut serial = Serial::new(pins.d1.into_output(), pins.d0.into_pull_up_input(), timer);

    for b in 0x00..=0xFF {
        serial.write(b).unwrap();
        delay_ms(1);
    }
    
    loop {}
}
