use std::io; // for returning error result

use crate::models::{Instruction, Window};

// iterates through prev and checks for blocking dependencies with inst1
// returns bool - can execute or not, and i32 - key of blocking dependency (or -1)
fn check_dependencies(inst1: &Instruction, prev: &[Instruction], memory_renaming: bool, register_renaming: bool) -> (bool, i32) {
    
    let key = prev.iter().rev().find_map(|inst2| { // iterates through prev, returns the blocking key if we get it
         // check if a logged shortcut dependency is still in the execute stage
        if (inst1.shortcut_dep >= 0) & (inst2.key == inst1.shortcut_dep) {
            return Some(inst2.key);
        }

        // check memory dependencies
        if inst1.mem_store { // if a store, must wait for all earlier loads and stores to resolve
            if inst2.mem_store | inst2.mem_load {
                return Some(inst2.key);
            }
        } else if inst1.mem_load { // if a load, must wait for all earlier stores OR actual blocking earlier store (if renaming)
            if inst2.mem_store {
                if memory_renaming {
                    if let Some(addr1) = &inst1.mem_addr { // can only check memory renaming if the memory address has been logged
                        if let Some(addr2) = &inst2.mem_addr {
                            if addr1 == addr2 {
                                return Some(inst2.key);
                            }
                        } else {
                            Some(inst2.key);
                        }
                    } else {
                        Some(inst2.key);
                    }
                } else {
                    return Some(inst2.key);
                }
             }
        }

        // check for register dependencies
        if inst1.reg_read_dep.iter().any(|x| inst2.reg_write_dep.iter().any(|y| x == y)) { // checks for RAW hazard
            return Some(inst2.key);
        }
        if !register_renaming {
            // if NOT register renaming, check for WAR and WAW depedencies
            let war = inst1.reg_write_dep.iter().any(|x| inst2.reg_read_dep.iter().any(|y| x == y));
            let waw = inst1.reg_write_dep.iter().any(|x| inst2.reg_write_dep.iter().any(|y| x == y));
            if war | waw {
                return Some(inst2.key);
            }
        }

        None
    });

    if let Some(k) = key { // if a dependency was logged, return false and the key
        return (false, k);
    } else { // else return true, and -1
        return (true, -1);
    }

}


pub fn simulate_list(inst_list: &Vec<Instruction>, width: &usize, register_renaming: bool, memory_renaming: bool) -> io::Result<(usize, usize)> {
    // intialize counting logic
    let total_instructions  = inst_list.len();
    let mut total_executed = 0;
    let mut total_fetched = 0;
    let mut num_cycles: usize = 0;

    // intialize state logic
    let mut window = Window{fetch: vec![], decode: vec![], execute: vec![]};

    while total_executed < total_instructions {
        num_cycles += 1;

        // first, fetch all instructions which can be fetched
        let capacity = width - (window.fetch.len() + window.decode.len() + window.execute.len()); // capacity is width - current num of instructions in window
        let mut fetch_now: Vec<Instruction> = vec![];
        for i in total_fetched..(total_fetched + capacity) {
            if i >= total_instructions {
                break;
            }
            fetch_now.push(inst_list[i].clone());
        }
        total_fetched += capacity; // update number of fetched

        // then, move all previous fetch instruction to decode
        let decode_now = window.fetch;

        // execute all instructions which can be executed (those in the previous exeucte or decode stage)
        let mut execute_now: Vec<Instruction> = [window.execute, window.decode].concat();

        let len = execute_now.len();
        // determine if CAN execute

        let decide_execute: Vec<(bool, i32)> = execute_now.iter().enumerate().map(|(i, inst)| {
            check_dependencies(inst, &execute_now[..i], memory_renaming, register_renaming)
        }).collect();


        // execute_prev is all the instructions which cannot yet be executed
        window.execute = vec![];
        let mut removed = 0;
        for (i, (decide, key)) in decide_execute.iter().enumerate() {
            if !decide {
                execute_now[i - removed].shortcut_dep = *key;
                window.execute.push(execute_now.remove(i - removed));
                removed += 1;
            }
        }

        total_executed += len - window.execute.len();

        // transfer decode and fetch stages for the next cycle
        window.fetch = fetch_now;
        window.decode = decode_now;

    }

    Ok((total_instructions, num_cycles))
}