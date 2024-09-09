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
                let min = arg1.unwrap_or("48");
                let min = min.parse().unwrap_or(48);
                let max = arg2.parse().unwrap_or(255);
                return Command::MEM(min, max);
            } else {
                let max = arg1.unwrap_or("255");
                let max = max.parse().unwrap_or(255);
                return Command::MEM(48, max);
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
                    let Instruction(ins, addr) = execute(MEMORY.read(MEMORY.read(Z)));
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
                        // if name.ends_with('+') {
                        //     if n % col == col-1 {
                        //         println!("{:>6} {:>10}{:>2}", ins, name, MEMORY.table_offset(name, addr));
                        //     } else {
                        //         print!("{:>6} {:>2}{:>2} || ", ins, name, MEMORY.table_offset(name, addr));
                        //     }

                        //     continue;
                        // }

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
                if min < 48 {min = 48}
                if max > 256 {max = 255}
                if max < min {max = 49}
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