use std::io; // for returning error result

use crate::models::{InstructionRaw, Instruction};


// get register from offsett-ed formatting
fn parse_offset(input: &str) -> String {
    if let Some(i) = input.find('(') {
        input[i+1..].trim_end_matches(')').to_string()
    } else {
        panic!("Should have an offset")
    }
}

pub fn translate_list(raw_inst_list: Vec<InstructionRaw>) -> io::Result<Vec<Instruction>> {
    // set buffer for translated instruction list
    let mut inst_list = vec![];

    for (i, inst) in raw_inst_list.iter().enumerate() {
        // create empty dependency lists
        let mut reg_read_dep = vec![];
        let mut reg_write_dep = vec![];
        // create whether a memory load or store
        let mut mem_store = false;
        let mut mem_load = false;

        // fill dependency lists as necessary
        match inst.inst.as_str() {
            "jal" => {
                reg_write_dep.push(inst.arguments[0].clone());
                reg_read_dep.push("pc".to_string());
                reg_write_dep.push("pc".to_string());
            }
            "auipc" => {
                reg_write_dep.push(inst.arguments[0].clone());
                reg_read_dep.push("pc".to_string());
            }, 
            "slti" | "fabs" | "addi" | "mv" | "andi" | "slli" | "srli" | "neg" | "addiw" | "slliw" | "xori" | "sext" | "sraiw" | "srliw" | "snez" | "not" | "ori" | "srai" | "negw" | "seqz" | "sgtz" | "fcvt" | "fmv" => {
                reg_write_dep.push(inst.arguments[0].clone());
                reg_read_dep.push(inst.arguments[1].clone());
            }, 
            "ld" | "lw" | "lbu" | "lhu" | "lwu" | "lb" | "fld" | "lr" | "lh" | "flw" => {
                reg_write_dep.push(inst.arguments[0].clone());
                let reg = parse_offset(&inst.arguments[1]);
                reg_read_dep.push(reg.clone());
                mem_load = true;
            }, 
            "jalr" => {
                reg_write_dep.push(inst.arguments[0].clone());
                reg_read_dep.push(inst.arguments[1].clone());
                reg_write_dep.push("pc".to_string());
                reg_read_dep.push("pc".to_string());
            },
            "sd" | "sw" | "sb" | "sh" | "fsd" | "fsw" => {
                reg_read_dep.push(inst.arguments[0].clone());
                let reg = parse_offset(&inst.arguments[1]);
                reg_read_dep.push(reg.clone());
                mem_store = true;
            },
            "feq" | "remw" | "sra" | "divw" | "divuw" | "rem" | "srlw" | "add" | "mul" | "sub" | "and" | "divu" | "addw" | "xor" | "remu" | "or" | "sllw" | "mulhu" | "srl" | "sltu" | "subw" | "remuw" | "mulw" | "div" | "slt" | "sll" | "sraw" | "fle" | "fmul" | "fdiv" | "flt" | "fadd" | "fsub" => {
                reg_write_dep.push(inst.arguments[0].clone());
                reg_read_dep.push(inst.arguments[1].clone());
                reg_read_dep.push(inst.arguments[2].clone());
            }, 
            "bnez" | "beqz" | "jr" | "blez" | "bltz" | "bgtz" | "bgez" => {
                reg_read_dep.push(inst.arguments[0].clone());
                reg_write_dep.push("pc".to_string());
            }, 
            "bleu" | "bne" | "bgtu" | "beq" | "ble" | "bgt" => {
                reg_read_dep.push(inst.arguments[0].clone());
                reg_read_dep.push(inst.arguments[1].clone());
                reg_write_dep.push("pc".to_string());
            },
            "lui" | "frrm" | "frflags" | "fsflags" => {
                reg_write_dep.push(inst.arguments[0].clone());
            },
            "j" => {
                reg_write_dep.push("pc".to_string());
            }, 
            "amoswap" | "sc" | "amoadd" | "amomaxu" => {
                reg_write_dep.push(inst.arguments[0].clone());
                reg_read_dep.push(inst.arguments[1].clone());
                let reg = parse_offset(&inst.arguments[2]);
                reg_read_dep.push(reg.clone());
                mem_store = true;
            },
            "ret" | "ecall" | "fence" | "nop" => (),
            _ => panic!("Instruction {} not implemented!", inst.inst.as_str()),
        }

        inst_list.push(Instruction{reg_read_dep, reg_write_dep, mem_store, mem_load, mem_addr: inst.mem_addr.clone(), key: i as i32, shortcut_dep: -1});
    }

    Ok(inst_list)
}