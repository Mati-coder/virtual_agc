use core::sync::atomic::AtomicU16;
use core::sync::atomic::Ordering;
mod fixed_memory_initialization;

// Constant for memory initialization
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
    // 16 bit value. Bit 1 is extracode flag. Bit 2 enables interrups
    pub extra: Memloc,
    // Indexing value added to the next instruction's address 
    pub index: Memloc,
}
impl Memory {
    const fn new() -> Self{
        Self {
            central_registers: CentralRegisters::new(), erasable: ErasableMemory::new(), 
            fixed: FixedMemory::new(), extra: Memloc::new(0), index: Memloc::new(0)}
    }

    pub fn write(&self, k: Address, val: u16) {
        let k = k & 0x0FFF; // Extract address
        let val15: Word = val & ZERO_BIT16; // Ensures we never write 16 bit values into 15-bit registers
        match k {
            0 ..= 7 => self.central_registers.write(k, val),
            8 ..= 48 => unimplemented!(), // Not implemented, contains the especially handled memory locations
            49 ..= 2047 => self.erasable.write(k, val15),
            2048 ..= 4095 => panic!("Tried to write to fixed memory"), // Cannot write fixed memory
            _ => unreachable!(),
        }
    }

    pub fn read(&self, k: Address) -> Word{
        let k = k & 0x0FFF; // Extract 12-bit address
        match k {
            0 ..= 7 => self.central_registers.read(k),
            8 ..= 48 => unimplemented!(), // Not implemented yet, contains the especially handled memory locations
            49 ..= 1023 => self.erasable.read(k),
            1024 ..= 4095 => self.fixed.read(k),
            _ => unreachable!()
        }
    }

    pub fn set_extracode(&self) {
        self.extra.write(self.extra.read() | 0x0001) // Set bit 1
    }

    pub fn clear_extracode(&self) {
        self.extra.write(self.extra.read() & 0xFFFE) // Clear bit 1
    }

    pub fn extracode(&self) -> bool {
        self.extra.read() % 2 != 0
    }

    pub fn inhint(&self) {
        self.extra.write(self.extra.read() & 0xFFFD) // Clear bit 2
    }

    pub fn relint(&self) {
        self.extra.write(self.extra.read() | 0x0002) // Set bit 2
    }

    pub fn set_index(&self, val: Word) {
        self.index.write(val)
    }

    pub fn clear_index(&self) {
        self.index.write(0)
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
        if k > 255 {unimplemented!()} // We've only implemented 1 memory bank
        return self.fixed_bank0[(k - 48) as usize].read()
    }

    pub fn write(&self, k: ErasableAddress, val: Word) {
        if k > 255 {unimplemented!()} // We've only implemented 1 memory bank
        self.fixed_bank0[(k - 48) as usize].write(val & ZERO_BIT16);
    }
}

pub struct FixedMemory {
    pub fixed_bank0: [Memloc; 1024]
}
impl FixedMemory {
    // Inicialization function in another file

    pub fn read(&self, k: FixedAddress) -> Word {
        if k < 2048 || k >= 3072 {unimplemented!()} // We've only implemented memory bank 0
        return self.fixed_bank0[(k - 2048) as usize].read()
    }

    // Used internally, not accessible to the "programmer"
    pub fn write(&self, k: FixedAddress, val: Word) {
        if k < 2048 || k >= 3072 {unimplemented!()} // We've only implemented memory bank 0
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
            bb: Memloc::new(0), z: Memloc::new(2048)
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
            0 => self.acc.write(val),
            1 => self.l.write(val & ZERO_BIT16),
            2 => self.q.write(val),
            3 => self.bb.write(val & 0x0700), // writes bits 0 000 0xx x00 000 000
            4 => self.bb.write(val & 0x7C00), // writes bits 0 xxx xx0 000 000 000
            5 => self.z.write(val & 0x0FFF), // zeroes bits 16-13
            6 => self.bb.write(val & ZERO_BIT16),
            7 => panic!("Tried to write to address 7"), // As address 0b111 is hard-wired to zero, you should never write to it
            _ => unreachable!()
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