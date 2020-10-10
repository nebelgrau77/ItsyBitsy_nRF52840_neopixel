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

use apa102_spi::Apa102;

use smart_leds_trait::RGB8;
use smart_leds::SmartLedsWrite;

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
    
    let state0: [RGB8; 1] = [RGB8 { r: 16, g: 0, b: 16 }]; //purple
    let state1: [RGB8; 1] = [RGB8 { r: 16, g: 16, b: 0 }]; //yellow
    let state2: [RGB8; 1] = [RGB8 { r: 0, g: 16, b: 16 }]; //aqua
    
    loop {
        dotstar.write(state0.iter().cloned()).unwrap();
        delay.delay_ms(1000 as u16);
        dotstar.write(state1.iter().cloned()).unwrap();
        delay.delay_ms(1000 as u16);
        dotstar.write(state2.iter().cloned()).unwrap();
        delay.delay_ms(1000 as u16);
    }
    
}