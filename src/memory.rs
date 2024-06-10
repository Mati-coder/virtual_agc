use core::sync::atomic::AtomicU16;
use core::sync::atomic::Ordering;

const MEMLOC_INITIALIZE: Memloc = Memloc::new(0);
// Useful values named for readability
pub const NEG_ONE: u16 = 0xFFFE; // Negative one represented in one's complement, bit s2 set
pub const NEG_ZERO: u16 = 0xFFFF; // Negative zero in one's complement
pub const ZERO_BIT16: u16 = 0x7FFF; // Mask to zero the bit 16
// Commonly used addresses
pub const ACC: ErasableAddress = 0;
pub const L: ErasableAddress = 1;
pub const Q: ErasableAddress = 2;
pub const Z: ErasableAddress = 5;

// Denotes a 15-bit value
pub type Word = u16; 
// Denotes a 12-bit address
pub type Address = u16;
// Denotes a 12-bit adress that only refers to fixed memory
pub type FixedAddress = u16;
// Denotes a 10-bit address that referenciates erasable memory
pub type ErasableAddress = u16;
// Denotes an instruction
pub type Instruction = u16;


// The memory object
pub static MEMORY: Memory = Memory::new();

pub struct Memory {
    pub central_registers: CentralRegisters,
    pub erasable: ErasableMemory,
    pub fixed: FixedMemory,
    // 16 bit value. Bit 1 is extracode flag
    pub extra: Memloc, 
}
impl Memory {
    const fn new() -> Self{
        Self {
            central_registers: CentralRegisters::new(), erasable: ErasableMemory::new(), 
            fixed: FixedMemory::new(), extra: Memloc::new(0)}
    }

    pub fn write(&self, k: Address, val: u16) {
        let k = k & 0x0FFF; // Extract address
        let val15: Word = val & ZERO_BIT16; // Ensures we never write 16 bit values into 15-bit registers
        match k {
            0b000000000000 ..= 0b000000000111 => self.central_registers.write(k, val),
            0b000000001000 ..= 0b000000110000 => panic!(), // Not implemented yet, contains the especially handled memory locations
            0b000000110001 ..= 0b001111111111 => self.erasable.write(k, val15),
            0b011111111111 ..= 0b111111111111 => panic!(), // Cannot write fixed memory
            _ => panic!(), // Shouldn't happen
        }
    }

    pub fn read(&self, k: Address) -> Word{
        let k = k & 0x0FFF; // Extract 12-bit address
        match k {
            0b000000000000 ..= 0b000000000111 => self.central_registers.read(k),
            0b000000001000 ..= 0b000000110000 => panic!(), // Not implemented yet, contains the especially handled memory locations
            0b000000110001 ..= 0b001111111111 => self.erasable.read(k),
            0b011111111111 ..= 0b111111111111 => self.fixed.read(k),
            _ => panic!() // Shouldn't happen
        }
    }

    pub fn write_acc_signed(&self, val: Word) {
        // Copy bit 15 into bit 16 and writes the new value in the accumulator
        if val & 0x4000 == 0 {
            self.write(ACC, val & 0x7FFF);
        } else {
            self.write(ACC, val | 0x8000);
        }
    }

    pub fn set_extracode(&self) {
        self.extra.write(1)
    }

    pub fn clear_extracode(&self) {
        self.extra.write(0)
    }
}


pub struct ErasableMemory {
    // The first 49 values are the central registers or special memory locations
    pub fixed_bank0: [Memloc; 207], 
}
impl ErasableMemory {
    const fn new() -> Self {
        Self {fixed_bank0: [MEMLOC_INITIALIZE; 207]}
    }

    pub fn read(&self, k: ErasableAddress) -> Word {
        if k > 0xFF {panic!()} // We've only implemented 1 memory bank
        return self.fixed_bank0[(k - 48) as usize].read()
    }

    pub fn write(&self, k: ErasableAddress, val: Word) {
        if k > 0xFF {panic!()} // We've only implemented 1 memory bank
        self.fixed_bank0[(k - 48) as usize].write(val & ZERO_BIT16);
    }
}

pub struct FixedMemory {
    pub fixed_bank0: [Memloc; 1024]
}
impl FixedMemory {
    const fn new() -> Self {
        // We should write our source code here
        Self {
            fixed_bank0: [ MEMLOC_INITIALIZE; 1024 ]
        }
    }

    pub fn read(&self, k: FixedAddress) -> Word {
        if k < 2048 || k >= 3072 {panic!()} // We've only implemented memory bank 0
        return self.fixed_bank0[(k - 2048) as usize].read()
    }

    // Used internally, not accessible to the "programmer"
    pub fn write(&self, k: FixedAddress, val: Word) {
        if k < 2048 || k >= 3072 {panic!()} // We've only implemented memory bank 0
        self.fixed_bank0[(k - 2048) as usize].write(val & ZERO_BIT16);
    }
}

pub struct CentralRegisters {
    pub acc: Memloc, // is 16-bit
    pub l: Memloc, 
    pub q: Memloc, // is 16-bit
    pub bb: Memloc, // contains also the FB and EB registers. 0 FFF FF0 000 000 EEE 
    pub z: Memloc, // is 12-bit
}
impl CentralRegisters {
    const fn new() -> Self {
        Self {
            acc: Memloc::new(0), l: Memloc::new(0), q: Memloc::new(0), 
            bb: Memloc::new(0), z: Memloc::new(0)
        }
    }

    pub fn read(&self, k: ErasableAddress) -> u16 {
        match k {
            0b000 => self.acc.read(),
            0b001 => self.l.read(),
            0b010 => self.q.read(),
            0b011 => (self.bb.read() & 0x0007) << 8, //EB register: 0 000 0EE E00 000 000
            0b100 => self.bb.read() & 0x7C00, //FB register: 0 FFF FF0 000 000 000
            0b101 => self.z.read() & 0x0FFF, // zeroes bits 16-13
            0b110 => self.bb.read(),
            0b111 => 0, // Hard-wired to zero
            _ => panic!()
        }
    }

    pub fn write(&self, k: ErasableAddress, val: u16) {
        // Enforces the real size of registers
        match k {
            0b000 => self.acc.write(val),
            0b001 => self.l.write(val & ZERO_BIT16),
            0b010 => self.q.write(val),
            0b011 => self.bb.write(val & 0x0700), // writes bits 0 000 0xx x00 000 000
            0b100 => self.bb.write(val & 0x7C00), // writes bits 0 xxx xx0 000 000 000
            0b101 => self.z.write(val & 0x0FFF), // zeroes bits 16-13
            0b110 => self.bb.write(val & ZERO_BIT16),
            0b111 | _ => panic!() // As address 0b111 is hard-wired to zero, you should never write to it
        }
    }
}

// Wrapper for managing atomic values
pub struct Memloc {
    pub val: AtomicU16
}
impl Memloc {
    pub const fn new(n: u16) -> Self {
        Self {val: AtomicU16::new(n)}
    }

    pub fn read(&self) -> Word {
        return self.val.load(Ordering::Relaxed);
    }

    pub fn write(&self, val: Word) {
        self.val.store(val, Ordering::Relaxed);
    }
}

pub fn is_16bit(k: Address) -> bool {
    match k {
        ACC | Q => true, 
        _ => false
    }
}