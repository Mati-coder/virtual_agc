use std::fs;
mod constants;
use constants::*;
mod types;
use types::*;

fn main() {
    const FILE_COUNT: usize = 4;
    
    let files = [
        //fs::read_to_string("../programs/if.agc").unwrap(),
        fs::read_to_string("../programs/blink.agc").unwrap(),
        fs::read_to_string("../programs/for.agc").unwrap(),
        fs::read_to_string("../programs/fin.agc").unwrap(),
        fs::read_to_string("../programs/if.agc").unwrap(),
    ];

    let sections = [Section::None, Section::Config, Section::Code, Section::Data];
    let mut contents: [FileContent; FILE_COUNT] = [FileContent::new(), FileContent::new(), FileContent::new(), FileContent::new()];
    let mut tables: Vec<UndefinedTable> = vec![];

    // Parse files
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
                if first == "VEC" {
                    let name = line.next().expect("No name");
                    let len = line.next().expect("No len").parse().unwrap();
                    tables.push(UndefinedTable::new(name, len));
                }
            }

            if current_section == Section::Data {
                // For labels
                if first.ends_with(":") {
                    let name = &first[..first.len()-1];

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

                let number = line.next().expect("No number");
                let num;
                if number.starts_with("-") {
                    let number = &number[1..];
                    let number: u16 = number.parse().unwrap();
                    num = (!number) % 32768;
                } else {
                    num = number.parse().unwrap();
                }

                contents[file_index].data.push(num)
            }

            if current_section == Section::Code {
                // For labels
                if first.ends_with(":") {
                    let name = &first[..first.len()-1];

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

    let mut defined: Vec<DefinedSymbol> = vec![ACC, L, Q, Z, BB, ZERO, PANT, BTNUP, BTNRGT, BTNDWN, BTNLFT, BTN1, BTN2, POTE];
    let mut binary: Vec<u16> = vec![];

    let mut len_code_total = 0;
    let mut len_code = [0; FILE_COUNT];
    let mut len_data = [0; FILE_COUNT];

    for file_index in 0..contents.len() {
        let code_len = contents[file_index].code.len();
        let data_len = contents[file_index].data.len();

        len_code_total += code_len;

        len_code[file_index] = code_len;
        len_data[file_index] = data_len;
    }

    // Define all labels
    for file_index in 0..contents.len() {
        for label in &contents[file_index].labels {
            let mut section_offset = start_of_fixed;

            // Add the len of the previous code sections
            if file_index > 0 && label.section == Section::Code {
                for i in 0..=file_index-1 {
                    section_offset += len_code[i];
                }
            }

            // Add the len of all code sections and of previous data sections
            if label.section == Section::Data {
                section_offset += len_code_total;

                if file_index > 0 {
                    for i in 0..=file_index-1 {
                        section_offset += len_data[i];
                    }
                }
            }

            let len;
            let und = &UndefinedTable::new(label.name, 0);
            if tables.contains(und) {
                len = tables.iter().find(|&e| e == und).unwrap().len;
                println!("Fixed table {} added, len {}", label.name, len);
            } else {
                len = 0;
            }

            let defined_label = label.define(section_offset as u16, len);
            if defined.contains(&defined_label) {
                panic!("Label {} defined in multiple files", defined_label.name)
            }
            defined.push(defined_label);

            println!("{:?} {:?}", defined.last().unwrap(), section_offset);
        }
    }

    // Assemble all instructions
    for file_index in 0..contents.len() {
        for instruction in &contents[file_index].code {
            let mut assembled: u16 = decode(instruction.operation);

            let len;
            let und = &UndefinedTable::new(instruction.operand.name, 0);
            if tables.contains(und) {
                len = tables.iter().find(|&e| e == und).unwrap().len;
                println!("Erasable table {} considered, len {}", instruction.operand.name, len);
            } else {
                len = 0;
            }
            let op_defined = instruction.operand.define(erasable, len);
            
            if defined.contains(&op_defined) {
                let op = defined.iter().find(|&e| e == &op_defined).unwrap();
                if ERASABLE.contains(&&instruction.operation) {
                    if op.r#type == SymbolType::Label {
                        panic!("The operand is a position in fixed memory but the operation works on erasable only, {:?}", instruction)
                    }
                    if let SymbolType::LabelTable(_) = op.r#type {
                        panic!("The operand is a position in fixed memory but the operation works on erasable only, {:?}", instruction)
                    }
                    
                }
                if FIXED.contains(&&instruction.operation) {
                    if op.r#type == SymbolType::Variable {
                        panic!("The operand is a position in erasable memory but the operation works on fixed only, {:?}", instruction)
                    }
                    if let SymbolType::VariableTable(_) = op.r#type {
                        panic!("The operand is a position in erasable memory but the operation works on fixed only, {:?}", instruction)
                    }
                }
                assembled += op.address
            } else if op_defined.r#type == SymbolType::Label {
                panic!("Label never defined, {:?}", op_defined)
            } else {
                if op_defined.r#type == SymbolType::Label {
                    panic!("The operand is a position in fixed memory but the operation works on erasable only, {:?}", instruction)
                }
                if let SymbolType::LabelTable(_) = op_defined.r#type {
                    panic!("The operand is a position in fixed memory but the operation works on erasable only, {:?}", instruction)
                }

                // Create new variable
                assembled += erasable;
                erasable += 1;
                
                defined.push(op_defined);
                
            }

            binary.push(assembled);
            println!("{:?} {:?}", instruction, assembled);
        }
    }

    // Add the data sections
    for file_index in 0..contents.len() {
        for num in &contents[file_index].data {
            binary.push(*num);
            println!("data: {}", num);
        }
    }

    //println!("{:#?}", defined);

    let mut to_file: String = "[".to_string();
    let mut bin_iter = binary.iter();

    for _ in 0..1024 {
        to_file.push_str(&format!("\nMemloc::new({}),", bin_iter.next().unwrap_or(&0)));
    }
    to_file.push_str("\n]");

    fs::write("../agc_emulator/memory/fixed.in", to_file).unwrap();


    let mut to_file: String = "match addr {".to_string();

    for symbol in defined {
        if let SymbolType::LabelTable(len) = symbol.r#type {
            for i in 0..len {
                to_file.push_str(&format!("\n\t{} => \"{}+{}\",", symbol.address + i, symbol.name, i));
            }
        }
        else if let SymbolType::VariableTable(len) = symbol.r#type {
            for i in 0..len {
                to_file.push_str(&format!("\n\t{} => \"{}+{}\",", symbol.address + i, symbol.name, i));
            }
        }
        else {
            to_file.push_str(&format!("\n\t{} => \"{}\",", symbol.address, symbol.name));
        }
        
    }
    to_file.push_str("\n\t_ => \"\",\n}");

    fs::write("../agc_emulator/memory/names.in", to_file).unwrap();
}