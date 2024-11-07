#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::slice::from_raw_parts;
use agc_emulator::instructions::Instruction;
use cortex_m::asm::delay;
use cortex_m::register::control::read;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_hal::digital::InputPin;
use embedded_hal::digital::StatefulOutputPin;
use embedded_hal::spi::SpiBus;
use embedded_hal_0_2::adc::OneShot;
use rp2040_hal::gpio::bank0::Gpio16;
use rp2040_hal::gpio::bank0::Gpio6;
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

// Modes
enum Modes {
    MANUAL = 0,
    AUTO = 1,
    CONTINUO = 2,
    RESET = 3,
    MEM = 4,
}
impl From<u8> for Modes {
    fn from(value: u8) -> Self {
        match value % 5 {
            0 => Modes::MANUAL,
            1 => Modes::AUTO,
            2 => Modes::CONTINUO,
            3 => Modes::RESET,
            4 => Modes::MEM,
            _ => unreachable!()
        }
    }
}

fn char(value: u16) -> &'static str {
    match value % 10{
        0 => "0",
        1 => "1",
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        8 => "8",
        9 => "9",
        _ => unreachable!()
    }
}

// Addresses for led matrix control
const DECODE: u16 = 0x900;
const INTENSITY: u16 = 0xa00;
const SCAN_LIMIT: u16 = 0xb00;
const SHUTDOWN: u16 = 0xc00;
const TEST: u16 = 0xf00;

// Addresses of peripherals
macro_rules! register {
    ($name:ident, $value:literal) => {
        const $name: ErasableAddress = $value;
    };
}
register!(ACC, 0);
register!(PANT, 256);
register!(BTNUP, 264);
register!(BTNRGT, 265);
register!(BTNDWN, 266);
register!(BTNLFT, 267);
register!(BTN1, 268);
register!(BTN2, 269);
register!(POTE, 270);
register!(CORTO, 271);
register!(MEDIO, 272);
register!(LARGO, 273);

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
    button!(btnup, gpio9); 
    button!(btnrgt, gpio8); 
    button!(btndwn, gpio12);
    button!(btnlft, gpio10);
    button!(btn1, gpio14); 
    button!(btn2, gpio6);
    button!(btncfg, gpio13);
    button!(btnclk, gpio7);
    
    // LED Matrix control variables and setup
    const SCREEN_MASK: u16 = 0x00FF;
    let spi_pins = (pins.gpio3.into_function(), pins.gpio2.into_function());
    let mut spi: hal::Spi<_, _, _, 16> = hal::Spi::new(p.SPI0, spi_pins).init(&mut p.RESETS, clocks.peripheral_clock.freq(), 8.MHz(), embedded_hal::spi::MODE_0);
    let mut cs = pins.gpio5.into_push_pull_output();
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
    sendto_matrix!(INTENSITY + 1); // ABOUT HALF INTENSITY

    for i in 1..=8 {
        sendto_matrix!(i * 16 * 16 + 0);
    } 

    // LCD control variables and setup
    let mut i2c = hal::I2C::i2c0(
        p.I2C0, 
        pins.gpio0.reconfigure(), 
        pins.gpio1.reconfigure(), 
        100.kHz(),
        &mut p.RESETS,
        &clocks.system_clock,
    );
    const LCD_ADDRESS: u8 = 0x27;
    let mut lcd_timer = timer.clone();
    let mut lcd = lcd_lcm1602_i2c::sync_lcd::Lcd::new(&mut i2c, &mut lcd_timer)
        .with_address(LCD_ADDRESS)
        .with_cursor_on(false) // no visible cursor
        .with_rows(2) // two rows
        .init().unwrap();
    
    lcd.set_cursor(0, 0);
  
    // Potentiometer control variables
    let mut adc = hal::Adc::new(p.ADC, &mut p.RESETS);
    let mut potentiometer = hal::adc::AdcPin::new(pins.gpio26).unwrap();


    let mut mode: Modes = Modes::MANUAL;
    let mut pulsedclk: bool = false;
    let mut pulsedcfg: bool = false;
    let mut imp: bool = true;
    let mut executing: bool = false;
    let mut address = 256;
    let mut pulsedup: bool = false;
    let mut pulsedown: bool = false;
    loop {
        macro_rules! update_btn {
            ($name:ident, $addr:expr) => {
                if $name.is_high().unwrap() {
                    MEMORY.write($addr, 1);
                } else {
                    MEMORY.write($addr, 0);
                }
            };
        }
        update_btn!(btnup, BTNUP);
        update_btn!(btndwn, BTNDWN);
        update_btn!(btnlft, BTNLFT);
        update_btn!(btnrgt, BTNRGT);
        update_btn!(btn1, BTN1);
        update_btn!(btn2, BTN2);
        let reading: u16 = adc.read(&mut potentiometer).unwrap();
        let reading = reading & 4095;
        MEMORY.write(POTE, reading);
        
        macro_rules! print_lcd {
            ($mode: literal) => {
                let Instruction(name, addr) = decode(MEMORY.read(MEMORY.read(Z)));
                let addr_name = MEMORY.get_address_name(addr.unwrap_or(513));
                lcd.clear();
                lcd.write_str($mode);
                lcd.write_str(" ");
                lcd.write_str(name);
                lcd.write_str(" "); 
                lcd.write_str(addr_name);
            };
        }
        macro_rules! print_val_at {
            ($addr: expr) => {
                let mut val = MEMORY.read($addr);
                if val >> 14 != 0 {
                    val = !val;
                    lcd.write_str("-");
                } else {
                    lcd.write_str("+");
                }
                lcd.write_str(char(val/10000));
                lcd.write_str(char(val/1000));
                lcd.write_str(char(val/100));
                lcd.write_str(char(val/10));
                lcd.write_str(char(val));
            }
        }
        for i in 0..8 {   
           sendto_matrix!(16*16*(8-i) + (MEMORY.read(PANT+i) & SCREEN_MASK));
        }
        
        if btncfg.is_high().unwrap() && !pulsedcfg {
            mode = (mode as u8 + 1).into();
            timer.delay_ms(100);
            imp = true;
            pulsedcfg = true;
        }
        if btncfg.is_low().unwrap() {
            pulsedcfg = false;
        }

        match mode {
            Modes::MANUAL => {
                if imp {
                    MEMORY.write(CORTO, 0);
                    MEMORY.write(MEDIO, 0);
                    MEMORY.write(LARGO, 3);
                    print_lcd!("M");
                    lcd.set_cursor(1, 0);
                    lcd.write_str("ACC: ");
                    print_val_at!(ACC);
                    imp = false;
                }
                if btnclk.is_high().unwrap() && !pulsedclk {
                    execute(MEMORY.read(MEMORY.read(Z)));
                    lcd.set_cursor(0, 0);
                    btnclk.is_high().unwrap();
                    print_lcd!("M");
                    lcd.set_cursor(1, 0);
                    lcd.write_str("ACC: ");
                    print_val_at!(ACC);
                    pulsedclk = true;
                    timer.delay_ms(100);
                }
                if btnclk.is_low().unwrap() {
                    pulsedclk = false;
                }
            },
            Modes::AUTO => {
                if imp {
                    MEMORY.write(CORTO, 0);
                    MEMORY.write(MEDIO, 0);
                    MEMORY.write(LARGO, 3);
                    print_lcd!("A");
                    lcd.set_cursor(1, 0);
                    lcd.write_str("ACC: ");
                    print_val_at!(ACC);
                    imp = false;
                }
                if btnclk.is_high().unwrap(){
                    execute(MEMORY.read(MEMORY.read(Z)));
                    lcd.set_cursor(0, 0);
                    btnclk.is_high().unwrap();
                    print_lcd!("A");
                    lcd.set_cursor(1, 0);
                    lcd.write_str("ACC: ");
                    print_val_at!(ACC);
                    timer.delay_ms(300);
                }
            },
            Modes::CONTINUO => {
                if imp {
                    MEMORY.write(CORTO, 50);
                    MEMORY.write(MEDIO, 125);
                    MEMORY.write(LARGO, 200);
                    lcd.clear();
                    lcd.write_str("    CONTINUO    ");
                    lcd.set_cursor(1, 0);
                    lcd.write_str("    PAUSADO     ");
                    executing = false;
                    imp = false;
                }
                if btnclk.is_high().unwrap() && !pulsedclk{
                    executing = !executing;
                    lcd.set_cursor(1, 0);
                    if executing {
                        lcd.write_str("    EJECUTANDO  ");
                    } else {
                        lcd.write_str("    PAUSADO     ");
                    }
                    
                    pulsedclk = true;
                }
                if btnclk.is_low().unwrap() {
                    pulsedclk = false;
                }
                if executing {
                    execute(MEMORY.read(MEMORY.read(Z)));
                }
                
            },
            Modes::RESET => {
                if imp {
                    lcd.clear();
                    lcd.write_str("RESET");
                    imp = false;
                }
                if btnclk.is_high().unwrap() && !pulsedclk{
                    hal::reset();
                }
                if btnclk.is_low().unwrap() {
                    pulsedclk = false;
                }
            },
            Modes::MEM => {
                if btndwn.is_high().unwrap() && !pulsedown {
                    if  address < 287{
                        address = address + 1;
                    }
                    pulsedown = true;
                }
                if btnup.is_high().unwrap() && !pulsedup{
                    if address > 256{
                       address = address - 1;
                    }
                    pulsedup = true;
                }
                if btndwn.is_low().unwrap() {
                    pulsedown = false;
                }
                if btnup.is_low().unwrap() {
                    pulsedup = false;
                }
                lcd.clear();
                lcd.write_str(MEMORY.get_address_name(address));
                lcd.write_str(": ");
                print_val_at!(address);
                lcd.set_cursor(1, 0);
                lcd.write_str(MEMORY.get_address_name(address + 1));
                lcd.write_str(": ");
                print_val_at!(address + 1);
            }
        }
    }
}

