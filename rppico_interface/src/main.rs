#![no_std]
#![no_main]

use core::panic::PanicInfo;
use embedded_hal::digital::OutputPin;
use embedded_hal::digital::StatefulOutputPin;
use embedded_hal::spi::SpiBus;
use rp_pico::entry;
use rp_pico::hal::prelude::*;
use rp_pico::hal::pac;
use rp_pico::hal;

use max7219;
use hal::fugit::RateExtU32;

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn entry() -> ! {
    let mut p = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(p.WATCHDOG);
    
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ, 
        p.XOSC, 
        p.CLOCKS, 
        p.PLL_SYS, 
        p.PLL_USB, 
        &mut p.RESETS, 
        &mut watchdog)
        .unwrap();
    
    let sio = hal::Sio::new(p.SIO);

    let pins = rp_pico::Pins::new(p.IO_BANK0, p.PADS_BANK0, sio.gpio_bank0, &mut p.RESETS);

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    
    let mut led = pins.led.into_push_pull_output();

    // let spi_pins = (pins.gpio3.into_function(), pins.gpio2.into_function());

    // let spi: hal::Spi<_, _, _, 8> = hal::Spi::new(p.SPI0, spi_pins).init(&mut p.RESETS, clocks.peripheral_clock.freq(), 8.MHz(), embedded_hal::spi::MODE_0);

    loop {}
}



