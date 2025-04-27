use std::io;

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

    print!("\nStores: {}", store_inst);
    print!("\nLoads: {}", load_inst);
    print!("\nInstructions: {}", all_inst);


    for m in [true, false] {
        for r in [true, false] {
            let (i, c) = simulate::simulate_list(&inst_list, &32, r, m)?;
            print!("\n{}, {}\n", i, c);
        }
    }

    Ok(())
}
