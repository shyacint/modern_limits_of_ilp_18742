use std::io;
use std::fs::File;
use csv::Writer;

mod parse;
mod translate;
mod simulate;
mod models;

fn main() -> io::Result<()>{

    let raw_inst_list = parse::parse_file("/Volumes/KINGSTON/Coursework/modern_limits_of_ilp_18742/sim_traces/SPEC06/omnetpp_trace.log")?;
    let inst_list = translate::translate_list(raw_inst_list)?;

    let all_inst = inst_list.len();
    let store_inst = inst_list.iter().filter(|&i| i.mem_store).count();
    let load_inst = inst_list.iter().filter(|&i| i.mem_load).count();

    let mut profile_writer = Writer::from_writer(File::create("/Volumes/KINGSTON/Coursework/modern_limits_of_ilp_18742/sim_results/exp1/profile.csv")?);
    profile_writer.write_record(&["Instructions", "Stores", "Loads"])?;
    profile_writer.write_record(&[all_inst.to_string(), store_inst.to_string(), load_inst.to_string()])?;
    profile_writer.flush()?;

    let mut sim_writer = Writer::from_writer(File::create("/Volumes/KINGSTON/Coursework/modern_limits_of_ilp_18742/sim_results/exp1/sim.csv")?);
    sim_writer.write_record(&["Width", "Reg Renaming", "Mem Renaming", "Instructions", "Cycles"])?;


    for w in [1,2,4,8,16,32,64,128,256,512,1024] {
        for m in [true, false] {
            for r in [true, false] {
                let (i, c) = simulate::simulate_list(&inst_list, &w, r, m)?;
                sim_writer.write_record(&[w.to_string(), r.to_string(), m.to_string(), i.to_string(), c.to_string()])?;
                sim_writer.flush()?;
            }
        }
    }

    Ok(())
}
