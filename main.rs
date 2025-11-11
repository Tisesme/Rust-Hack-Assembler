/*
Road to completion of the Hack Assembler:
1. Implement the Parser -> Reads files and parses information
    Get the current instruction:
    -> boolean hasMoreLines() -> Are there more lines to work through?
    -> string advance() -> Gets the next instruction and makes it the current instruction
    Parse the current instruction:
    -> instructionType() -> returns the instruction type as a constant A_INSTRUCTION, C_INSTRUCTION, or L_INSTRUCTION
    -> symbol() -> Returns instruction symbol @sum or (loop) -> sum or loop symbol
    -> dest() -> Returns the destination field  |
    -> comp() -> Returns the computation field  |-> For C_INSTRUCTION i.e. D=comp;jump
    -> jump() -> Returns the jump field         |
2. Implement the Code API -> Generate binary code
    A_INSTRUCTION are easy, @17 is binary 17 -> 0000_0000_0001_0001
    C_INSTRUCTION based on comp, dest, and jump fields -> 111_a_cccccc_ddd_jjj (a bit turns A into M when 1)
    -> dest(str) -> Returns binary of parsed destination field
    -> comp(str) -> Returns binary of parsed computation field
    -> jump(str) -> Returns binary of parsed jump field
3. Implement the SymbolTable -> Handles the symbols
    -> void addEntry (string symbol, int address) -> Adds <symbol, address> to the table
    -> boolean contains(string symbol) -> checks if symbol exists in the table
    -> int getAddress(string symbol) -> Returns addr associated with a symbol
*/
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::env;

fn main() -> io::Result<()> {
    //Get in the arguments from the command line
    let args: Vec<String> = env::args().collect();
    //Check to see if there are any arguments, fail if there are not
    let file = args.get(1).expect("No file path provided!");
    //Initialize the symbol table using the predef_symbols function
    let mut symbols = predef_symbols();
    //Create a path and display information for the filepath
    let path = Path::new(file);
    //Print for debugging
    println!("The file you provided is: '{}'", file);
    //Get the file stem to create a name for the output file "name.hack"
    let mut name = String::from(path.file_stem().and_then(|s| s.to_str()).expect("Couldn't get file stem of {path}"));
    println!("The file stem is: '{}'", name);
    name.push_str(".hack");

    //Open the file at the filepath, fails if there is no file at the filepath.
    let _f = File::open(file)?;
    let _f = BufReader::new(_f);

    // 1st for loop to check and add symbols
    let mut i = 0;
    for line in _f.lines() {
        let line = line?;
        //Skip comments and blank lines
        if line.trim().is_empty() | line.trim().starts_with("//") {continue}
        //If the line isn't a label, increment the line counter
        else if !line.trim().starts_with("(") {i+=1}
        //Otherwise, assume the line is a label and add the symbol
        else{
            //Put the stripped label in the variable 'sym' (SYM) -> SYM
            let sym = line.trim().strip_prefix('(').expect("Expected '(' in Label").strip_suffix(')').expect("Expected ')' in Label");
            //If the symbol is NOT in the HashMap, add it.
            if !symbols.contains_key(sym) {
                symbols.insert(sym.into(),i);
            }
            //If the symbol IS in the HashMap, panic. It shouldn't be.
            else {
                panic!("Label already used!");
            }
            //For debug, show what address each label is at.
            println!("label {sym} is at address {i}");
        }
    }
    //Open the file at the filepath, fails if there is no file at the filepath.
    let _f = File::open(file)?;
    let _f = BufReader::new(_f);

    //Create the new file for writing the binary program (file.asm becomes file.hack)
    let mut outfile = File::create(name)?;

    // 2nd for loop for processing instructions into binary and writing to the file.
    let mut i = 16;
    for line in _f.lines() {
        let line = line?;
        //If the line is a comment, blank, or a label, skip it.
        if line.trim().is_empty() | line.trim().starts_with("//") | line.trim().starts_with("(") {continue}
        //Assume we have an instruction, parse it.
        let inst = parser(&line);
        //Handle the Address instructions
        if inst.kind == "A" {
            //Check if we are addressing a number
            if let Ok(binary) = inst.dest.parse::<u16>() {
                println!("{binary:016b}");
                writeln!(outfile, "{binary:016b}").expect("Couldn't write to {name}.hack");
            }
            //Assume we are dealing with a symbol, check if it's in the symbol table
            else if let Some(binary) = symbols.get(inst.dest).copied() {
                println!("{binary:016b}");
                writeln!(outfile, "{binary:016b}").expect("Couldn't write to {name}.hack");
            }
            //Assume it is a variable, add it to the symbol table and increment the address counter
            else {
                symbols.insert(inst.dest.into(), i);
                println!("{i:016b}");
                writeln!(outfile, "{i:016b}").expect("Couldn't write to {name}.hack");
                i += 1;
            }
        }
        //Assume this is a C-Instruction
        else{
        let binary = encode(&inst);
        println!("{binary:016b}");
        writeln!(outfile, "{binary:016b}").expect("Couldn't write to {name}.hack")
        }
    }
    //Since the last line is a newline, the Nand2Tetris compare code tool says mine is too long.. this is to get rid of the newline..
    let len = outfile.metadata()?.len();
    if len > 0 {outfile.set_len(len-1)?}
    println!("ok done now");
    Ok(())
}
//End of main

struct Instruction<'a> {
    kind: &'a str,
    dest: &'a str,
    comp: &'a str,
    jump: &'a str,
}

fn parser(line: &str) -> Instruction<'_> {
    //Assume blank lines, comments, and labels are skipped
    //Match based on first character of the line
    let line = line.trim();
    match line.chars().next().unwrap_or(' ') {
        '@' => {
            //For A-Instructions: mark 'kind' of instruction, strip the prefix and store in dest (we reuse dest here for our address/symbol)
            let alpha = line.strip_prefix('@').expect("Expected '@' in A-Instruction");
            println!("This is A-Instruction:{alpha}");
            Instruction {kind: "A", dest: alpha, comp: "", jump: ""}
        }

        _=> {
            //For C-Instructions, handle Dest, Comp, and Jump fields
            //Grab 'dest' and the 'rest' of the code at '='
            let (dest, rest) = match line.split_once('=') {
                Some((d,r)) => (d,r),
                None =>("", line),
            };

            //Split the rest into comp and jump at ';'
            let (comp, jump) = match rest.split_once(';') {
                Some((c, j)) => (c, j),
                None => (rest, ""),
            };
            println!("This is C-Instruction:{}", line);
            Instruction {kind: "C", dest, comp, jump}
        }
    }
}

fn encode(inst: &Instruction) -> u16 {
    let dest:u16;
    let comp:u16;
    let jump:u16;
    //Convert dest value to binary (assign decimal value and convert binary later)
    match inst.dest {
        ""  => dest = 0,
        "M" => dest = 0b001000,
        "D" => dest = 0b010000,
        "MD"=> dest = 0b011000,
        "A" => dest = 0b100000,
        "AM"=> dest = 0b101000,
        "AD"=> dest = 0b110000,
        "ADM"=>dest = 0b111000,
        _   => panic!("Unexpected destination:'{}'", inst.dest),
    };
    //Convert comp value to binary (assign decimal value and convert binary later)
    match inst.comp {
        "0" => comp = 0b0101010000000,
        "1" => comp = 0b0111111000000,
        "-1"=> comp = 0b0111010000000,
        "D" => comp = 0b0001100000000,
        "A" => comp = 0b0110000000000,
        "!D"=> comp = 0b0001101000000,
        "!A"=> comp = 0b0110001000000,
        "-D"=> comp = 0b0001111000000,
        "-A"=> comp = 0b0110011000000,
        "D+1"=>comp = 0b0011111000000,
        "A+1"=>comp = 0b0110111000000,
        "D-1"=>comp = 0b0001110000000,
        "A-1"=>comp = 0b0110010000000,
        "D+A"=>comp = 0b0000010000000,
        "D-A"=>comp = 0b0010011000000,
        "A-D"=>comp = 0b0000111000000,
        "D&A"=>comp = 0b0000000000000,
        "D|A"=>comp = 0b0010101000000,
        "M" => comp = 0b1110000000000,
        "!M"=> comp = 0b1110001000000,
        "-M"=> comp = 0b1110011000000,
        "M+1"=>comp = 0b1110111000000,
        "M-1"=>comp = 0b1110010000000,
        "D+M"=>comp = 0b1000010000000,
        "D-M"=>comp = 0b1010011000000,
        "M-D"=>comp = 0b1000111000000,
        "D&M"=>comp = 0b1000000000000,
        "D|M"=>comp = 0b1010101000000,
        _   => panic!("Unexpected computation:'{}'", inst.comp),
    };
    //Convert jump value to binary (assign decimal value and convert binary later)
    match inst.jump  {
        ""  =>  jump = 0,
        "JGT"=> jump = 0b001,
        "JEQ"=> jump = 0b010,
        "JGE"=> jump = 0b011,
        "JLT"=> jump = 0b100,
        "JNE"=> jump = 0b101,
        "JLE"=> jump = 0b110,
        "JMP"=> jump = 0b111,
        _   =>  panic!("Unexpected jump:'{}'", inst.jump),
    };
    //Binary "OR" the jump, comp, and dest with binary 1110000000000000 (unsigned 57344) to create the C-Instruction
    let binary = 0b1110000000000000 | dest | comp | jump;
    binary
}

fn predef_symbols() -> HashMap<String, u16> {
    HashMap::from([
        ("SP".into(), 0),
        ("LCL".into(), 1),
        ("ARG".into(), 2),
        ("THIS".into(), 3),
        ("THAT".into(), 4),
        ("R0".into(), 0),
        ("R1".into(), 1),
        ("R2".into(), 2),
        ("R3".into(), 3),
        ("R4".into(), 4),
        ("R5".into(), 5),
        ("R6".into(), 6),
        ("R7".into(), 7),
        ("R8".into(), 8),
        ("R9".into(), 9),
        ("R10".into(), 10),
        ("R11".into(), 11),
        ("R12".into(), 12),
        ("R13".into(), 13),
        ("R14".into(), 14),
        ("R15".into(), 15),
        ("SCREEN".into(), 16384),
        ("KBD".into(), 24576),
    ])
}
