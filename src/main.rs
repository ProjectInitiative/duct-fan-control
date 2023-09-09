//! This example shows how to use PWM (Pulse Width Modulation) in the RP2040 chip.
//!
//! The LED on the RP Pico W board is connected differently. Add a LED and resistor to another pin.

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::pwm::{Config, Pwm};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut c: Config = Default::default();
    // c.top = 0x1388;
    c.top = 5000;
    c.compare_a = c.top / 2;
    c.compare_b = c.top / 2;

    let mut pwm = Pwm::new_output_ab(p.PWM_CH4, p.PIN_8, p.PIN_9, c.clone());
    pwm.set_config(&c);

    loop {}
}
