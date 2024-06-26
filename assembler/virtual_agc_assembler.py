SOURCE = "../programs/source.agc"
TARGET = "../emulator/src/Memory/fixed_memory_initialization.rs"

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

labels = {
    
}

variables = {
    "ACC": 0,
    "L": 1,
    "Q": 2,
    "Z": 5,
    "ZERO": 7,
}

lines = []
with open(SOURCE, "r") as file:
    for line in file:
        lines.append(line.replace("\n", "").replace("\t", " ").strip(" ").split(' '))

binary = "use crate::Memory::FixedMemory; use crate::Memory::Memloc; impl FixedMemory {pub const fn new() -> Self {Self {fixed_bank0: ["

ins_added = 0
entry_point = 2048

# Look for and define all labels
for line in lines:
    first = line[0]
    # Ignore blank lines
    if first == "":
        continue

    # Interpret it as a label
    if first[-1] == ":":
        label:str = first[:-1] # Remove the :

        if not label.isalnum():
            raise SystemExit("Invalid label", f"line: {lines.index(line) + 1}")
        
        labels[label] = entry_point + ins_added
        continue
    
    # If not a valid instruction
    if not first in instructions_erasable and \
       not first in instructions_fixed and \
       not first in instructions_general and \
       not first in instructions_implied_and_named and \
       not first == "DEC":
        print("SKIP", first)
        continue

    ins_added += 1


ins_added = 0
erasable = 49

# Actually assemble the program
for line in lines:
    ins_assembled = 0
    ins = line[0]

    # Ignore blank lines
    if ins == "":
        continue

    # Ignore labels
    if ins[-1] == ":":
        continue

    try:
        operand = line[1]
    except LookupError:
        operand = ""

    if ins in instructions_implied_and_named:
        ins_assembled = instructions_implied_and_named[ins]
        print("IMPLIED", ins)

        ins_added += 1
        binary += f"Memloc::new({ins_assembled}), "
    
    if ins in instructions_erasable:
        ins_assembled += instructions_erasable[ins]
        print("ERASABLE", ins, end=" ")

        if operand == "":
            raise SystemExit("Operand missing", f"line: {lines.index(line) + 1}")
        
        if operand in variables:
            ins_assembled += variables[operand]
            print("SEEN", operand, ins_assembled)
        else:
            # Interpret as a number
            if operand.isdecimal():
                ins_assembled += int(operand)
                print("DECIMAL", operand)

            elif not operand.isalnum():
                raise SystemExit("Invalid variable name", f"line: {lines.index(line) + 1}", operand)
            # Create new variable
            else:
                ins_assembled += erasable
                variables[operand] = erasable
                erasable += 1
                print("NEW", operand)

        ins_added += 1
        binary += f"Memloc::new({ins_assembled}), "
    
    if ins in instructions_fixed:
        ins_assembled += instructions_fixed[ins]
        print("FIXED", ins, end=" ")

        if operand == "":
            raise SystemExit("Operand missing", f"line: {lines.index(line) + 1}")
        
        if operand in labels:
            ins_assembled += labels[operand]
            print(operand)
        else:
            # Interpret as a number
            if operand.isdecimal():
                ins_assembled += int(operand)
                print("DECIMAL", operand)
            else:
                raise SystemExit("Invalid label", f"line: {lines.index(line) + 1}", operand)
            
        ins_added += 1
        binary += f"Memloc::new({ins_assembled}), "
    
    if ins in instructions_general:
        ins_assembled += instructions_general[ins]
        print("GENERAL", ins, end=" ")

        if operand == "":
            raise SystemExit("Operand missing", f"line: {lines.index(line) + 1}")
        
        if operand in variables:
            ins_assembled += variables[operand]
            print("SEEN", operand)
        elif operand in labels:
            ins_assembled += labels[operand]
            print(operand)

        else:
            # Interpret as a number
            if operand.isdecimal():
                ins_assembled += int(operand)
                print("DECIMAL", operand)

            elif not operand.isalnum():
                raise SystemExit("Invalid variable name", f"line: {lines.index(line) + 1}", operand)
            # Create new variable
            else:
                ins_assembled += erasable
                variables[operand] = erasable
                erasable += 1
                print("NEW", operand)

        ins_added += 1
        binary += f"Memloc::new({ins_assembled}), "

    # This is not an instruction per se, it is used to give a certain value to a location in fixed memory
    if ins == "DEC":
        if operand == "":
            raise SystemExit("Operand missing", f"line: {lines.index(line) + 1}")

        if not operand.isdecimal():
            raise SystemExit("Invalid decimal", f"line: {lines.index(line) + 1}", operand)
        
        if int(operand) < 2**15:
            ins_assembled += int(operand)
        else:
            raise SystemExit("Invalid decimal", f"line: {lines.index(line) + 1}", operand)

        print("DEC")
        ins_added += 1
        binary += f"Memloc::new({ins_assembled}), "
    
    

# Fill the remaining of memory with 0s
for x in range(1024 - ins_added):
    binary += 'Memloc::new(0), '
binary += "]}}}"

# Write to file
with open(TARGET, "w") as file:
    file.write(binary)
print("DONE")