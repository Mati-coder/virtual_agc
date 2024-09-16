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

pub const IMPLIED: [&'static str; 2] = [
    "EXTEND",
    "RETURN",
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