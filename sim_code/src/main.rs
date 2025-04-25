use std::io;

mod parse;
mod translate;
mod simulate;
mod models;

fn main() -> io::Result<()>{

    let raw_inst_list = parse::parse_file("/Users/fionafisher/Desktop/s25/modern_limits_of_ilp_18742/sim_traces/SPEC06/lbm_trace.log")?;
    let inst_list = translate::translate_list(raw_inst_list)?;
    let (i, c) = simulate::simulate_list(&inst_list, &32, true, false)?;

    print!("{}/{}", i, c);

    Ok(())
}
