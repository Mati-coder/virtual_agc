#![no_std]
#![no_main]

use core::panic::PanicInfo;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_hal::digital::InputPin;
use embedded_hal::digital::StatefulOutputPin;
use embedded_hal::spi::SpiBus;
use embedded_hal_0_2::adc::OneShot;
use rp_pico::entry;
use rp_pico::hal::prelude::*;
use rp_pico::hal::pac;
use rp_pico::hal;
use hal::fugit::RateExtU32;
use lcd_lcm1602_i2c;
use agc_emulator::memory::*;
use agc_emulator::instructions::decode;
use agc_emulator::instructions::execute;

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

// Addresses for led matrix control
const DECODE: u16 = 0x900;
const INTENSITY: u16 = 0xa00;
const SCAN_LIMIT: u16 = 0xb00;
const SHUTDOWN: u16 = 0xc00;
const TEST: u16 = 0xf00;

#[entry]
fn entry() -> ! {
    let mut p = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // BASIC CONFIGURATION
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
    let mut timer = hal::Timer::new(p.TIMER, &mut p.RESETS, &clocks);

    // Internal LED
    let mut led = pins.led.into_push_pull_output();

    // Buttons
    macro_rules! button {
        ($name: ident, $pin: ident) => {
            let mut $name = pins.$pin.into_pull_down_input();
        };
    }
    button!(btnup, gpio8);
    button!(btnrgt, gpio9);
    button!(btndwn, gpio10);
    button!(btnlft, gpio11);
    button!(btn1, gpio13);
    button!(btn2, gpio14);
    button!(btncfg, gpio15);
    button!(btnclk, gpio16);
    
    // LED Matrix control variables and setup
    let spi_pins = (pins.gpio3.into_function(), pins.gpio2.into_function());
    let mut spi: hal::Spi<_, _, _, 16> = hal::Spi::new(p.SPI0, spi_pins).init(&mut p.RESETS, clocks.peripheral_clock.freq(), 8.MHz(), embedded_hal::spi::MODE_0);
    let mut cs = pins.gpio7.into_push_pull_output();
    macro_rules! sendto_matrix {
        ($data: expr) => {
            cs.set_high();
            timer.delay_ns(25);
            spi.write(&[$data]);
            timer.delay_ns(5);
            cs.set_low();
            timer.delay_ns(10);
        };
    }
    sendto_matrix!(SHUTDOWN + 1); // ON
    sendto_matrix!(TEST + 0); // NO TEST
    sendto_matrix!(SCAN_LIMIT + 7); // ALL DIGITS
    sendto_matrix!(DECODE + 0); // NO DECODE
    sendto_matrix!(INTENSITY + 7); // ABOUT HALF INTENSITY

    // LCD control variables and setup
    let mut i2c = hal::I2C::i2c0(
        p.I2C0, 
        pins.gpio0.reconfigure(), 
        pins.gpio1.reconfigure(), 
        100.kHz(),
        &mut p.RESETS,
        &clocks.system_clock,
    );
    const LCD_ADDRESS: u8 = 0x3F;
    let mut lcd = lcd_lcm1602_i2c::sync_lcd::Lcd::new(&mut i2c, &mut timer)
        .with_address(LCD_ADDRESS)
        .with_cursor_on(false) // no visible cursor
        .with_rows(2) // two rows
        .init().unwrap();

    lcd.set_cursor(1, 2);

    // Potentiometer control variables
    let mut adc = hal::Adc::new(p.ADC, &mut p.RESETS);
    let mut potentiometer = hal::adc::AdcPin::new(pins.gpio26).unwrap();
    
    // Ejemplo lectura del pote
    let reading: u16 = adc.read(&mut potentiometer).unwrap();

    loop {
        
    }
}



