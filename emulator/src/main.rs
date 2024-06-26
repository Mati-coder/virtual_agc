#[allow(non_snake_case)]
pub mod Memory;
pub mod instructions;
#[cfg(test)]
mod tests;
use crate::instructions::*;
use crate::Memory::*;

fn main() {
    // Execute 100 clock cycles
    let col = 6;
    for i in 0..= 150 {
        execute(MEMORY.read(MEMORY.read(Z)));
        if i % col == col-1 {
            println!("{}: {:<10}", i, (MEMORY.read(ACC) % 32768));
        } else {
            print!("{}: {:<10}", i, (MEMORY.read(ACC) % 32768));
        }
    }
}