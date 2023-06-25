mod handler;
mod types;
mod utils;

use clap::Parser;
use handler::scheduler;
use types::process::Proc;
use utils::{parse_args, sort_by_pid, summary, Args};

fn main() {
    let args = Args::parse();
    let (parsed_procs, policy) = parse_args(&args);
    let procs: Vec<Proc>;
    if let Ok(p) = parsed_procs {
        procs = p;
    } else {
        println!(
            " Error parsing argument supplied to --jobs: \"{}\"",
            args.jobs
        );
        println!("\n Usage <ARRIVAL_TIME>:<BURST_TIME>... | Supply multiple processes by separating them with a comma.");
        println!("-- e.g. 0:10,5:10,10:20");
        println!(" A single number will be taken as burst time, arrival time is 0 by default.");
        println!("-- e.g. 10,5 translates to 0:10,0:5");
        return;
    }

    let (mut processed_procs, total_time) = scheduler(procs, &policy, args.trace);

    sort_by_pid(&mut processed_procs);
    summary(
        &processed_procs,
        policy,
        total_time,
        args.trace,
        args.minimal,
    );
}
