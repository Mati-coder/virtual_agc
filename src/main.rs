pub mod memory;
pub mod instructions;
#[cfg(test)]
mod tests;
use memory::*;

fn main() {
    print!("hola, {}", CR.acc.as_i16());
}