pub struct CentralRegisters {
    pub acc: Word,
}

pub struct Word {
    pub val: u16 // In reality just 15 bits are used
}

pub struct DoubleWord {
    pub val: u32 // In reality just 30 bits are used
}

impl Word {
    pub fn as_i16(self) -> i16 {
        let v = ((self.val << 1) as i16) / 2;
        if v < 0 {return v - 1;}
        else {return v;}
    }
    pub fn as_u16(self) -> u16 {
        return self.val;
    }
}