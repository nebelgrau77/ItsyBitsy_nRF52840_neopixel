#![no_main]
#![no_std]

use panic_halt as _;

use nrf52840_hal as hal;

use hal::{pac::{            
            Peripherals},
        prelude::*,
        gpio::Level,        
        spim::Spim,
        rtc::{Rtc, RtcCompareReg, RtcInterrupt},        
        };

use cortex_m_rt::entry;

use apa102_spi::Apa102;

use smart_leds_trait::RGB8;
use smart_leds::{
    hsv::{hsv2rgb, Hsv},
    SmartLedsWrite,
    };

const PXL_SATURATION: u8 = 255;
const PXL_VALUE: u8 = 12; // how bright the NeoPixel is
const PXL_HUE_STEP: u8 = 32; //the smaller the step, the more hues
const PXL_HUE_PHASE: u8 = 16; //shifted by this amount from the inital value

#[entry]
fn main() -> ! {
    
    let p = Peripherals::take().unwrap();
    //let core = CorePeripherals::take().unwrap();

    let clocks = hal::clocks::Clocks::new(p.CLOCK);
    let _clocks = clocks.start_lfclk();

    // set up Real Time Clock, interrupt will fire after one second
    let mut rtc = Rtc::new(p.RTC0,0).unwrap();
    rtc.set_compare(RtcCompareReg::Compare0, 32_768).unwrap();
    rtc.enable_event(RtcInterrupt::Compare0);

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

    
    // set up the NeoPixel
    let mut dotstar = Apa102::new(spi);
        
    rtc.enable_counter(); // start the RTC

    let mut pxl_hue: u8 = PXL_HUE_PHASE; //starting hue

    loop {

        while !rtc.is_event_triggered(RtcInterrupt::Compare0) {}
        rtc.reset_event(RtcInterrupt::Compare0); // reset interrupt
        rtc.clear_counter(); // zero the RTC counter again

        let color: [RGB8; 1] = [hsv2rgb(Hsv {
            hue: pxl_hue,
            sat: PXL_SATURATION,
            val: PXL_VALUE,
            })];

        dotstar.write(color.iter().cloned()).unwrap();
        
        pxl_hue = pxl_hue.wrapping_add(PXL_HUE_STEP);

    }
    
}