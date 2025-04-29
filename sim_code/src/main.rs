use std::io;
use std::fs;
use rayon::prelude::*;

mod parse;
mod translate;
mod simulate;
mod experiment;
mod models;

fn main() -> io::Result<()>{
    // read through directories to get vectors of files, then concatentate them
    let files_06: Vec<fs::DirEntry> = fs::read_dir("/Volumes/KINGSTON/Coursework/modern_limits_of_ilp_18742/sim_traces/SPEC06")?.filter_map(Result::ok).collect();
    let files_17: Vec<fs::DirEntry> = fs::read_dir("/Volumes/KINGSTON/Coursework/modern_limits_of_ilp_18742/sim_traces/SPEC17")?.filter_map(Result::ok).collect();
    let mut files = files_06;
    files.extend(files_17);

    // simulate these experiments in parallel
    let result: Vec<io::Result<()>> = files.par_iter().map(|file| {
        if let Some(path) = file.path().to_str() {
            let path_split: Vec<_> = path.split("/").collect();
            if path_split[7] == "gcc_trace.log" {
                let profile_path = "/Volumes/KINGSTON/Coursework/modern_limits_of_ilp_18742/sim_results/exp6/".to_string() + path_split[6] + "/" + path_split[7] + "/profile.csv";
                let sim_path = "/Volumes/KINGSTON/Coursework/modern_limits_of_ilp_18742/sim_results/exp6/".to_string() + path_split[6] + "/" + path_split[7] + "/sim.csv";
                experiment::run_experiment(path, &profile_path, &sim_path)?;
            }
            
        }
        Ok(())
    }).collect();

    if let Some(err) = result.into_iter().find_map(Result::err) {
        return Err(err);
    }

    Ok(())
    
}
