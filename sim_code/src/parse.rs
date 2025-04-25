use std::fs::File; // for file reading
use std::io::{self, BufRead}; // for file reading by line
use std::collections::HashMap; // for dictionary mapping

use crate::models::InstructionRaw;

pub fn parse_file(f: &str) -> io::Result<Vec<InstructionRaw>> { // return a vector of raw instructions OR an error
    // read the file into lines
    let file = File::open(f)?;
    let lines = io::BufReader::new(file);

    // set raw instruction buffer for execution
    let mut raw_instructions: Vec<InstructionRaw> = vec![]; 
    // set dictionary mapping PC to instruction
    let mut pc_mapping: HashMap<String, InstructionRaw> = HashMap::new();

    for line in lines.lines() { // iterate through lines of file

        if raw_instructions.len() > 10 {
            break;
        }

        // split each line by whitespace
        let line_s = String::from(line?);
        let words: Vec<&str> = line_s.split_whitespace().collect();

        // safety check length
        let len = words.len();

        if len > 0 { // if empty, no use in looking
            if words[0].starts_with("0x") { // if a PC mapping to an instruction, save that in the dictionary
                let pc = words[0][2..10].to_string();
                let inst = if len > 2 {words[2].split(".").map(|s| s.to_string()).collect()} else {vec![]};
                let arguments = if len > 3 {words[3].split(",").map(|s| s.to_string()).collect()} else {vec![]};

                pc_mapping.insert(pc, InstructionRaw{inst: inst[0].clone(), arguments, mem_addr: None});  
            } else if words[0] == "Trace" { // if this indicates an execution of an instruction, find that instruction and add it to the instruction list
                assert!(len >= 3);
                let args: Vec<String> = words[3].split("/").map(|s| s.to_string()).collect();
                if let Some(inst) = pc_mapping.get(&args[1][8..].to_string()) {
                    raw_instructions.push(inst.clone());
                } else {
                    panic!("Encountered trace to unmapped PC.");
                }

            } else if words[0] == "MEM_OP:" { // if this indicates a memory op, add the memory address to the last pushed instruction
                if let Some(recent) = raw_instructions.last_mut() {
                   recent.mem_addr = Some(words[2].to_string());
                } else {
                    panic!("Memory op detected before instructions stored (somehow).");
                }
            }
        }

    }

    Ok(raw_instructions)
}