use agc_emulator as emu;

use emu::instructions::*;
use emu::memory::*;

use text_io::read;
use core::ops::Deref;

enum Command {
    ACC,
    Z,
    MEM(ErasableAddress, ErasableAddress),
    RUN(u16),
    SHOW,
    FAIL,   
    EXIT, 
}

fn get_command() -> Command {
    let s: String = read!("{}\n");
    let mut iter = s.deref().split_whitespace();

    match iter.next().unwrap() {
        "acc" => return Command::ACC,
        "z" => return Command::Z,
        "run" => return Command::RUN(iter.next().unwrap_or("1").parse().unwrap()),
        "show" => return Command::SHOW,
        "mem" => {
            let arg1 = iter.next();
            let arg2 = iter.next();

            if let Some(arg2) = arg2 {
                let min = arg1.unwrap_or("256");
                let min = min.parse().unwrap_or(256);
                let max = arg2.parse().unwrap_or(511);
                return Command::MEM(min, max);
            } else {
                let max = arg1.unwrap_or("511");
                let max = max.parse().unwrap_or(511);
                return Command::MEM(256, max);
            }
        },
        "exit" => return Command::EXIT,
        _ => return Command::FAIL,
    }
}

fn main() {
    let col = 6;
    let mut show:bool = true;
    let mut cycles_executed = 0;
    loop {
        let command = get_command();
        match command {
            Command::ACC => println!("{}", MEMORY.read(ACC)),
            Command::Z => println!("{}", MEMORY.read(Z)),
            Command::RUN(cycles) => {
                for n in 0..cycles {
                    let Instruction(ins, addr) = decode(MEMORY.read(MEMORY.read(Z)));
                    execute(MEMORY.read(MEMORY.read(Z)));
                    cycles_executed += 1;
                    if show {
                        let name = MEMORY.get_address_name(addr);
                        if name == "" {
                            if n % col == col-1 {
                                println!("|{:>3}| {:>6} {:<10} ", cycles_executed, ins, addr);
                            } else {
                                print!("|{:>3}| {:>6} {:<10} ", cycles_executed, ins, addr);
                            }

                            continue;
                        }

                        if n % col == col-1 {
                            println!("|{:>3}| {:>6} {:<10} ", cycles_executed, ins, name);
                        } else {
                            print!("|{:>3}| {:>6} {:<10} ", cycles_executed, ins, name);
                        }
                    }
                }
                if show && (cycles-1) % col != col-1 {println!()} //Only adds newline if the loop didn't end in one already
            },
            Command::SHOW => show = !show,
            Command::MEM(mut min, mut max) => {
                if min < 256 {min = 256}
                if max > 511 {max = 511}
                if max < min {max = 256}
                for addr in min..=max {
                    if (addr-min) % col == col-1 {
                        println!("{:<3}: {:<10}", addr, (MEMORY.read(addr)));
                    } else {
                        print!("{:<3}: {:<10}", addr, (MEMORY.read(addr)));
                    }
                }
                if (max-min) % col != col-1 {println!()} //Only adds newline if the loop didn't end in one already
            }
            Command::FAIL => continue,
            Command::EXIT => break,
        }
    }
}