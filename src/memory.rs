pub struct CentralRegisters {
    pub acc: Word,
}

pub struct Word {
    pub val: u16,
}

impl Word {
    pub fn read(self) -> i16 {
        let v = (self.val << 1 / 2) as i16;
        if v > 0 {return v}
        else {return v}
    }
}