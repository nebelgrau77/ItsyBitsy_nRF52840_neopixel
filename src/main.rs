#![no_main]
#![no_std]

use panic_halt as _;

use nrf52840_hal as hal;

use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::gpio::Level;
use hal::delay::Delay;

use hal::spim::Spim;

use cortex_m_rt::entry;

use apa102_spi as apa102;
use apa102::Apa102;
use smart_leds_trait::RGB8;
use smart_leds::{
    hsv::{hsv2rgb, Hsv},
    SmartLedsWrite,
    };

use hal::adc::{Adc, AdcConfig};

use rand::prelude::*;

use micromath::F32Ext;
use core::f32::consts::FRAC_PI_2;

#[entry]
fn main() -> ! {
    
    let p = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let port1 = hal::gpio::p1::Parts::new(p.P1);
    
    let di = port0.p0_08.into_push_pull_output(Level::Low).degrade(); // data line
    let ci = port1.p1_09.into_push_pull_output(Level::Low).degrade(); // clock
    let nc = port1.p1_14.into_floating_input().degrade(); // not connected

    let pins = hal::spim::Pins{
                sck: ci, 
                miso: Some(nc), 
                mosi: Some(di)};

    let spi = Spim::new(
        p.SPIM2,
        pins,
        hal::spim::Frequency::K500,
        hal::spim::MODE_0,
        0,
    );

    let mut delay = Delay::new(core.SYST);

    let mut dotstar = Apa102::new(spi);
    
    let seed = 0x1337_d00d_8d34_dee7; //trying to spell "leet dood ain't dead yet" LOL

    let mut rng = SmallRng::seed_from_u64(seed);

    
    loop {
        let rand = rng.next_u64() as u8;

        //slowly enable led
        for j in 0..255u8 {
            
            let color: [RGB8; 1] = [hsv2rgb(Hsv {
                                    hue: rand,
                                    sat: 255,
                                    val: sine_ease_in(j as f32, 0.0, 32.0, 255.0) as u8,
                                    })];

            dotstar.write(color.iter().cloned()).unwrap();
            
            delay.delay_ms(5u8);
        }

        //slowly enable led
        for j in (0..255u8).rev() {
            
            let color: [RGB8; 1] = [hsv2rgb(Hsv {
                                    hue: rand,
                                    sat: 255,
                                    val: sine_ease_in(j as f32, 0.0, 32.0, 255.0) as u8,
                                    })];

            dotstar.write(color.iter().cloned()).unwrap();
            
            delay.delay_ms(5u8);
        }
    }
    
}


#[inline]
// current step, where oputput starts, where output ends, last step
fn sine_ease_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
    -c * (t / d * FRAC_PI_2).cos() + c + b
}