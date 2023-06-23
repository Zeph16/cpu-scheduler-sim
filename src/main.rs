#![allow(unused_variables, dead_code, unused_imports)]
use std::num::ParseIntError;
use crate::{data_types::{Proc, Policy, Status}, policies::{fifo, sjf, rr}, utils::sort_by_pid};
use policies::psjf;
use tabled::{Table, settings::{Style, Disable, locator::ByColumnName}};
use clap::Parser;
use data_types::PolicyArgs;

mod data_types;
mod policies;
mod utils;


#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = PolicyArgs::FIFO)]
    policy: PolicyArgs,
 
    /// Specify list of processes as <ARRIVAL_TIME>:<BURST_TIME>, separated only by a comma
    ///
    /// e.g. --jobs 0:5,2:4,5:10    // ARRIVAL_TIME will be 0 if not specified
    #[arg(short, long)]
    jobs: String,

    /// Time quantum in the case of Round Robin scheduling policy
    #[arg(short, long, default_value_t = 2, value_parser = clap::value_parser!(u32).range(1..))]
    quantum: u32,

    /// Dump complete cpu trace
    #[arg(short, long, default_value_t = false)]
    trace: bool,

    /// Show only minimal report without a table
    #[arg(short, long, default_value_t = false)]
    minimal: bool

}

fn main() {
    let args = Args::parse();
    let (proc_result, policy) = parse_args(&args);
    let mut procs: Vec<Proc>;
    if let Ok(proc_list) = proc_result {
        procs = proc_list;
    } else {
        println!(" Error parsing argument supplied to --jobs: \"{}\"", args.jobs);
        println!("\n Usage <ARRIVAL_TIME>:<BURST_TIME>... | Supply multiple processes by separating them with a comma."); 
        println!("-- e.g. 0:10,5:10,10:20");
        println!(" A single number will be taken as burst time, arrival time is 0 by default.");
        println!("-- e.g. 10,5 translates to 0:10,0:5");
        return;
    }

    let total_time =  match policy {
        Policy::FIFO => fifo(&mut procs, args.trace),
        Policy::SJF => sjf(&mut procs, args.trace),
        Policy::PSJF => psjf(&mut procs, args.trace),
        Policy::RR(quantum) => rr(&mut procs, quantum, args.trace)
    };
    sort_by_pid(&mut procs);
    summary(&procs, policy, total_time, args.trace, args.minimal);
}

fn summary(procs: &Vec<Proc>, policy: Policy, total_time: u32, trace: bool, minimal: bool) {
    let mut total_turnaround = 0;
    let mut total_waiting = 0;
    let mut total_response = 0;
    let procs_len = procs.len();
    for proc in procs {
        total_turnaround += proc.turnaround_time;
        total_waiting += proc.waiting_time;
        total_response += proc.response_time;
    }

    println!("*** CPU Scheduling Policy: {policy} ***");

    println!();
    println!(" (PID is assigned in the order of submission, NOT execution.)");
    if minimal {
        for proc in procs {
            println!("PID {}:  Turnaround time - {:.2}ms | Waiting time - {:.2}ms | Response time - {:.2}ms"
                    , proc.pid, proc.turnaround_time as f64, proc.waiting_time as f64, proc.response_time as f64);
        }
    } else {
        let mut table = Table::new(procs);
        table.with(Style::modern());
        table.with(Disable::column(ByColumnName::new("status")));
        table.with(Disable::column(ByColumnName::new("remaining_burst_time")));
        table.with(Disable::column(ByColumnName::new("trace")));
        println!("{table}");
        println!();
    }
    println!();
    print!("         ");
    for i in 0..total_time {
        print!("{i:>3}");
    }
    println!();
    for proc in procs {
        print!("PID {:<3}- ", proc.pid);
        for icon in proc.trace.split(" ") {
            if icon == "x" {
                print!("   ")
            } else {
                print!("{icon:>3}");
            }
        }
        println!();
    }
    println!("   (Hint: *-RUNNING | .-READY)");
    println!();
    println!();
    println!("Statistics: ");
    println!("- {:<25} {:.2}ms", "Average turnaround time:", total_turnaround as f64 / procs_len as f64 );
    println!("- {:<25} {:.2}ms", "Average response time:",  total_response as f64  / procs_len as f64 );
    println!("- {:<25} {:.2}ms", "Average waiting time:", total_waiting as f64  / procs_len as f64 );
    println!("- {:<25} {:.2}ms", "Overall time taken:", total_time as f64);
    println!();
    if !trace && total_time > 25 {
        println!("- The Diagram isn't feasable for long execution times, so as an alternative
  run the program with the -t or --trace argument to print a comprehensive cpu trace
  that you can then dump into a file.");
    }
}

fn parse_into_proc(input: &str) -> Result<Vec<Proc>, ParseIntError> {
    let mut parsed: Vec<Proc> = Vec::new();
    let mut arrival_time = 0;
    let mut burst_time = 0;
    let mut pid = 0;
    for slice in input.split(',') {
        let mut iterator = slice.split(':');
        if let Some(first) = iterator.next() {
            if let Some(second) = iterator.next() {
                arrival_time = first.parse()?;
                burst_time = second.parse()?;
            } else {
                burst_time = first.parse()?;
            }
        }

        parsed.push(Proc::from(pid, arrival_time, burst_time, 0, 0, 0, Status::Initial));
        arrival_time = 0;
        pid += 1;
    }
    return Ok(parsed);
}

fn parse_args(input: &Args) -> (Result<Vec<Proc>, ParseIntError>, Policy) {
    let policy = match input.policy {
        PolicyArgs::RR => Policy::RR(input.quantum),
        PolicyArgs::SJF => Policy::SJF,
        PolicyArgs::PSJF => Policy::PSJF,
        _ => Policy::FIFO
    };
    let procs = parse_into_proc(&(input.jobs));

    return (procs, policy);
}
