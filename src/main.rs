#![no_std]
#![no_main]

use arduino_hal::pac::TC1;
use bitbang_hal::serial::Serial;
use embedded_hal::serial::{Read, Write};
use embedded_hal::timer::{CountDown, Periodic};
use panic_halt as _;

/// A 9600Hz timer.
///
/// Since the clock ticks at 16MHz we would need to count to 1666.66.. bits. We can't, so instead
/// we count 1667, 1667, 1666, 1667, 1667, 1666, ... .
struct Timer {
    // We need the 16 bit counter.
    // TODO: Do we? Should be able to use the prescaler.
    inner: TC1,
    skip: u8,
}

impl Timer {
    pub fn new(timer: TC1) -> Self {
        Self {
            inner: timer,
            skip: 2,
        }
    }
}

impl CountDown for Timer {
    type Time = ();

    fn start<T>(&mut self, _: T) {
        self.inner.tccr1b.write(|w| w.cs1().direct());
        self.inner.tcnt1.write(|w| unsafe { w.bits(0) });
        self.skip = 2;
    }

    fn wait(&mut self) -> nb::Result<(), void::Void> {
        let mut bits = self.inner.tcnt1.read().bits();
        if self.skip == 0 {
            bits += 1;
            self.skip = 2;
        } else {
            self.skip -= 1;
        }
        if bits < 1667 {
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

    let mut timer = Timer::new(dp.TC1);
    timer.start(());
    let mut serial = Serial::new(pins.d1.into_output(), pins.d0.into_pull_up_input(), timer);

    // let mut serial =
    //     arduino_hal::Usart::new(dp.USART0, pins.d0, pins.d1.into_output(), 9600.into());

    for b in 0x00..=0xFF {
        serial.write(b).unwrap();
    }

    loop {
        let b = serial.read().unwrap();
        serial.write(b).unwrap();
    }
}
