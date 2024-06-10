pub mod memory;
pub mod instructions;
#[cfg(test)]
mod tests;
use crate::instructions::*;
use crate::memory::*;

/* 
instructions todo:
- ad done 
- ads done 
- bzf (ex) done 
- bzmf (ex) done 
- ca done 
- cs done 
- das done 
- dca (ex) done 
- dcs (ex) done 
- extend done
- incr done
- mask done
- su (ex) done
- ts done
- xch done
*/

fn main() {
    execute(0b0010100000000000 + 120);
    execute(0b0011000000000000 + 120);
    println!("{}", MEMORY.read(ACC)); // Should print 1
}