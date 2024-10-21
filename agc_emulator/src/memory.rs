use core::sync::atomic::AtomicU16;
use core::sync::atomic::Ordering;

// Constant for memory initialization
const MEMLOC_INITIALIZE: Memloc = Memloc::new(0);
// Useful values named for readability
pub const NEG_ONE: u16 = 0xFFFE; // Negative one represented in one's complement, bit s2 set
pub const NEG_ZERO: u16 = 0xFFFF; // Negative zero in one's complement
pub const ZERO_BIT16: u16 = 0x7FFF; // Mask to zero the bit 16
// Registers
macro_rules! register {
    ($name:ident, $value:literal) => {
        pub const $name: ErasableAddress = $value;
    };
}
register!(ZERO, 7);
register!(ACC, 0);
register!(L, 1);
register!(Q, 2);
register!(Z, 5);

// Denotes an AGC word
pub type Word = u16;

// Denotes a 12-bit address
pub type Address = u16;

// Denotes a 12-bit adress that only refers to fixed memory
pub type FixedAddress = u16;

// Denotes a 10-bit address that referenciates erasable memory
pub type ErasableAddress = u16;


// The memory object
pub static MEMORY: Memory = Memory::new();

pub fn is_16bit(k: Address) -> bool {
    match k {
        ACC | Q => true, 
        _ => false
    }
}

#[derive(Debug)]
pub struct Memory {
    central_registers: CentralRegisters,
    erasable: ErasableMemory,
    fixed: FixedMemory,
    // 16 bit value. Bit 1 is extracode flag. Bit 2 enables interrups
    extra: Memloc,
    // Indexing value added to the next instruction's address 
    index: Memloc,
}
impl Memory {
    const fn new() -> Self{
        Self {
            central_registers: CentralRegisters::new(), erasable: ErasableMemory::new(), 
            fixed: FixedMemory::new(), extra: Memloc::new(0), index: Memloc::new(0)
        }
    }

    pub fn write(&self, k: Address, val: u16) {
        let k: Address = k & 0x0FFF; // Extract address
        let val15: Word = val & ZERO_BIT16; // Ensures we never write 16 bit values into 15-bit registers
        match k {
            0 ..= 7 => self.central_registers.write(k, val),
            8 ..= 47 => unimplemented!(), // Contains the especially handled memory locations
            48 ..= 2047 => self.erasable.write(k, val15),
            2048 ..= 4095 => panic!("Tried to write to fixed memory"), // Cannot write fixed memory
            _ => unreachable!(),
        }
    }

    pub fn read(&self, k: Address) -> Word{
        let k = k & 0x0FFF; // Extract 12-bit address
        match k {
            0 ..= 7 => self.central_registers.read(k),
            8 ..= 47 => unimplemented!(), // Contains the especially handled memory locations
            48 ..= 1023 => self.erasable.read(k),
            1024 ..= 4095 => self.fixed.read(k),
            _ => unreachable!()
        }
    }
    
    pub fn get_address_name(&self, addr: Address) -> &'static str {
        include!("../memory/names.in")
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

    pub fn relint(&self) {
        self.extra.write(self.extra.read() | 0x0002) // Set bit 2
    }

    pub fn inhint(&self) {
        self.extra.write(self.extra.read() & 0xFFFD) // Clear bit 2
    }

    pub fn set_index(&self, val: Word) {
        self.index.write(val)
    }

    pub fn clear_index(&self) {
        self.index.write(0)
    }

    pub fn get_index(&self) -> Word {
        self.index.read()
    }
}

#[derive(Debug)]
struct ErasableMemory {
    // The first 49 values are the central registers or special memory locations
    erasable_bank1: [Memloc; 256], 
}
impl ErasableMemory {
    const fn new() -> Self {
        Self {erasable_bank1: [MEMLOC_INITIALIZE; 256]}
    }

    fn read(&self, k: ErasableAddress) -> Word {
        if k < 255 || k >= 512 {unimplemented!()} // We've only implemented 1 memory bank
        return self.erasable_bank1[(k - 256) as usize].read()
    }

    fn write(&self, k: ErasableAddress, val: Word) {
        if k < 255 || k >= 512 {unimplemented!()} // We've only implemented 1 memory bank
        self.erasable_bank1[(k - 256) as usize].write(val);
    }
}

#[derive(Debug)]
struct FixedMemory {
    fixed_bank1: [Memloc; 1024]
}
impl FixedMemory {
    const fn new() -> Self {
        Self {
            fixed_bank1: include!("../memory/fixed.in"),
        }
    }

    fn read(&self, k: FixedAddress) -> Word {
        if k < 2048 || k >= 3072 {unimplemented!()} // We've only implemented memory bank 0
        return self.fixed_bank1[(k - 2048) as usize].read()
    }

    // Just for debug and testing purposes, not accessible to the "programmer"
    pub(crate) fn write(&self, k: FixedAddress, val: Word) {
        if k < 2048 || k >= 3072 {unimplemented!()} // We've only implemented memory bank 0
        self.fixed_bank1[(k - 2048) as usize].write(val & ZERO_BIT16);
    }
}

#[derive(Debug)]
struct CentralRegisters {
    acc: Memloc, // is 16-bit
    l: Memloc, 
    q: Memloc, // is 16-bit
    bb: Memloc, // contains also the FB and EB registers. 0 FFF FF0 000 000 EEE 
    z: Memloc, // is 12-bit
}
impl CentralRegisters {
    const fn new() -> Self {
        Self {
            acc: Memloc::new(0), l: Memloc::new(0), q: Memloc::new(0), 
            bb: Memloc::new(0), z: Memloc::new(2048)
        }
    }

    fn read(&self, k: ErasableAddress) -> u16 {
        match k {
            0 => self.acc.read(),
            1 => self.l.read(),
            2 => self.q.read(),
            3 => (self.bb.read() & 0x0007) << 8, //EB register: 0 000 0EE E00 000 000
            4 => self.bb.read() & 0x7C00, //FB register: 0 FFF FF0 000 000 000
            5 => self.z.read(), // zeroes bits 16-13
            6 => self.bb.read(),
            7 => 0, // Hard-wired to zero
            _ => panic!()
        }
    }

    fn write(&self, k: ErasableAddress, val: u16) {
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
#[derive(Debug)]
struct Memloc {
    val: AtomicU16
}
impl Memloc {
    const fn new(n: u16) -> Self {
        Self {val: AtomicU16::new(n)}
    }

    fn read(&self) -> Word {
        return self.val.load(Ordering::Relaxed);
    }

    fn write(&self, val: Word) {
        self.val.store(val, Ordering::Relaxed);
    }
}