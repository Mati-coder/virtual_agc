use std::fs;
mod constants;
use constants::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum SymbolType {
    Label,
    Variable,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Section {
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
struct UndefinedLabel<'a> {
    name: &'a str,
    section: Section,
    offset: u16,
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
    fn new(name: &'a str, section: Section, offset: u16) -> Self {
        Self{name, section, offset}
    }

    fn define(&self, section_start: u16) -> DefinedSymbol<'a> {
        DefinedSymbol::new(self.name, SymbolType::Label, section_start + self.offset)
    }
}

#[derive(Debug)]
struct UndefinedSymbol<'a> {
    name: &'a str,
    r#type: Option<SymbolType>,
}
impl<'a> UndefinedSymbol<'a> {
    const fn new (name: &'a str, r#type: Option<SymbolType>) -> Self {
        Self {name, r#type}
    }

    fn define(&self, value: u16) -> DefinedSymbol<'a> {
        DefinedSymbol::new(self.name, self.r#type.unwrap_or(SymbolType::Variable), value)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ExternalSymbol<'a> {
    name: &'a str,
}
impl<'a> From<&'a str> for ExternalSymbol<'a> {
    fn from(s: &'a str) -> Self {
        Self {name: s}
    }
}

#[derive(Debug, Eq)]
struct DefinedSymbol<'a> {
    name: &'a str,
    r#type: SymbolType,
    value: u16,
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
    const fn new (name: &'a str, r#type: SymbolType, value: u16) -> Self {
        Self {name, r#type, value}
    }
}

#[derive(Debug)]
struct Instruction<'a> {
    operation: &'a str,
    operand: UndefinedSymbol<'a>,
}
impl<'a> Instruction<'a> {
    fn new(operation: &'a str, operand: UndefinedSymbol<'a>) -> Self {
        Self {operation, operand}
    }
}

#[derive(Debug)]
struct FileContent<'a> {
    external: Vec<ExternalSymbol<'a>>,
    labels: Vec<UndefinedLabel<'a>>,
    code: Vec<Instruction<'a>>,
    data: Vec<&'a str>
}

impl FileContent<'_> {
    fn new() -> Self {
        Self{external: vec![],labels: vec![],code: vec![],data: vec![]}
    }
}

fn main() {
    let files = [
        fs::read_to_string("../programs/threshold.agc").unwrap(),
        fs::read_to_string("../programs/end_loop.agc").unwrap(),
    ];

    let sections = [Section::None, Section::Config, Section::Code, Section::Data];
    let mut contents: [FileContent; 2] = [FileContent::new(), FileContent::new()];

    for file_index in 0..files.len() {
        let mut current_section = Section::None;
        let mut next_extended = false;
        for ln in files[file_index].lines() {
            let mut line = ln.split_whitespace();
            let first = line.next();

            // Ignore blank lines
            if first == None {
                continue;
            }
            let first = first.unwrap();

            // Ignore comments
            if first.starts_with("#") {
                continue;
            }

            // Handle sections
            if first.starts_with(".") {
                let sec = &first[1..];
                let sec = sec.into();

                if !sections.contains(&sec) {
                    panic!("Invalid section name")
                }

                if sections.iter().position(|s| s == &sec).unwrap()
                < sections.iter().position(|s| s == &current_section).unwrap() {
                    panic!("There can only be one section of each type, and should be in the order 'config', 'code', 'data'")
                }

                current_section = sec;
                continue;
            }

            if current_section == Section::Config {
                if first != "EXTERN" {
                    panic!("Only EXTERN in config")
                }
                
                let symbol = line.next().expect("No Symbol");

                if contents[file_index].external.contains(&symbol.into()) {
                    panic!("Repeated symbol {}", symbol);
                }

                contents[file_index].external.push(symbol.into());
            }

            if current_section == Section::Data {
                // For labels
                if first.ends_with(":") {
                    let name = &first[..first.len()-1];

                    if contents[file_index].external.contains(&name.into()) {
                        continue;
                    }

                    let label = UndefinedLabel::new(name, Section::Data, contents[file_index].data.len() as u16);

                    if contents[file_index].labels.contains(&label) {
                        panic!("Duplicated label")
                    }

                    contents[file_index].labels.push(label);
                    continue;
                }

                if first != "DEC" {
                    panic!("Only DEC in data")
                }

                // HANDLE THE NUMBERS CORRECTLY TODO

                let number = line.next().expect("No number");
                
                contents[file_index].data.push(number)
            }

            if current_section == Section::Code {
                // For labels
                if first.ends_with(":") {
                    let name = &first[..first.len()-1];

                    if contents[file_index].external.contains(&name.into()) {
                        continue;
                    }

                    let label = UndefinedLabel::new(name, Section::Code, contents[file_index].code.len() as u16);

                    if contents[file_index].labels.contains(&label) {
                        panic!("Duplicated label")
                    }

                    contents[file_index].labels.push(label);
                    continue;
                }
                
                if EXTENDED.contains(&first) {
                    if !next_extended {
                        panic!("Extended instruction not preceded by EXTEND");
                    }
                }

                if IMPLIED.contains(&first) {
                    next_extended = false;

                    if first == "EXTEND" {
                        next_extended = true;
                    }

                    let instruction = Instruction::new(
                        first, 
                    UndefinedSymbol::new("ACC", Some(SymbolType::Variable)));

                    contents[file_index].code.push(instruction);
                } else {
                    if first != "INDEX" {
                        next_extended = false;
                    }

                    let operation = first;
                    let operand = line.next().expect("No operand");

                    let operand = 
                    {
                        if GENERAL.contains(&operation) {
                            UndefinedSymbol::new(operand, None)
                        } else if ERASABLE.contains(&operation) {
                            UndefinedSymbol::new(operand, Some(SymbolType::Variable))
                        } else if FIXED.contains(&operation) {
                            UndefinedSymbol::new(operand, Some(SymbolType::Label))
                        } else {
                            panic!("Invalid instruction")
                        }
                    };

                    let instruction = Instruction::new(operation, operand);
                    contents[file_index].code.push(instruction);
                }
            }

        }
    }
    
    let start_of_fixed = 2048; 
    let mut erasable = 271; // Start of RAM

    let mut defined: Vec<DefinedSymbol> = vec![];
    let mut binary: Vec<u16> = vec![];

    for file_index in 0..contents.len() {
        for label in &contents[file_index].labels {
            let mut section_offset = start_of_fixed;

            if file_index > 1 && label.section == Section::Code {
                section_offset += contents[file_index-1].code.len();
            }
            if label.section == Section::Data {
                section_offset += contents[file_index].code.len();

                if file_index > 1 {
                    section_offset += contents[file_index-1].code.len();
                    section_offset += contents[file_index-1].data.len();
                }
            }

            defined.push(label.define(section_offset as u16));
        }

        for instruction in &contents[file_index].code {
            let mut assembled: u16 = 0; // Decoding of the instruction should be done here
            
            let op_defined = instruction.operand.define(erasable);

            if defined.contains(&op_defined) {
                assembled += defined.iter().find(|&e| e == &op_defined).unwrap().value
            } else if op_defined.r#type == SymbolType::Label {
                panic!("Label never defined")
            } else {
                assembled += erasable;
                erasable += 1;
                defined.push(op_defined);
            }

            binary.push(assembled);
            println!("{:?} {:?}", 
            instruction, assembled);
        }

        
    }
}