SOURCES = [
    "programs/threshold.agc",
    "programs/end_loop.agc"
]
TARGET = "emulator_rppico/src/Memory/assembler_output.rs"

instructions_general = {
    "CA":     0b011000000000000,
    "INDEX":  0b101000000000000,
    "TC":     0b000000000000000,
    "CS":     0b100000000000000,
    "AD":     0b110000000000000,
    "MASK":   0b111000000000000,
    "DCA":    0b011000000000001,
    "DCS":    0b100000000000001,
    "MP":     0b111000000000000,
}

instructions_erasable = {
    "CCS":    0b001000000000000,
    "TS":     0b101100000000000,
    "DIM":    0b010110000000000,
    "ADS":    0b010110000000000,
    "AUG":    0b010100000000000,
    "DAS":    0b010000000000001,
    "DV":     0b001000000000000,
    "DXCH":   0b101010000000001,
    "INCR":   0b010100000000000,
    "LXCH":   0b010010000000000,
    "MSU":    0b010000000000000,
    "QXCH":   0b010010000000000,
    "SU":     0b110000000000000,
    "XCH":    0b101110000000000,
}

instructions_fixed = {
    "TCF":    0b001000000000000,
    "BZF":    0b001000000000000,
    "BZMF":   0b110000000000000,
}

instructions_implied_and_named = {
    "EXTEND": 6,
    "INHINT": 4,
    "RELINT": 3,
    "RETURN": 2,

    "COM":    0b100000000000000,
    "DCOM":   0b100000000000001,
    "DDOUBL": 0b001000000000001,
    "DOUBLE": 0b011000000000000,
    "DTCB":   0b010101000000110,
    "DTCF":   0b010101000000101,
    "OVSK":   0b010110000000000,
    "SQUARE": 0b011100000000000,
    "ZL":     0b001001000000111,
    "ZQ":     0b001001000000111,
}

instructions_extended = [
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
]

all_instructions = []
all_instructions.extend(instructions_general.keys())
all_instructions.extend(instructions_fixed.keys())
all_instructions.extend(instructions_erasable.keys())

instructions_with_operand = ["DEC"]
instructions_with_operand.extend(all_instructions)

all_instructions.extend(instructions_implied_and_named.keys())


labels = {
    
}

builtin_addresses = {
    "ACC": 0,
    "L": 1,
    "Q": 2,
    "Z": 5,
    "BB": 6,
    "ZERO": 7,
    "SCREEN": 48,
}

vectors = {
}

def is_valid_name(name:str):
    return not ( not name.isalnum() or name[0].isdecimal() )

def error(err:str, line:list, index:int):
    raise SystemExit(f"{err}, in line {index+1}: {' '.join(line)}")

# HOW THE ASSEMBLER WORKS
# Parses all files
#   Separate in sections
#   Parse instructions and labels inside of each section
#   .data must only consist of DEC instructions
# Parse order file
# Start assembling
#   Ensure no duplicated labels
#   Ensure no duplicated variables (to share variables one file doesn't declare them while another file uses EXTERN \VARIABLE\)
#   Data sections are always placed at the end of the program, one after another in the order of parsing of the files

sections = []
section_instructions = {}
file_symbols = []

# Parse all files
for path_index in range(len(SOURCES)):
    path = SOURCES[path_index]
    file_symbols.append({"labels": set({}), "operands": set({})})
    # Load file
    file = []
    with open(path, "r") as f:
        for line in f:
            # All of this ensures that we get a list of the relevant words only,
            # removing any whitespace
            line = line.replace("\n", "").replace("\t", " ").strip().split(' ')
            file.append([word for word in line if word != ""])


    current_section = None
    next_extended = False
    # Parse file
    for i in range(len(file)):
        line = file[i];

        # Ignore blank lines
        if line == [''] or len(line) == 0:
            continue
        
        first = line[0];
        
        # Ignore comments
        if first[0] == '#':
            continue
            
        # Data section requires special handling
        if current_section == "data":
            pass
            # Special handling TODO
        
        # Handle sections
        if first[0] == '.':
            name = first[1:] # Remove the .
            
            if not is_valid_name(name):
                error("Invalid section name", line, i)

            # Ensure no duplicated sections (except data)
            if name in sections:
                error("Duplicated section", line, i)

            # Create new section
            if name != "data":
                sections.append((name, path_index)) 
                section_instructions[name] = []

            current_section = name 
            continue
        
        # Add label to section. Data stored are label name and offset with respect to the start of the section
        if first[-1] == ":":
            if current_section == None:
                error("Every piece of code should be part of a section", line, i)
            name = first[:-1]

            if not is_valid_name(name):
                error("Invalid label name", line, i)

            if name in [label[0] for label in file_symbols[path_index]["labels"]]:
                error("Duplicated label name", line, i)
            
            file_symbols[path_index]["labels"].add((name, current_section, len(section_instructions[current_section])))
            continue

        # Handle instructions with operand
        if first in instructions_with_operand: 
            if current_section == None:
                error("Every piece of code should be part of a section", line, i)
            name = first

            # Ensure DEC is only used in the data section
            if name == "DEC":
                if current_section != "data":
                    error("DEC should only be used in the data section", line, i)

            # Ensure extended instructions are preceded by an EXTEND
            if name in instructions_extended:
                if not next_extended:
                    error("Extended instructions not preceded by extend", line, i)

            # Reset the extend flag except in the case of an INDEX instruction
            if not name == "INDEX":
                next_extended = False

            if len(line) < 2:
                error("No operand found", line, i)
            operand = line[1]

            # Check if the operand is valid
            if name == "DEC":
                num = operand
                if operand[0] == "-":
                    num = operand[1:]

                if not num.isdecimal():
                    error("Operand is not a valid number", line, i)
                
                if int(num) >= 2**14:
                    error("Operand number is too big", line, i)
            else:
                if not is_valid_name(operand):
                    error("Operand name is invalid", line, i)

                # Ensure the operand is not a built-in address nor is it defined in an external file TODO
                if operand not in builtin_addresses.keys():
                    file_symbols[path_index]["operands"].add(operand)

            if len(line) > 2:
                # Ensure comments start with #
                if line[2][0] != "#":
                    error("Comments should start with #", line, i)

            section_instructions[current_section].append((name, operand))
            continue
        
        if first in instructions_implied_and_named.keys():
            if current_section == None:
                error("Every piece of code should be part of a section", line, i)
            name = first

            next_extended = False
            if name == "EXTEND":
                next_extended = True

            # Ensure comments start with #
            if len(line) > 1:
                if line[1][0] != "#": 
                    error("Comments should start with #", line, i)

            # In operations without operand we set it to the acc, meaning address 0, so the instruction isn't changed
            section_instructions[current_section].append((name, "ACC"))
            continue
        
        error("Invalid instruction", line, i)

print(sections, section_instructions, file_symbols)



        
        

# binary = """use crate::Memory::FixedMemory; 
# use crate::Memory::Memory; 
# use crate::Memory::Memloc;
# use crate::Memory::Address; 
# impl FixedMemory {
#     pub const fn new() -> Self {
#         Self {fixed_bank0: ["""

# ins_added = 0
# entry_point = 2048
# erasable = 56

# # Look for and define all labels
# for line in lines:
#     first = line[0]
#     # Ignore blank lines and comments
#     if first == "" or first[0] == '#':
#         continue

#     # Interpret it as a label
#     if first[-1] == ":":
#         label:str = first[:-1] # Remove the :

#         if not label.isalnum():
#             raise SystemExit("Invalid label", f"line: {lines.index(line) + 1}")
        
#         labels[label] = entry_point + ins_added
#         continue

#     if first == "VEC":
#         try:
#             operand = line[1]
#         except LookupError:
#             raise SystemExit("Name missing", f"line: {lines.index(line) + 1}")
        
#         if not operand.isalnum() or operand.isnumeric():
#                 raise SystemExit("Invalid vector name", f"line: {lines.index(line) + 1}", operand)
#         name = operand

#         dimensions = []
#         for i in range(3):
#             try:
#                 dimensions.append(line[i + 2])
#             except LookupError:
#                 break

#         if len(dimensions) == 0:
#             raise SystemExit("Size missing", f"line: {lines.index(line) + 1}")
        
#         size = 1
#         for dimension in dimensions:
#             if not dimension.isdecimal():
#                 raise SystemExit("Invalid size", f"line: {lines.index(line) + 1}")
#             size *= int(dimension)

#         vectors[name] = (size, erasable)
#         variables[name] = erasable
#         erasable += size;
    
#     # If not a valid instruction
#     if not first in instructions_erasable and \
#        not first in instructions_fixed and \
#        not first in instructions_general and \
#        not first in instructions_implied_and_named and \
#        not first == "DEC":
#         continue

#     ins_added += 1


# ins_added = 0
# next_extended = False
# # Actually assemble the program
# for line in lines:
#     ins_assembled = -1
#     ins = line[0]

#     # Ignore blank lines and comments
#     if ins == "" or ins[0] == '#' or ins == "VEC":
#         continue

#     # Ignore labels
#     if ins[-1] == ":":
#         continue

#     try:
#         operand = line[1]
#     except LookupError:
#         operand = ""

#     # Force commments to start with #
#     if len(line) > 2 and ins != "VEC":
#         if line[2][0] != "#":
#             raise SystemExit("Comments should start with #", f"line: {lines.index(line) + 1}")
    
#     # Ensure extended instructions are preceded by an EXTEND
#     if ins in instructions_extended and not next_extended:
#         raise SystemExit("This instruction should be preceded by an EXTEND", f"line: {lines.index(line) + 1}", ins)

#     if ins in instructions_erasable:
#         ins_assembled = instructions_erasable[ins]

#         if operand == "":
#             raise SystemExit("Operand missing", f"line: {lines.index(line) + 1}")
        
#         if operand in variables:
#             ins_assembled += variables[operand]
#         else:
#             # Interpret as a number
#             if operand.isdecimal():
#                 ins_assembled += int(operand)

#             elif not operand.isalnum():
#                 raise SystemExit("Invalid variable name", f"line: {lines.index(line) + 1}", operand)
#             # Create new variable
#             else:
#                 print(f"NEW {operand} {erasable}")
#                 ins_assembled += erasable
#                 variables[operand] = erasable
#                 erasable += 1
    
#     if ins in instructions_fixed:
#         ins_assembled = instructions_fixed[ins]

#         if operand == "":
#             raise SystemExit("Operand missing", f"line: {lines.index(line) + 1}")
        
#         if operand in labels:
#             ins_assembled += labels[operand]
#         else:
#             # Interpret as a number
#             if operand.isdecimal():
#                 ins_assembled += int(operand)
#             else:
#                 raise SystemExit("Invalid label", f"line: {lines.index(line) + 1}", operand)
    
#     if ins in instructions_general:
#         ins_assembled = instructions_general[ins]

#         if operand == "":
#             raise SystemExit("Operand missing", f"line: {lines.index(line) + 1}")
        
#         if operand in variables:
#             ins_assembled += variables[operand]
#         elif operand in labels:
#             if ins == "INDEX" and not next_extended:
#                 raise SystemExit("This instruction should be preceded by an EXTEND", f"line: {lines.index(line) + 1}", ins)
#             ins_assembled += labels[operand]

#         else:
#             # Interpret as a number
#             if operand.isdecimal():
#                 if ins == "INDEX" and int(operand) >= 2048 and not next_extended:
#                     raise SystemExit("This instruction should be preceded by an EXTEND", f"line: {lines.index(line) + 1}", ins)
#                 ins_assembled += int(operand)

#             elif not operand.isalnum():
#                 raise SystemExit("Invalid variable name", f"line: {lines.index(line) + 1}", operand)
#             # Create new variable
#             else:
#                 ins_assembled += erasable
#                 variables[operand] = erasable
#                 erasable += 1

#     # This is not an instruction per se, it is used to give a certain value to a location in fixed memory
#     if ins == "DEC":
#         if operand == "":
#             raise SystemExit("Operand missing", f"line: {lines.index(line) + 1}")     

#         if operand[0] == "-":
#             num = operand[1::]
#             if not num.isdecimal():
#                 raise SystemExit("Invalid decimal", f"line: {lines.index(line) + 1}", operand)
#             num = int(num)

#             if num >= 2**14:
#                 raise SystemExit("The number given is too big", f"line: {lines.index(line) + 1}", num)
            
#             ins_assembled = 2**15 - 1 - num
#         else:
#             if not operand.isdecimal():
#                 raise SystemExit("Invalid decimal", f"line: {lines.index(line) + 1}", operand)
            
#             if int(operand) >= 2**15:
#                 raise SystemExit("The number given is too big", f"line: {lines.index(line) + 1}", operand)
#             ins_assembled = int(operand)
     
#     next_extended = False
#     if ins in instructions_implied_and_named:
#         if ins == "EXTEND":
#             next_extended = True
#         ins_assembled = instructions_implied_and_named[ins]
    
#     # If ins_assmebled hasn't been modified, throw an error
#     if ins_assembled == -1:
#         raise SystemExit("Invalid instruction", f"line: {lines.index(line) + 1}", ins)
        
#     ins_added += 1
#     binary += f"Memloc::new({ins_assembled}), "
    

# # Fill the remaining of memory with 0s
# for x in range(1024 - ins_added):
#     binary += 'Memloc::new(0), '
# binary += "]}}}"

# # Add the name of the addresses for debugging purposes
# binary += """
# impl Memory { 
#     pub fn get_address_name(&self, addr: Address) -> &'static str {
#         #[allow(unreachable_patterns)]
#         match addr {"""
# for label in labels:
#     binary += f"\n\t\t\t{labels[label]} => \"{label}\","

# for name in variables:
#     binary += f"\n\t\t\t{variables[name]} => \"{name}\","

# for name in vectors:
#     binary += f"\n\t\t\t{vectors[name][1]}..={vectors[name][1] + vectors[name][0]} => \"{name}\","

# binary += """\n\t\t\t_ => "",
# \t\t}
# \t}
# }"""
# # Write to file
# with open(TARGET, "w") as file:
#     file.write(binary)
# print("DONE")