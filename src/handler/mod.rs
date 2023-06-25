use crate::{
    types::{cpu::CPU, policy::Policy, process::Proc},
    utils::{sort_by_arrival, sort_by_burst},
};

use self::helpers::{assign_next_process, check_arrived_jobs};

pub mod helpers;

pub fn scheduler(process_list: Vec<Proc>, policy: &Policy, trace: bool) -> (Vec<Proc>, u32) {
    let mut cpu = CPU::new(policy); // Track current process

    let mut tick = 0; // Total time taken
    let process_count = process_list.len();
    let mut initial = process_list;
    let mut ready: Vec<Proc> = Vec::new(); // Ready queue
    let mut finished: Vec<Proc> = Vec::new(); // Finished processes to be returned

    sort_by_arrival(&mut initial);

    loop {
        if finished.len() == process_count {
            // All processes finished
            break;
        }
        if trace {
            println!("- Tick {} -", tick);
        }

        // Move arrived processes into ready queue
        check_arrived_jobs(&mut initial, &mut ready, tick, trace);

        // Preparations for some policies before assigning the next process
        match policy {
            Policy::SJF | Policy::PSJF =>
            // So that the first process in the ready queue will be the shortest
            {
                sort_by_burst(&mut ready)
            }

            Policy::RR(_) => {
                // Put current process back into ready queue if it's finished its quantum time
                if cpu.quantum_left == 0 {
                    ready.push(cpu.current.clone());
                }
            }

            _ => (),
        }

        // Put a process from the ready queue into the CPU according to the policy
        // This is the meat of the scheduler
        assign_next_process(policy, &mut ready, &mut cpu);

        if cpu.running {
            // If there's a process in the cpu, execute it
            if cpu.execute_process(trace) {
                // Returns true if process finishes
                finished.push(cpu.current.clone());
            }
        }

        if trace {
            println!("");
        }
        // Advance "timer"
        tick += 1;
    }

    return (finished, tick);
}
