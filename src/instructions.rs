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
    // u32 is used because sometimes we overflow the 16th bit
    // we set bit 16 to zero, it was the only way I found to detect overflows of the 15th bit, needed
    // to implement the 'end-around carry'.
    let mut sum: u32 = ((a & 0x7FFF) + (b & 0x7FFF)) as u32; 
    // impl of one's complement 'end-around carry'
    sum += (sum >> 15) % 2;

    // impl of s2 bit (it adds a and b's sign bit to bit 16)
    sum += ((a & 0b0100000000000000) << 1) as u32;
    sum += ((b & 0b0100000000000000) << 1) as u32;

    sum as u16
}

pub fn correct(n: u16) -> u16 {
    let sign_bits = n >> 15; // bits 16 and 15
    match sign_bits {
        0b00 | 0b11 => {
            return n
        }
        // overflow
        0b10 | 0b01 => {
            return n ^ 0b0100000000000000 // flip sign bit
        }
        _ => {panic!();} // Should never happen
    }
}

pub fn as_i32(n: u16) -> i32 {
    let v = ((n << 1) as i32) >> 1; // Ignores the 16th bit
    if v < 0 {return v + 1;} // Because v is in one's complement
    else {return v;}
}

pub fn ones_complement32(n: i32) -> u32 {
    if n < 0 { return !(n.abs()) as u32}
    return n as u32
}

pub fn ones_complement16(n: i16) -> u16 {
        if n < 0 { return !(n.abs()) as u16}
        return n as u16
    }

pub fn sign_bit(n: u16) -> u16 {
    return (n >> 15) % 2 // returns bit 15
}

// Saves a value overflow-corrected, except when saving to A or Q
// Returns +0 for no overflow, 1 for +overflow and -1(one's complement) for -overflow
pub fn save_corrected(n: u16, k: ErasableAddress) -> Word {
    let sign_bits = n >> 15; // bits 16 and 15
    match sign_bits {
        0b00 | 0b11 => {
            write_to(k, n);
            return 0; // +0
        }
        // Negative overflow
        0b10 => {
            if is_16bit(k) {
                write_to(k, n);
            } else {
                write_to(k, n ^ 0b0100000000000000); // flip sign bit
            }
            return NEG_ONE; // -1
        }
        // Positive overflow
        0b01 => {
            if is_16bit(k) {
                write_to(k, n);
            } else {
                write_to(k, n ^ 0b0100000000000000); // flip sign bit
            }
            return 0x0001; // 1
        }
        _ => {panic!();}
    }
}

// Add
pub fn ad(k: Address) {
    let a: u16 = CR.acc.load();
    let b: u16 = load_from(k);
    CR.acc.write(add_modified(a, b));
}

// Add to storage
pub fn ads(k: ErasableAddress) {
    let a: u16 = CR.acc.load();
    let b: u16 = load_from(k);
    let sum = add_modified(a, b);
    CR.acc.write(sum);

    // This is wrong, the sum should be saved overflow-corrected
    write_to(k, sum);
}

// Augment
pub fn aug(k: ErasableAddress) {
    let n = load_from(k);
    if is_16bit(k) {
        if n >> 15 == 0 {write_to(k, add_modified(n, 1))} // +1
        else {write_to(k, add_modified(k, NEG_ONE))} // -1
    }
    else {
        if (n >> 14) % 2 == 0 {write_to(k, add_modified(n, 1))} // +1
        else {write_to(k, add_modified(k, NEG_ONE))} // -1
    }
}

// Branch zero to fixed
pub fn bzf(k: FixedAddress) {
    let acc = CR.acc.load();
    if acc == 0 || acc == NEG_ZERO {CR.z.write(k)} // acc +0 or -0
}

// Branch zero or minus to fixed
pub fn bzmf(k: FixedAddress) {
    let acc = CR.acc.load();
    if acc == 0 || sign_bit(acc) == 1 {CR.z.write(k)}
}

// Clear and Add
pub fn ca(k: Address) {
    let n = load_from(k);
    CR.acc.write(n);
}

// Clear and Substract
pub fn cs(k: Address) {
    let n = load_from(k);
    if is_16bit(k) {
        CR.acc.write(!n);
    } else {
        if sign_bit(n) == 0 {CR.acc.write(!n | 0x8000)} // !n is negative, bit 16 set to 1
        else {CR.acc.write(!n & 0x7FFF)} // !n is positive, bit 16 set to 0
    }
}

// Double Add to Storage
pub fn das(k: ErasableAddress) {
    let a = CR.acc.load();
    let a_low = CR.l.load();
    let b = load_from(k);
    let b_low = load_from(k + 1);

    // Add and save lower word, save overflow
    let sum_low = add_modified(a_low, b_low);
    let overflow_low = save_corrected(sum_low, k + 1);

    // Add higher words and previous overflow, save sum
    let mut sum = add_modified(a, b); 
    sum = add_modified(sum, overflow_low);
    let overflow = save_corrected(sum, k);

    // As defined in documentation
    CR.acc.write(overflow);
    CR.l.write(0);
} 

// Double Clear and Add
pub fn dca(k: Address) {
    // The AGC processed the instruction in the following order
    let low = load_from(k + 1);
    CR.l.write(low);
    let high = load_from(k);
    CR.acc.write(high);
}

// Double Clear and Substract
pub fn dcs(k: Address) {
    // The AGC processed the instruction in the following order
    let low = load_from(k + 1);
    CR.l.write(!low);
    let high = load_from(k);
    CR.acc.write(!high);
}

// Diminish
pub fn dim(k: ErasableAddress) {
    let n = load_from(k);
    if is_16bit(k) {
        if n >> 15 == 0 {write_to(k, add_modified(n, NEG_ONE))} // +1
        else {write_to(k, add_modified(k, 1))} // -1
    }
    else {
        if (n >> 14) % 2 == 0 {write_to(k, add_modified(n, NEG_ONE))} // +1
        else {write_to(k, add_modified(k, 1))} // -1
    }
}

// Double Exchange
pub fn dxch(k: ErasableAddress) {
    let high = load_from(k);
    let low = load_from(k + 1);

    CR.acc.write(high);
    CR.l.write(low);
}

// Increment
pub fn incr(k: ErasableAddress) {
    let n = load_from(k);
    write_to(k, add_modified(n, 1));
}

// Exchange L and K
pub fn lxch(k: ErasableAddress) {
    let n = load_from(k);
    write_to(k, CR.l.load());
    CR.l.write(n);
}

// AND A and k
pub fn mask(k: Address) {
    let mut acc = CR.acc.load();
    if is_16bit(k) {
        CR.acc.write(acc & load_from(k));
    } else {
        acc = correct(acc); // correct overflow
        acc = ( (acc << 1) & (load_from(k) << 1) ) >> 1; // AND components (ignoring bit 16)
        acc += (acc & 0b0100000000000000) << 1; // copy bit 15 into bit 16
        CR.acc.write(acc);
    }
}

// Multiply
pub fn mp(k: Address) {
    // Perform the signed multiplication in two's complement
    let acc = correct(CR.acc.load());
    let n = load_from(k);
    let product: i32 = as_i32(acc) * as_i32(n);

    // Handling as defined in documentation
    if product == 0 {
        // acc != +/- 0 or signs are equal
        if ( acc != 0 && acc != NEG_ZERO ) || sign_bit(acc) == sign_bit(n) { 
            CR.acc.write(0);
            CR.l.write(0);
        }
        else {
            CR.acc.write(NEG_ZERO);
            CR.l.write(NEG_ZERO);
        }
        return ;
    }

    
    let product = ones_complement32(product);
    let sign_bit = (product >> 31) as u16;
    // SP values are scaled by a factor of 2^-14, that's why the results "start" in the L register
    let low= (product % (2 << 13)) as u16; // Takes the lower 14 bits
    let high = ((product >> 14) % (2 << 13)) as u16; // Takes bits 28 through 15

    CR.acc.write(high + sign_bit);
    CR.l.write(low + sign_bit);
}

// Modular Substract
// In this instruction the accumulator and the value at k should contain
// a two's complement value
pub fn msu(k: ErasableAddress) {
    let n = load_from(k);
    let acc = CR.acc.load();
    if is_16bit(k) {
        let diff = ones_complement16(acc as i16 - n as i16);
        CR.acc.write(diff as u16);
    } else {
        let mut diff = (acc << 1) as i16 - (n << 1) as i16; // ignore bit 16
        diff >>= 1;
        let mut diff = ones_complement16(diff);
        diff += (diff & 0b0100000000000000) << 1; // copy bit 15 into bit 16
        CR.acc.write(diff);
    }
}