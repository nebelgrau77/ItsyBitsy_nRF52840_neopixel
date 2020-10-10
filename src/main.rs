#![no_main]
#![no_std]

use panic_halt as _;

use nrf52840_hal as hal;

use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::gpio::Level;
use hal::delay::Delay;
use hal::spim::Spim;
use hal::rng;

use cortex_m_rt::entry;

use apa102_spi::Apa102;
use smart_leds_trait::RGB8;
use smart_leds::{
    hsv::{hsv2rgb, Hsv},
    SmartLedsWrite,
    };

use micromath::F32Ext;
use core::f32::consts::FRAC_PI_2; // half Pi constant value

#[entry]
fn main() -> ! {
    
    let p = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    // set up GPIO ports
    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let port1 = hal::gpio::p1::Parts::new(p.P1);
    
    // set up pins for the NeoPixel 
    let di = port0.p0_08.into_push_pull_output(Level::Low).degrade(); // data line
    let ci = port1.p1_09.into_push_pull_output(Level::Low).degrade(); // clock
    let nc = port1.p1_14.into_floating_input().degrade(); // not connected

    // use these pins for the SPI bus
    let pins = hal::spim::Pins{
                sck: ci, 
                miso: Some(nc), 
                mosi: Some(di)};

    // set up SPI                
    let spi = Spim::new(
        p.SPIM2,
        pins,
        hal::spim::Frequency::K500,
        hal::spim::MODE_0,
        0,
    );

    // initialize a delay provider
    let mut delay = Delay::new(core.SYST);

    // set up the NeoPixel
    let mut dotstar = Apa102::new(spi);
    
    // initialize Random Numbers Generator
    let mut rng = rng::Rng::new(p.RNG);
    
    loop {
        
        // get a random value between 0 and 255
        let rand = rng.random_u8();
        
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

        //slowly disable led
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