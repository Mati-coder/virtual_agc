use core::sync::atomic::AtomicU16;
use core::sync::atomic::Ordering;

pub const NEG_ONE: u16 = 0b1111111111111110; // Negative one represented in one's complement, bit s2 set
pub const NEG_ZERO: u16 = 0xFFFF; // Negative zero in one's complement
pub static CR: CentralRegisters = CentralRegisters::new();
// Provisional
pub static EM: [Memloc; 16] = initialize_memory();
// We will use the convention that a word means 'we only use 15 bits'. Bit 16 should always be zero
// This convention doesn't hold for operations that may use the value of the acc, which is 16 bits long,
// such as read or write memory operations. Any other function that takes a Word as a parameter operates
// with the convention in mind.
pub type Word = u16; 
// provisional
pub type Address = u16;
pub type FixedAddress = u16;
pub type ErasableAddress = u16;

pub struct CentralRegisters {
    pub acc: Memloc,
    pub l: Memloc,
    pub z: Memloc,
}
impl CentralRegisters {
    const fn new() -> Self {
        Self {acc: Memloc::new(0), l: Memloc::new(0), z: Memloc::new(0)}
    }
}

// Wrapper for managing atomic values
pub struct Memloc {
    pub val: AtomicU16 // In reality just 15 bits are used
}

impl Memloc {
    pub const fn new(n: u16) -> Self {
        Self {val: AtomicU16::new(n)}
    }

    pub fn load(&self) -> Word {
        return self.val.load(Ordering::Relaxed);
    }

    pub fn write(&self, val: Word) {
        self.val.store(val, Ordering::Relaxed);
    }
}

pub fn is_16bit(k: Address) -> bool {
    match k {
        0 | 2 => true, // Registers A and Q
        _ => false
    }
}

// Provisional
const fn initialize_memory() -> [Memloc; 16] {
    let mem: [Memloc; 16] = [
        Memloc::new(0), Memloc::new(0), Memloc::new(0), Memloc::new(0), 
        Memloc::new(0), Memloc::new(0), Memloc::new(0), Memloc::new(0), 
        Memloc::new(0), Memloc::new(0), Memloc::new(0), Memloc::new(0), 
        Memloc::new(0), Memloc::new(0), Memloc::new(0), Memloc::new(0),
    ];
    mem
}