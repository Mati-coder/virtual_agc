use crate::memory::*;

// This is used internally by the emulator, it is not an instruction accesible to the programmer
// Provisional
pub fn load_from(add: Address) -> Word {
    return EM[add as usize].load();
}

// Provisional
pub fn write_to(add: Address, n: Word) {
    EM[add as usize].write(n);
}

// How the AGC's ALU added
// It represents numbers in 15 bit one's complement and adds a modification to the sign bit
pub fn add_modified(a: u16, b: u16) -> u16 {
    // sum is u32 because sometimes we overflow the 16th bit
    let mut sum: u32 = (a + b) as u32; 

    // impl of one's complement 'end-around carry'
    sum += (sum >> 15) % 2;

    // impl of s2 bit (it adds a and b's sign bit to bit 16)
    sum += ((a & 0b0100000000000000) << 1) as u32;
    sum += ((b & 0b0100000000000000) << 1) as u32;

    sum as u16
}

// The accumulator uses the 16th bit of the word (not used anywhere else) to store the s2 bit
// No check for previous overflow is performed
pub fn ad(k: Address) {
    let a: u16 = CR.acc.load();
    let b: u16 = load_from(k);
    CR.acc.write(add_modified(a, b));
}

pub fn ads(k: Address) {
    let a: u16 = CR.acc.load();
    let b: u16 = load_from(k);
    let sum = add_modified(a, b);
    CR.acc.write(sum);

    // This is wrong, the sum should be saved overflow-corrected
    write_to(k, sum);
}

pub fn aug(k: ErasableAddress) {
    let n = load_from(k);
    if is_16bit(k) {
        if n >> 15 == 0 {write_to(k, add_modified(n, 1))} // +1
        else {write_to(k, add_modified(k, 0b0111111111111111))} // -1
    }
    else {
        if (n >> 15) % 2 == 0 {write_to(k, add_modified(n, 1))} // +1
        else {write_to(k, add_modified(k, 0b0111111111111111))} // -1
    }
}

pub fn bzf(k: FixedAddress) {
    let acc = CentralRegisters.acc.load();
    if acc == 0 || acc == 0xFFFF {CentralRegisters.z.write(k)} // acc +0 or -0
    // Clear extracode flag
}

pub fn bzmf(k: FixedAddress) {
    let acc = CentralRegisters.acc.load();
    if acc == 0 || (acc >> 15) % 2 {CentralRegisters.z.write(k)}
}

pub fn ca(k: Address) {
    
}