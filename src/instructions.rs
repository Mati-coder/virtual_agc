use crate::memory::*;

// How the AGC's ALU added
// It represents numbers in 15 bit one's complement and adds a modification to the sign bit
// For non-overflow conditions this function returns a value who's bit 15 and 16 are equal
pub fn add_modified(a: u16, b: u16) -> u16 {
    // u32 is used because sometimes we overflow the 16th bit
    // we set bit 16 to zero, it was the only way I found to detect overflows of the 15th bit, needed
    // to implement the 'end-around carry'.
    let mut sum: u32 = ((a & 0x7FFF) + (b & 0x7FFF)) as u32; 
    // impl of one's complement 'end-around carry'
    sum += (sum >> 15) % 2;

    // addition of bit 16
    sum += (a & 0x8000) as u32;
    sum += (b & 0x8000) as u32;

    sum as u16
}

// Corrects the overflow of a value (flips bit 15) if it has one, else returns the value
pub fn correct(n: u16) -> u16 {
    let sign_bits = n >> 15; // bits 16 and 15
    match sign_bits {
        0b00 | 0b11 => return n,
        // overflow
        0b10 | 0b01 => return n ^ 0b0100000000000000, // flip bit 15
        _ => panic!() // Should never happen
    }
}

// Copy bit 15 into bit 16, return new value
pub fn sign_extend(n: u16) -> u16 {
    
    if n & 0x4000 == 0 {
        n & 0x7FFF
    } else {
        n | 0x8000
    }
}

pub fn execute(ins: Instruction) {

    let opcode = (ins & 0x7000) >> 12; // bits 15-13
    let qc = (ins & 0x0C00) >> 10; // bits 12-11
    let er_address: ErasableAddress = ins & 0x03FF; // first 10 bits
    let address: Address = ins & 0x0FFF; // first 12 bits

    MEMORY.write(Z, MEMORY.read(Z) + 1); // Increment program counter

    // Instruction decoding according to AGC's documentation
    if MEMORY.extra.read() == 0 { // Basic instructions
        match opcode {
            0b000 => match address {
                6 => MEMORY.set_extracode(), // EXTEND instruction
                _ => panic!() // Sould be tc(address), but it is not yet implemented
            }
            0b010 => match qc {
                0b00 => das(er_address),
                0b01 => lxch(er_address),
                0b10 => incr(er_address),
                0b11 => ads(er_address),
                _ => panic!()
            }
            0b011 => ca(address),
            0b100 => cs(address),
            0b101 => match qc {
                0b10 => ts(er_address),
                0b11 => xch(er_address),
                _ => panic!()
            }
            0b110 => ad(address),
            0b111 => mask(address),
            _ => panic!(),
        }
    } else { // Extended instructions
        match opcode {
            0b001 => match qc {
                0b00 => panic!(), // DV instruction, not implemented
                0b01 | 0b10 | 0b11 => bzf(address),
                _ => panic!(),
            }
            0b011 => dca(address),
            0b100 => dcs(address),
            0b110 => match qc {
                0b00 => su(er_address),
                0b01 | 0b10 | 0b11 => bzmf(address),
                _ => panic!(),
            }
            _ => panic!()
        }
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

pub fn sign_bit(n: u16) -> Word {
    return (n >> 14) % 2 // returns bit 15
}

// Saves a value overflow-corrected, except when saving to A or Q
// Returns +0 for no overflow, +1 for +overflow and -1(one's complement) for -overflow
pub fn save_corrected(n: u16, k: ErasableAddress) -> Word {
    let sign_bits = n >> 15; // bits 16 and 15
    match sign_bits {
        0b00 | 0b11 => {
            MEMORY.write(k, n);
            return 0;
        }
        // Negative overflow
        0b10 => {
            if is_16bit(k) {
                MEMORY.write(k, n);
            } else {
                MEMORY.write(k, n ^ 0b0100000000000000); // flip sign bit
            }
            return NEG_ONE;
        }
        // Positive overflow
        0b01 => {
            if is_16bit(k) {
                MEMORY.write(k, n);
            } else {
                MEMORY.write(k, n ^ 0b0100000000000000); // flip sign bit
            }
            return 1;
        }
        _ => panic!()
    }
}

// Add
pub fn ad(k: Address) {
    let a: u16 = MEMORY.read(ACC);
    let b: u16 = MEMORY.read(k);
    MEMORY.write(ACC, add_modified(a, b));
}

// Add to storage
pub fn ads(k: ErasableAddress) {
    let a: u16 = MEMORY.read(ACC);
    let b: u16 = MEMORY.read(k);
    let sum = add_modified(a, b);
    MEMORY.write(ACC, sum);

    // This is wrong, the sum should be saved overflow-corrected
    MEMORY.write(k, sum);
}

// Augment
pub fn aug(k: ErasableAddress) {
    let n = MEMORY.read(k);
    if is_16bit(k) {
        if n >> 15 == 0 {MEMORY.write(k, add_modified(n, 1))} // +1
        else {MEMORY.write(k, add_modified(k, NEG_ONE))} // -1
    }
    else {
        if (n >> 14) % 2 == 0 {MEMORY.write(k, add_modified(n, 1))} // +1
        else {MEMORY.write(k, add_modified(k, NEG_ONE))} // -1
    }
}

// Branch zero to fixed
pub fn bzf(k: FixedAddress) {
    let acc = MEMORY.read(ACC);
    if acc == 0 || acc == NEG_ZERO {MEMORY.write(Z, k)} // acc +0 or -0
}

// Branch zero or minus to fixed
pub fn bzmf(k: FixedAddress) {
    let acc = MEMORY.read(ACC);
    if acc == 0 || sign_bit(acc) == 1 {MEMORY.write(Z, k)}
}

// Clear and Add
pub fn ca(k: Address) {
    let n = MEMORY.read(k);
    if is_16bit(k){
        MEMORY.write(ACC, n)
    } else {
        MEMORY.write_acc_signed(n);
    }
}

// Clear and Substract
pub fn cs(k: Address) {
    let n = MEMORY.read(k);
    if is_16bit(k) {
        MEMORY.write(ACC, !n);
    } else {
        MEMORY.write_acc_signed(!n);
    }
}

// Double Add to Storage
pub fn das(k: ErasableAddress) {
    let a = MEMORY.read(ACC);
    let a_low = MEMORY.read(L);
    let b = MEMORY.read(k);
    let b_low = MEMORY.read(k + 1);

    // Add and save lower word, save overflow
    let sum_low = add_modified(a_low, b_low);
    let overflow_low = save_corrected(sum_low, k + 1);

    // Add higher words and previous overflow, save sum
    let mut sum = add_modified(a, b); 
    sum = add_modified(sum, overflow_low);
    let overflow = save_corrected(sum, k);

    // As defined in documentation
    MEMORY.write(ACC, overflow);
    MEMORY.write(L, 0);
} 

// Double Clear and Add
pub fn dca(k: Address) {
    // The AGC processed the instruction in the following order
    let low = MEMORY.read(k + 1);
    MEMORY.write(L, low);
    let high = MEMORY.read(k);
    if is_16bit(k) {
        MEMORY.write(ACC, high);
    } else {
        MEMORY.write_acc_signed(high);
    }
    
}

// Double Clear and Substract
pub fn dcs(k: Address) {
    // The AGC processed the instruction in the following order
    let low = MEMORY.read(k + 1);
    MEMORY.write(L, !low);
    let high = MEMORY.read(k);
    if is_16bit(k) {
        MEMORY.write(ACC, !high);
    } else {
        MEMORY.write_acc_signed(!high);
    }
}

// Diminish
pub fn dim(k: ErasableAddress) {
    let n = MEMORY.read(k);
    if is_16bit(k) {
        if n >> 15 == 0 {MEMORY.write(k, add_modified(n, NEG_ONE))} // +1
        else {MEMORY.write(k, add_modified(k, 1))} // -1
    }
    else {
        if (n >> 14) % 2 == 0 {MEMORY.write(k, add_modified(n, NEG_ONE))} // +1
        else {MEMORY.write(k, add_modified(k, 1))} // -1
    }
}

// Double Exchange
pub fn dxch(k: ErasableAddress) {
    let high = MEMORY.read(k);
    let low = MEMORY.read(k + 1);

    if is_16bit(k) {
        MEMORY.write(ACC, high);
    } else {
        MEMORY.write_acc_signed(high);
    }
    MEMORY.write(L, low);
}

// Increment
pub fn incr(k: ErasableAddress) {
    let n = MEMORY.read(k);
    MEMORY.write(k, add_modified(n, 1));
}

// Exchange L and K
pub fn lxch(k: ErasableAddress) {
    let n = MEMORY.read(k);
    MEMORY.write(k, MEMORY.read(L));
    MEMORY.write(L, n);
}

// AND A and k
pub fn mask(k: Address) {
    let mut acc = MEMORY.read(ACC);
    if is_16bit(k) {
        MEMORY.write(ACC, acc & MEMORY.read(k));
    } else {
        acc = correct(acc); // correct overflow
        acc = ( (acc << 1) & (MEMORY.read(k) << 1) ) >> 1; // AND components (ignoring bit 16)
        acc += (acc & 0b0100000000000000) << 1; // copy bit 15 into bit 16
        MEMORY.write(ACC, acc);
    }
}

// Multiply
pub fn mp(k: Address) {
    // Perform the signed multiplication in two's complement
    let acc = correct(MEMORY.read(ACC));
    let n = MEMORY.read(k);
    let product: i32 = as_i32(acc) * as_i32(n);

    // Handling as defined in documentation
    if product == 0 {
        // acc != +/- 0 or signs are equal
        if ( acc != 0 && acc != NEG_ZERO ) || sign_bit(acc) == sign_bit(n) { 
            MEMORY.write(ACC, 0);
            MEMORY.write(L, 0);
        }
        else {
            MEMORY.write(ACC, NEG_ZERO);
            MEMORY.write(L, NEG_ZERO);
        }
        return ;
    }

    let product = ones_complement32(product);
    let sign_bit = (product >> 31) as u16;
    // SP values are scaled by a factor of 2^-14, that's why the results "start" in the L register
    let low= (product % (2 << 13)) as u16; // Takes the lower 14 bits
    let high = ((product >> 14) % (2 << 13)) as u16; // Takes bits 28 through 15

    MEMORY.write(ACC, high + sign_bit);
    MEMORY.write(L, low + sign_bit);
}

// Modular Substract
// In this instruction the accumulator and the value at k should contain
// a two's complement value
pub fn msu(k: ErasableAddress) {
    let n = MEMORY.read(k);
    let acc = MEMORY.read(ACC);
    if is_16bit(k) {
        let diff = ones_complement16(acc as i16 - n as i16);
        MEMORY.write(ACC, diff as u16);
    } else {
        let mut diff = (acc << 1) as i16 - (n << 1) as i16; // ignore bit 16
        diff >>= 1;
        let mut diff = ones_complement16(diff);
        diff += (diff & 0b0100000000000000) << 1; // copy bit 15 into bit 16
        MEMORY.write(ACC, diff);
    }
}

// Substract
pub fn su(k: ErasableAddress) {
    let acc = MEMORY.read(ACC);
    let mut n = MEMORY.read(k);

    if !is_16bit(k) {
        n = sign_extend(n);
    }
    n = !n;

    let sum = add_modified(acc, n);
    MEMORY.write(ACC, sum);
}

// Exchange A and k
pub fn xch(k: ErasableAddress) {
    let acc = MEMORY.read(ACC);
    let val = MEMORY.read(k);

    if is_16bit(k) {
        MEMORY.write(ACC, val);
        MEMORY.write(k, acc);
    } else {
        MEMORY.write(ACC, sign_extend(val));
        MEMORY.write(k, correct(acc));
    }
}

// Transfer to storage
pub fn ts(k: ErasableAddress) {
    let acc = MEMORY.read(ACC);
    let overflow = save_corrected(acc, k);

    // If the accumulator contained an overflow, skip the next intruction and save either +1 or -1 in the acc, depending on the sign
    // of the overflow. If k is the accumulator itself, leave its value unchanged (OVSK instruction)
    if overflow != 0 {
        if k != 0 {
            MEMORY.write(ACC, overflow);
        }
        MEMORY.write(Z, MEMORY.read(Z) + 1);
    }
}