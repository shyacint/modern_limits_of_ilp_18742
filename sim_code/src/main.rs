use csv::Writer;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
#[derive(Clone)]
struct InstructionRaw {
    inst: String,
    arguments: Vec<String>,
    mem_addr: Option<String>, // in case of a memory instruction
}

#[derive(Debug)]
#[derive(Clone)]
struct Instruction {
    reg_read_dep: Vec<String>,
    reg_write_dep: Vec<String>,
    mem_store: bool,
    mem_load: bool,
    mem_addr: Option<String>,
    key: i32,
    shortcut_dep: i32, // write the key of the blocking instruction
}

fn parse_offset(input: &str) -> (String, String) {
    if let Some(i) = input.find('(') {
        (input[..i].to_string(), input[i+1..].trim_end_matches(')').to_string())
    } else {
        panic!("Should have an offset")
    }
}

fn parse(f: &str) -> io::Result<Vec<InstructionRaw>> {
    // read the file into lines
    let file = File::open(f)?;
    let lines = io::BufReader::new(file);

    // set raw instruction buffer
    let mut raw_instructions: Vec<InstructionRaw> = vec![];

    for line in lines.lines() { // iterate through lines of file

        // split each line by whitespace
        let line_s = String::from(line?);
        let words: Vec<&str> = line_s.split_whitespace().collect();

        // safety check length
        let len = words.len();

        if len > 0 {
            if words[0].starts_with("0x") {
                // this is an instruction line -> save instruction, register dependencies, (empty) corresponding memory dependencies, and optional memory comment
                let inst = if len > 2 {words[2].split(".").map(|s| s.to_string()).collect()} else {vec![]};
                let arguments = if len > 3 {words[3].split(",").map(|s| s.to_string()).collect()} else {vec![]};

                raw_instructions.push(InstructionRaw{inst: inst[0].clone(), arguments, mem_addr: Some("TODO".to_string())})      
            }
        }

    }

    Ok(raw_instructions)
}

fn translate(raw_inst_list: Vec<InstructionRaw>) -> io::Result<Vec<Instruction>> {
    // set buffer for translated instruction list
    let mut inst_list = vec![];

    // index
    let mut i = 0;

    for inst in raw_inst_list {
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
            "addi" | "mv" | "andi" | "slli" | "srli" | "neg" | "addiw" | "slliw" | "xori" | "sext" | "sraiw" | "srliw" | "snez" | "not" | "ori" | "srai" | "negw" | "seqz" | "sgtz" | "fcvt" | "fmv" => {
                reg_write_dep.push(inst.arguments[0].clone());
                reg_read_dep.push(inst.arguments[1].clone());
            }, 
            "ld" | "lw" | "lbu" | "lhu" | "lwu" | "lb" | "fld" | "lr" | "lh" | "flw" => {
                reg_write_dep.push(inst.arguments[0].clone());
                let (_, reg) = parse_offset(&inst.arguments[1]);
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
                let(_, reg) = parse_offset(&inst.arguments[1]);
                reg_read_dep.push(reg.clone());
                mem_store = true;
            },
            "remw" | "sra" | "divw" | "divuw" | "rem" | "srlw" | "add" | "mul" | "sub" | "and" | "divu" | "addw" | "xor" | "remu" | "or" | "sllw" | "mulhu" | "srl" | "sltu" | "subw" | "remuw" | "mulw" | "div" | "slt" | "sll" | "sraw" | "fle" | "fmul" | "fdiv" | "flt" | "fadd" | "fsub" => {
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
            "lui" => {
                reg_write_dep.push(inst.arguments[0].clone());
            },
            "j" => {
                reg_write_dep.push("pc".to_string());
            }, 
            "amoswap" | "sc" | "amoadd" | "amomaxu" => {
                reg_write_dep.push(inst.arguments[0].clone());
                reg_read_dep.push(inst.arguments[1].clone());
                let (_, reg) = parse_offset(&inst.arguments[2]);
                reg_read_dep.push(reg.clone());
                mem_store = true;
            },
            "ret" | "ecall" | "fence" | "nop" => (),
            _ => panic!("Instruction {} not implemented!", inst.inst.as_str()),
        }

        inst_list.push(Instruction{reg_read_dep, reg_write_dep, mem_store, mem_load, mem_addr: inst.mem_addr, key: i, shortcut_dep: -1});
        i = i + 1;
    }

    Ok(inst_list)
}

fn simulate(inst_list: &Vec<Instruction>, width: &usize, register_renaming: bool, memory_renaming: bool) -> io::Result<(usize, usize)> {
    // intialize counting logic
    let total_instructions  = inst_list.len();
    let mut total_executed = 0;
    let mut total_fetched = 0;
    let mut num_cycles: usize = 0;

    // intialize state logic
    let mut fetch_prev: Vec<Instruction> = vec![];
    let mut decode_prev: Vec<Instruction> = vec![];
    let mut execute_prev: Vec<Instruction> = vec![];

    while total_executed < total_instructions {
        num_cycles += 1;

        // first, fetch all instructions which can be fetched
        let capacity = width - (fetch_prev.len() + decode_prev.len() + execute_prev.len()); // capacity is width - current num of instructions in window
        let mut fetch_now: Vec<Instruction> = vec![];
        for i in total_fetched..(total_fetched + capacity) {
            if i >= total_instructions {
                break;
            }
            fetch_now.push(inst_list[i].clone());
        }
        total_fetched += capacity; // update number of fetched

        // then, move all previous fetch instruction to decode
        let decode_now = fetch_prev.clone();

        // execute all instructions which can be executed (those in the previous exeucte or decode stage)
        let mut execute_now: Vec<Instruction> = execute_prev.clone();
        execute_now.extend(decode_prev.clone());

        let len = execute_now.len();
        // determine if CAN execute
        let mut decide_execute = vec![true; len];
        for i in 0..len {
            let inst1 = &execute_now[i]; // instruction being examined 

            if inst1.shortcut_dep >= 0 {
                if (&execute_now[0..i]).iter().any(|prev| prev.key == inst1.shortcut_dep) {
                    decide_execute[i] = false;
                    continue; // if the the instruction is matched to another still remaining in execution, no need to check
                }
            }

            for j in 0..i { // iterate through earlier instruction in the execute stage
                let inst2 = &execute_now[j];

                // check for memory dependencies
                if inst1.mem_store {
                    if inst2.mem_store | inst2.mem_load {
                        decide_execute[i] = false;
                        execute_now[i].shortcut_dep = execute_now[j].key;
                        break;
                    }
                } else if inst1.mem_load {
                    if inst2.mem_store {
                        if memory_renaming {
                            if inst1.mem_addr == inst2.mem_addr {
                                decide_execute[i] = false;
                                execute_now[i].shortcut_dep = execute_now[j].key;
                                break;
                            }
                        } else {
                            decide_execute[i] = false;
                            execute_now[i].shortcut_dep = execute_now[j].key;
                            break;
                        }

                    }
                }

                // check for register dependencies
                if inst1.reg_read_dep.iter().any(|x| inst2.reg_write_dep.iter().any(|y| x == y)) {
                    decide_execute[i] = false;
                    execute_now[i].shortcut_dep = execute_now[j].key;
                    break; // this checks a RAW dependency
                }
                if !register_renaming {
                    // if not renaming, check other depedencies
                    let war = inst1.reg_write_dep.iter().any(|x| inst2.reg_read_dep.iter().any(|y| x == y));
                    let waw = inst1.reg_write_dep.iter().any(|x| inst2.reg_write_dep.iter().any(|y| x == y));
                    if war | waw {
                        decide_execute[i] = false;
                        execute_now[i].shortcut_dep = execute_now[j].key;
                        break;
                    }
                }
            }

        }

        // execute_prev is all the instructions which cannot yet be executed
        execute_prev = execute_now.iter().zip(decide_execute.iter()).filter_map(|(inst,&deci)| if !deci {Some(inst.clone())} else {None}).collect();
        total_executed += len - execute_prev.len();

        // transfer decode and fetch stages for the next cycle
        fetch_prev = fetch_now;
        decode_prev = decode_now;

    }

    Ok((total_instructions, num_cycles))
}

fn main() -> io::Result<()>{
    // define files & widths to parse through
    let files = vec!["spec06/403.gcc/gcc_trace.log"];
    let widths: Vec<usize> = vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];

    // iterate through trace files
    for f in files {
        // parse file for to get dependencies
        let raw_inst_list = parse(format!("/Users/fionafisher/Desktop/s25/modern_limits_of_ilp_18742/qemu_dynamic_traces/{}", f).as_str())?;
        // convert dependencies to read/write
        let inst_list = translate(raw_inst_list)?;

        // create csv file to write to
        let csv_file = File::create(format!("/Users/fionafisher/Desktop/s25/modern_limits_of_ilp_18742/simulation_results/{}.csv", f))?;
        let mut writer = Writer::from_writer(csv_file);
        writer.write_record(&["Width", "Register Renaming", "Memory Renaming", "Instructions", "Cycles", "IPC"])?;

        // iterate through width/renaming array to get IPC
        for w in &widths {
            let width = if *w > 0 {*w} else {inst_list.len()};
            for r in [true, false] {
                for m in [false] {
                    let (num_i, num_c) = simulate(&inst_list, &width, r, m)?;
                    let ipc = num_i as f64 / num_c as f64;

                    writer.write_record(&[width.to_string(), r.to_string(), m.to_string(), num_i.to_string(), num_c.to_string(), ipc.to_string()])?;
                }
            }
        }

        writer.flush()?;


    }


    Ok(())
}
