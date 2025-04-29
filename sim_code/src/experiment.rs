use std::io;
use std::fs::File;
use csv::Writer;

use crate::{parse, translate, simulate};

pub fn run_experiment(read_file: &str, profile_file: &str, sim_file: &str) -> io::Result<()> {

    // parse file
    let raw_inst_list = parse::parse_file(read_file)?;
    let inst_list = translate::translate_list(raw_inst_list)?;

    // profile the instructions
    let all_inst = inst_list.len();
    let store_inst = inst_list.iter().filter(|&i| i.mem_store).count();
    let load_inst = inst_list.iter().filter(|&i| i.mem_load).count();
    let no_addr_inst = inst_list.iter().filter(|&i| (i.mem_load | i.mem_store) & (i.mem_addr == None)).count();

    // write the profiled instruction
    let mut profile_writer = Writer::from_writer(File::create(profile_file)?);
    profile_writer.write_record(&["Instructions", "Stores", "Loads", "No Addr"])?;
    profile_writer.write_record(&[all_inst.to_string(), store_inst.to_string(), load_inst.to_string(), no_addr_inst.to_string()])?;
    profile_writer.flush()?;

    // simulate the instruction
    let mut sim_writer = Writer::from_writer(File::create(sim_file)?);
    sim_writer.write_record(&["Width", "Reg Renaming", "Mem Renaming", "Instructions", "Cycles"])?;


    for w in [1,2,4,8,16,32,64] {
        for m in [true, false] {
            for r in [true, false] {
                let (i, c) = simulate::simulate_list(&inst_list, &w, r, m, &1, &2)?;
                sim_writer.write_record(&[w.to_string(), r.to_string(), m.to_string(), i.to_string(), c.to_string()])?;
                sim_writer.flush()?;
            }
        }
    }


    Ok(())
    
}