
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SymbolType {
    Label,
    Variable,
    LabelTable(u16),
    VariableTable(u16),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Section {
    None,
    Config,
    Code,
    Data,
}
impl From<&str> for Section {
    fn from(value: &str) -> Self {
        match value {
            "config" => Section::Config,
            "code" => Section::Code,
            "data" => Section::Data,
            _ => panic!("Invalid section (thrown from conversion)")
        }
    }
}

#[derive(Debug, Eq)]
pub struct UndefinedLabel<'a> {
    pub name: &'a str,
    pub section: Section,
    pub offset: u16,
}
impl<'a> PartialEq for UndefinedLabel<'a> {
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
    
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<'a> UndefinedLabel<'a> {
    pub fn new(name: &'a str, section: Section, offset: u16) -> Self {
        Self{name, section, offset}
    }

    pub fn define(&self, section_start: u16, len: u16) -> DefinedSymbol<'a> {
        if len == 0 {
            DefinedSymbol::new(self.name, SymbolType::Label, section_start + self.offset)
        } else {
            DefinedSymbol::new(self.name, SymbolType::LabelTable(len), section_start + self.offset)
        }
    }
}

#[derive(Debug)]
pub struct UndefinedSymbol<'a> {
    pub name: &'a str,
    pub r#type: Option<SymbolType>,
}
impl<'a> UndefinedSymbol<'a> {
    pub const fn new (name: &'a str, r#type: Option<SymbolType>) -> Self {
        Self {name, r#type}
    }

    pub fn define(&self, value: u16, len: u16) -> DefinedSymbol<'a> {
        if len == 0 {
            DefinedSymbol::new(self.name, self.r#type.unwrap_or(SymbolType::Variable), value)
        } else {
            if self.r#type == Some(SymbolType::Variable) {
                DefinedSymbol::new(self.name, SymbolType::VariableTable(len), value)
            } else {
                DefinedSymbol::new(self.name, SymbolType::Label, value)
            }
            
        }
    }
}

#[derive(Debug, Eq)]
pub struct DefinedSymbol<'a> {
    pub name: &'a str,
    pub r#type: SymbolType,
    pub address: u16,
}
impl<'a> PartialEq for DefinedSymbol<'a> {
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
    
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl<'a> DefinedSymbol<'a> {
    pub const fn new (name: &'a str, r#type: SymbolType, address: u16) -> Self {
        Self {name, r#type, address}
    }
}

#[derive(Debug)]
pub struct Instruction<'a> {
    pub operation: &'a str,
    pub operand: UndefinedSymbol<'a>,
}
impl<'a> Instruction<'a> {
    pub fn new(operation: &'a str, operand: UndefinedSymbol<'a>) -> Self {
        Self {operation, operand}
    }
}

#[derive(Debug)]
pub struct FileContent<'a> {
    pub labels: Vec<UndefinedLabel<'a>>,
    pub code: Vec<Instruction<'a>>,
    pub data: Vec<u16>
}

impl FileContent<'_> {
    pub const fn new() -> Self {
        Self{labels: vec![],code: vec![],data: vec![]}
    }
}

#[derive(Debug)]
pub struct UndefinedTable<'a> {
    pub name: &'a str,
    pub len: u16,
}

impl<'a> UndefinedTable<'a> {
    pub fn new(name: &'a str, len: u16) -> Self {
        UndefinedTable {name, len}
    }
}
impl<'a> PartialEq for UndefinedTable<'a> {
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
    
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}