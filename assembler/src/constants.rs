use crate::DefinedSymbol;

pub const GENERAL: [&'static str; 9] = [
    "CA",
    "INDEX",
    "TC",
    "CS",
    "AD",
    "MASK",
    "DCA",
    "DCS",
    "MP",
];

pub const ERASABLE: [&'static str; 14] = [
    "CCS",
    "TS",
    "DIM",
    "ADS",
    "AUG",
    "DAS",
    "DV",
    "DXCH",
    "INCR",
    "LXCH",
    "MSU",
    "QXCH",
    "SU",
    "XCH",
];

pub const FIXED: [&'static str; 3] = [
    "TCF",
    "BZMF",
    "BZF"
];

pub const IMPLIED: [&'static str; 3] = [
    "EXTEND",
    "RETURN",
    "COM"
];

pub const EXTENDED: [&'static str; 11] = [
    "DV",
    "BZF",
    "MSU",
    "QXCH",
    "AUG",
    "DIM",
    "DCA",
    "DCS",
    "SU",
    "BZMF",
    "MP",
];

pub const ACC: DefinedSymbol = DefinedSymbol::new("ACC", crate::SymbolType::Variable, 0);
pub const L: DefinedSymbol = DefinedSymbol::new("L", crate::SymbolType::Variable, 1);
pub const Q: DefinedSymbol = DefinedSymbol::new("Q", crate::SymbolType::Variable, 2);
pub const Z: DefinedSymbol = DefinedSymbol::new("Z", crate::SymbolType::Variable, 5);
pub const BB: DefinedSymbol = DefinedSymbol::new("BB", crate::SymbolType::Variable, 6);
pub const ZERO: DefinedSymbol = DefinedSymbol::new("ZERO", crate::SymbolType::Variable, 7);
pub const PANT: DefinedSymbol = DefinedSymbol::new("PANT", crate::SymbolType::VariableTable(8), 256);
pub const BTNUP: DefinedSymbol = DefinedSymbol::new("BTNUP", crate::SymbolType::Variable, 264);
pub const BTNRGT: DefinedSymbol = DefinedSymbol::new("BTNRGT", crate::SymbolType::Variable, 265);
pub const BTNDWN: DefinedSymbol = DefinedSymbol::new("BTNDWN", crate::SymbolType::Variable, 266);
pub const BTNLFT: DefinedSymbol = DefinedSymbol::new("BTNLFT", crate::SymbolType::Variable, 267);
pub const BTN1: DefinedSymbol = DefinedSymbol::new("BTN1", crate::SymbolType::Variable, 268);
pub const BTN2: DefinedSymbol = DefinedSymbol::new("BTN2", crate::SymbolType::Variable, 269);
pub const POTE: DefinedSymbol = DefinedSymbol::new("POTE", crate::SymbolType::Variable, 270);
pub const CORTO: DefinedSymbol = DefinedSymbol::new("CORTO", crate::SymbolType::Variable, 271);
pub const MEDIO: DefinedSymbol = DefinedSymbol::new("MEDIO", crate::SymbolType::Variable, 272);
pub const LARGO: DefinedSymbol = DefinedSymbol::new("LARGO", crate::SymbolType::Variable, 273);

pub const RAM_START: u16 = 274;
pub const FIXED_START: usize = 2048;

pub fn decode(operation: &str) -> u16 {
    match operation {
        "CA"=>     0b011000000000000,
        "INDEX"=>  0b101000000000000,
        "TC"=>     0b000000000000000,
        "CS"=>     0b100000000000000,
        "AD"=>     0b110000000000000,
        "MASK"=>   0b111000000000000,
        "DCA"=>    0b011000000000001,
        "DCS"=>    0b100000000000001,
        "MP"=>     0b111000000000000,
        "CCS"=>    0b001000000000000,
        "TS"=>     0b101100000000000,
        "DIM"=>    0b010110000000000,
        "ADS"=>    0b010110000000000,
        "AUG"=>    0b010100000000000,
        "DAS"=>    0b010000000000001,
        "DV"=>     0b001000000000000,
        "DXCH"=>   0b101010000000001,
        "INCR"=>   0b010100000000000,
        "LXCH"=>   0b010010000000000,
        "MSU"=>    0b010000000000000,
        "QXCH"=>   0b010010000000000,
        "SU"=>     0b110000000000000,
        "XCH"=>    0b101110000000000,
        "TCF"=>    0b001000000000000,
        "BZF"=>    0b001000000000000,
        "BZMF"=>   0b110000000000000,
        "COM"=>    0b100000000000000,
        "EXTEND"=> 6,
        "RETURN"=> 2,
        _ => panic!("INVALID")
    }
}