use std::io;

mod parse;
mod models;

fn main() -> io::Result<()>{

    let raw_inst_list = parse::parse_file("/Users/fionafisher/Desktop/s25/modern_limits_of_ilp_18742/sim_traces/SPEC06/lbm_trace.log")?;

    Ok(())
}
