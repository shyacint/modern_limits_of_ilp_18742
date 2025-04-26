use std::io;

mod parse;
mod translate;
mod simulate;
mod models;

fn main() -> io::Result<()>{

    let raw_inst_list = parse::parse_file("/Users/fionafisher/Desktop/s25/modern_limits_of_ilp_18742/sim_traces/SPEC06/sjeng_trace.log")?;
    let inst_list = translate::translate_list(raw_inst_list)?;

    let all_inst = inst_list.len() as f64;
    let store_inst = inst_list.iter().filter(|&i| i.mem_store).count() as f64;
    let load_inst = inst_list.iter().filter(|&i| i.mem_load).count() as f64;

    print!("Stores: {}", store_inst);
    print!("\nLoads: {}", load_inst);
    print!("\nInstructions: {}", all_inst);

    // for r in [true, false] {
        // for m in [true, false] {
            // let (i, c) = simulate::simulate_list(&inst_list, &32, r, m)?;
            // print!("\n{}", i as f64/c as f64);
        // }
    //}

    Ok(())
}
