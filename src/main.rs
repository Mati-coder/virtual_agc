pub mod memory;
use memory::*;
fn main() {
    let hola: Word = Word {val:0b0000000000000001};
    print!("hola, {}", hola.read());
}