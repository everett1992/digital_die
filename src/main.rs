#![no_std]
#![no_main]

extern crate feather_m0 as hal;
extern crate panic_halt;
extern crate rand_hc;
extern crate rand;

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting;
extern crate embedded_hal;

use core::fmt::Write;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::time::MegaHertz; use hal::{entry, CorePeripherals, Peripherals}; 
use ssd1306::prelude::*;
use ssd1306::Builder;
use rand::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let spi = hal::spi_master(
        &mut clocks,
        MegaHertz(20),
        peripherals.SERCOM4,
        &mut peripherals.PM,
        pins.sck,
        pins.mosi,
        pins.miso,
        &mut pins.port,
    );
    let dc = pins.a0.into_push_pull_output(&mut pins.port);
    let mut rst = pins.a2.into_open_drain_output(&mut pins.port);

    let mut pin_d13 = pins.d13.into_pull_up_input(&mut pins.port);
    let mut pin_d12 = pins.d12.into_pull_up_input(&mut pins.port);
    let mut pin_d10 = pins.d10.into_pull_up_input(&mut pins.port);
    let mut pin_scl = pins.scl.into_pull_up_input(&mut pins.port);
    let mut pin_d1 = pins.d1.into_pull_up_input(&mut pins.port);
    let mut pin_d9 = pins.d9.into_pull_up_input(&mut pins.port);
    let mut pin_sda = pins.sda.into_pull_up_input(&mut pins.port);
    let mut pin_d11 = pins.d11.into_pull_up_input(&mut pins.port);
    let mut pin_d6 = pins.d6.into_pull_up_input(&mut pins.port);
    let mut pin_a5 = pins.a5.into_pull_up_input(&mut pins.port);
    let mut pin_a3 = pins.a3.into_pull_up_input(&mut pins.port);
    let mut pin_a4 = pins.a4.into_pull_up_input(&mut pins.port);
    let mut pin_d0 = pins.d0.into_pull_up_input(&mut pins.port);
    let mut pin_d5 = pins.d5.into_pull_up_input(&mut pins.port);

    let mut num_keys = [
        NumKey::new(0, &mut pin_d13),
        NumKey::new(1, &mut pin_d12),
        NumKey::new(2, &mut pin_d10),
        NumKey::new(3, &mut pin_scl),
        NumKey::new(4, &mut pin_d1),
        NumKey::new(5, &mut pin_d9),
        NumKey::new(6, &mut pin_sda),
        NumKey::new(7, &mut pin_d11),
        NumKey::new(8, &mut pin_d6),
        NumKey::new(9, &mut pin_a5),
    ];

    let mut die = Key::new(&mut pin_a3);
    let mut plus = Key::new(&mut pin_a4);
    let mut equals = Key::new(&mut pin_d0);
    let mut clear = Key::new(&mut pin_d5); // tab

    let mut disp: TerminalMode<_> = Builder::new()
        .with_size(DisplaySize::Quirk128x32)
        .connect_spi(spi, dc)
        .into();

    disp.reset(&mut rst, &mut delay);
    disp.init().unwrap();
    disp.clear().unwrap();

    let mut rng = rand_hc::Hc128Rng::seed_from_u64(0);
    let mut num: u64 = 0;
    let mut sum: u64 = 0;
    let mut quant: Option<u64> = None;
    let mut do_clear = false;

    disp.write_char(' ').unwrap();
    loop {
        for key in num_keys.iter_mut() {
            if key.is_pressed() {
                if do_clear {
                    do_clear = false;
                    disp.clear().unwrap();
                    disp.write_char(' ').unwrap();
                }
                // write the character and push into the number 'buffer'
                // dec 48 is '0' ascii.
                disp.write_char((key.num+48) as char).unwrap();
                num = num * 10 + (key.num as u64);
            }
        }

        if die.is_pressed() {
            if do_clear {
                do_clear = false;
                disp.clear().unwrap();
                disp.write_char(' ').unwrap();
            }
            disp.write_char('d').unwrap();

            // d10 is the same as 1d10, but '10' is not.
            quant = Some(match num {
                0 => 1,
                _ => num,
            });
            num = 0;
        }

        if plus.is_pressed() {
            if do_clear {
                do_clear = false;
                disp.clear().unwrap();
                disp.write_char(' ').unwrap();
            }
            if let Some(x) = quant {
                for _ in 0..x {
                    sum += rng.gen_range(1, num + 1);
                }
            } else {
                sum += num;
            }
            num = 0;
            quant = None;
            disp.write_char('+').unwrap();
        }

        if equals.is_pressed() {
            if let Some(x) = quant {
                for _ in 0..x {
                    sum += rng.gen_range(1, num + 1);
                }
            } else {
                sum += num;
            }

            disp.clear().unwrap();
            disp.write_fmt(format_args!(" {}", sum)).unwrap();
            sum = 0;
            num = 0;
            quant = None;
            do_clear = true;
        }

        if clear.is_pressed() {
            sum = 0;
            num = 0;
            quant = None;
            do_clear = false;
            disp.clear().unwrap();
            disp.write_char(' ').unwrap();
        }

        delay.delay_ms(10u8);
    }
}

use embedded_hal::digital::InputPin;
struct Key<'a> {
    pub pin: &'a mut InputPin,
    was_pressed: bool,
}

struct NumKey<'a> {
    pub key: Key<'a>,
    pub num: u8,
}

impl <'a> Key<'a> {
    fn new(pin: &'a mut InputPin) -> Key<'a> {
        Key { pin, was_pressed: false }
    }

    pub fn is_pressed(&mut self) -> bool {
        let is_pressed = self.pin.is_low();
        if is_pressed && ! self.was_pressed {
            self.was_pressed = true;
            true
        } else if ! is_pressed && self.was_pressed {
            self.was_pressed = false;
            false
        } else {
            false
        }
    }
}

impl <'a> NumKey<'a> {
    fn new(num: u8, pin: &'a mut InputPin) -> NumKey<'a> {
        NumKey { num, key: Key::new(pin) }
    }

    pub fn is_pressed(&mut self) -> bool {
        self.key.is_pressed()
    }

}
