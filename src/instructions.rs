use memory::*;

// The accumulator uses the 16th bit of the word (not used anywhere else) to store the s2 bit
// No check for previous overflow is performed
pub fn add(Word a, Word b) {
    let s2: bool = (a >> 14) + (b >> 14);
    let mut sum = a + b;
    // impl of one's complement 'end-around carry'
    sum += sum >> 15;

    // Set bit 16 to s2
    if s2 {sum &= 0b1000000000000000;}
    else {sum &= 0b0111111111111111;}
}

pub fn add
