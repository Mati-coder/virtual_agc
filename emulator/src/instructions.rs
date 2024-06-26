use crate::Memory::*;

// Pure instructions
pub const AD: u16 =     0b110000000000000;
pub const ADS: u16 =    0b010110000000000;
pub const AUG: u16 =    0b010100000000000;
pub const BZF: u16 =    0b001000000000000;
pub const BZMF: u16 =   0b110000000000000;
pub const CA: u16 =     0b011000000000000;
pub const CCS: u16 =    0b001000000000000;
pub const CS: u16 =     0b100000000000000;
pub const DAS: u16 =    0b010000000000001;
pub const DCA: u16 =    0b011000000000001;
pub const DCS: u16 =    0b100000000000001;
pub const DIM: u16 =    0b010110000000000;
pub const DV: u16 =     0b001000000000000;
pub const DXCH: u16 =   0b101010000000001;
pub const INCR: u16 =   0b010100000000000;
pub const INDEX: u16 =  0b101000000000000;
pub const LXCH: u16 =   0b010010000000000;
pub const MASK: u16 =   0b111000000000000;
pub const MP: u16 =     0b111000000000000;
pub const MSU: u16 =    0b010000000000000;
pub const QXCH: u16 =   0b010010000000000;
pub const SU: u16 =     0b110000000000000;
pub const TC: u16 =     0b000000000000000;
pub const TCF: u16 =    0b001000000000000;
pub const TS: u16 =     0b101100000000000;
pub const XCH: u16 =    0b101110000000000;

// Implied instructions, special meaning
pub const EXTEND: u16 = 6;
pub const INHINT: u16 = 4;
pub const RELINT: u16 = 3;
pub const RETURN: u16 = 2;

// Named for convenience
pub const COM: u16 =    0b100000000000000;
pub const DCOM: u16 =   0b100000000000001;
pub const DDOUBL: u16 = 0b001000000000001;
pub const DOUBLE: u16 = 0b011000000000000;
pub const DTCB: u16 =   0b010101000000110;
pub const DTCF: u16 =   0b010101000000101;
pub const OVSK: u16 =   0b010110000000000;
pub const SQUARE: u16 = 0b011100000000000;
pub const ZL: u16 =     0b001001000000111;
pub const ZQ: u16 =     0b001001000000111;

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
        _ => unreachable!() // Should never happen
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

pub fn read_16(k: Address) -> u16 {
    if is_16bit(k) {
        return MEMORY.read(k);
    }

    return sign_extend(MEMORY.read(k));
}

pub fn execute(mut ins: Instruction) {
    ins += MEMORY.index.read();

    let opcode = (ins & 0x7000) >> 12; // bits 15-13
    let qc = (ins & 0x0C00) >> 10; // bits 12-11
    let er_address: ErasableAddress = ins & 0x03FF; // first 10 bits
    let address: Address = ins & 0x0FFF; // first 12 bits
    let extracode: bool = MEMORY.extracode();

    MEMORY.write(Z, MEMORY.read(Z) + 1); // Increment program counter
    MEMORY.clear_index();
    MEMORY.clear_extracode();

    // Instruction decoding according to AGC's documentation
    if !extracode { // Basic instructions
        match opcode {
            0 => match address {
                2 => MEMORY.write(Z, Q), // RETURN
                3 => MEMORY.relint(), // RELINT
                4 => MEMORY.inhint(), // INHINT
                6 => MEMORY.set_extracode(), // EXTEND
                _ => tc(address)
            }
            1 => match qc {
                0 => ccs(er_address),
                1 | 2 | 3 => tcf(address),
                _ => unreachable!(),
            }
            2 => match qc {
                0 => das(er_address),
                1 => lxch(er_address),
                2 => incr(er_address),
                3 => ads(er_address),
                _ => unreachable!()
            }
            3 => ca(address),
            4 => cs(address),
            5 => match qc {
                0 => if address == 15 {
                    unimplemented!() // should be RESUME
                } else {
                    MEMORY.set_index(MEMORY.read(address)); // INDEX
                }
                1 => dxch(er_address),
                2 => ts(er_address),
                3 => xch(er_address),
                _ => unreachable!()
            }
            6 => ad(address),
            7 => mask(address),
            _ => unreachable!(),
        }
    } else { // Extended instructions
        match opcode {
            1 => match qc {
                0 => unimplemented!(), // DV instruction, not implemented
                1 | 2 | 3 => bzf(address),
                _ => unreachable!(),
            }
            2 => match qc {
                0 => msu(er_address),
                1 => qxch(er_address),
                2 => aug(er_address),
                3 => dim(er_address),
                _ => unreachable!(),
            }
            3 => dca(address),
            4 => dcs(address),
            5 => {
                MEMORY.set_index(MEMORY.read(address)); // INDEX
                MEMORY.set_extracode(); // Keep extracode flag
                } 
            6 => match qc {
                0 => su(er_address),
                1 | 2 | 3 => bzmf(address),
                _ => unreachable!(),
            }
            7 => mp(address),
            _ => unreachable!()
        }
    }
}

pub fn as_i32(n: u16) -> i32 {
    let mut v = ((n << 1) as i32) >> 1; // Ignores the 16th bit
    // Copy bit 15 into bit 32
    if (v & 0x4000) == 0 {
        v &= 0x7FFFFFFF;
    } else {
        v |= (0x80000000 as u32) as i32;
    }
    if v < 0 {return (v | 0x7FFF8000) + 1;} // Because v is in one's complement
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
    let sign_bits = n >> 14; // bits 16 and 15
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
        _ => unreachable!()
    }
}

// Add
pub fn ad(k: Address) {
    let a: u16 = MEMORY.read(ACC);
    let b: u16 = read_16(k);
    
    MEMORY.write(ACC, add_modified(a, b));
}

// Add to storage
pub fn ads(k: ErasableAddress) {
    let a: u16 = MEMORY.read(ACC);
    let b: u16 = read_16(k);
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
        else {MEMORY.write(k, add_modified(n, NEG_ONE))} // -1
    }
    else {
        if (n >> 14) % 2 == 0 {MEMORY.write(k, add_modified(n, 1))} // +1
        else {MEMORY.write(k, add_modified(n, NEG_ONE))} // -1
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
        MEMORY.write(ACC, sign_extend(n));
    }
}

// Clear and Substract
pub fn cs(k: Address) {
    let n = MEMORY.read(k);
    if is_16bit(k) {
        MEMORY.write(ACC, !n);
    } else {
        MEMORY.write(ACC, sign_extend(!n));
    }
}

// Cound, Compare and Skip
pub fn ccs(k: ErasableAddress) {
    let n = MEMORY.read(k);


    if is_16bit(k) {
        // Value is interpreted as 16bits
        // Negative n
        if n & 0x8000 != 0 {
            let mut dabs = !n; // Take absolute value
            if dabs >= 1 {dabs -= 1;} // Decrement if posible
            MEMORY.write(ACC, dabs); // Write to the acc
        } else { // Positive n
            let mut dabs = n; // Take absolute value
            if dabs >= 1 {dabs -= 1;} // Decrement if posible
            MEMORY.write(ACC, dabs); // Write to the acc
        }
    } else {
        // Value is interpreted as 15bits, thus requires bit extension
        // Negative n
        if n & 0x4000 != 0 {
            let mut dabs = !sign_extend(n); // Take absolute value
            if dabs >= 1 {dabs -= 1;} // Decrement if posible
            MEMORY.write(ACC, dabs); // Write to the acc
        } else { // Positive n
            let mut dabs = sign_extend(n); // Take absolute value
            if dabs >= 1 {dabs -= 1;} // Decrement if posible
            MEMORY.write(ACC, dabs); // Write to the acc
        }
    }

    // Perform jump
    if n == 0 {
        MEMORY.write(Z, MEMORY.read(Z) + 1); // Jump 2 places
    }
    if  (is_16bit(k) && n & 0x8000 != 0) || // negative overflow
        (!is_16bit(k) && n & 0x4000 != 0) // n negative
    {
        MEMORY.write(Z, MEMORY.read(Z) + 2) // Jump 3 places
    }
    // n equals -0
    if  (is_16bit(k) && n == NEG_ZERO) ||
        (!is_16bit(k) && sign_extend(n) == NEG_ZERO)
    {
        MEMORY.write(Z, MEMORY.read(Z) + 3); // Jump 4 places
    }
    // Normal execution happens when n > 0 or n has positive overflow
}

// Double Add to Storage
pub fn das(k: ErasableAddress) {
    let a = MEMORY.read(ACC);
    let a_low = MEMORY.read(L);
    let b = MEMORY.read(k - 1);
    let b_low = MEMORY.read(k);

    // Add and save lower word, save overflow
    let sum_low = add_modified(a_low, b_low);
    let overflow_low = save_corrected(sum_low, k);

    // Add higher words and previous overflow, save sum
    let mut sum = add_modified(a, b); 
    sum = add_modified(sum, overflow_low);
    let overflow = save_corrected(sum, k - 1);

    // As defined in documentation
    MEMORY.write(ACC, overflow);
    MEMORY.write(L, 0);
} 

// Double Clear and Add
pub fn dca(k: Address) {
    // The AGC processed the instruction in the following order
    let low = MEMORY.read(k);
    MEMORY.write(L, low);
    let high = MEMORY.read(k - 1);
    if is_16bit(k - 1) {
        MEMORY.write(ACC, high);
    } else {
        MEMORY.write(ACC, sign_extend(high));
    }
}

// Double Clear and Substract
pub fn dcs(k: Address) {
    // The AGC processed the instruction in the following order
    let low = MEMORY.read(k);
    MEMORY.write(L, !low);
    let high = MEMORY.read(k - 1);
    if is_16bit(k - 1) {
        MEMORY.write(ACC, !high);
    } else {
        MEMORY.write(ACC, sign_extend(!high));
    }
}

// Diminish
pub fn dim(k: ErasableAddress) {
    let n = MEMORY.read(k);
    if is_16bit(k) {
        if n >> 15 == 0 {MEMORY.write(k, add_modified(n, NEG_ONE))} // +1
        else {MEMORY.write(k, add_modified(n, 1))} // -1
    }
    else {
        if (n >> 14) % 2 == 0 {MEMORY.write(k, add_modified(n, NEG_ONE))} // +1
        else {MEMORY.write(k, add_modified(n, 1))} // -1
    }
}

// Double Exchange
pub fn dxch(k: ErasableAddress) {
    let high = MEMORY.read(k - 1);
    let low = MEMORY.read(k);

    if is_16bit(k) {
        MEMORY.write(ACC, high);
    } else {
        MEMORY.write(ACC, sign_extend(high));
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

// Exchange Q and k
pub fn qxch(k: ErasableAddress) {
    let q = MEMORY.read(Q);
    let val = MEMORY.read(k);

    if is_16bit(k) {
        MEMORY.write(Q, val);
        MEMORY.write(k, q);
    } else {
        MEMORY.write(Q, sign_extend(val));
        MEMORY.write(k, correct(q));
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

// Transfer control setting up return
pub fn tc(k: Address) {
    let z = MEMORY.read(Z);
    MEMORY.write(Z, k);
    MEMORY.write(Q, z);
}

// Transfer control to fixed (does not set up return)
pub fn tcf(k: FixedAddress) {
    MEMORY.write(Z, k);
}