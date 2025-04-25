use std::io;

mod parse;
mod translate;
mod models;

fn main() -> io::Result<()>{

    let raw_inst_list = parse::parse_file("/Users/fionafisher/Desktop/s25/modern_limits_of_ilp_18742/sim_traces/SPEC06/lbm_trace.log")?;
    let inst_list = translate::translate_list(raw_inst_list)?;

    for i in &inst_list[..25] {
        print!("\n{}: {:?}, {:?}, {:?}, {}", i.key, i.reg_read_dep, i.reg_write_dep, i.mem_addr, i.mem_load | i.mem_store);
    }

    Ok(())
}
