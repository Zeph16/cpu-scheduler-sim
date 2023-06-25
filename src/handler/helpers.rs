use crate::types::{cpu::CPU, policy::Policy, process::Proc};

pub fn check_arrived_jobs(initial: &mut Vec<Proc>, ready: &mut Vec<Proc>, tick: u32, trace: bool) {
    let mut to_ready_indexes: Vec<usize> = Vec::new(); // Index of processes that arrived
    for (i, proc) in initial.iter_mut().enumerate() {
        if proc.arrival_time == tick {
            to_ready_indexes.push(i);
            if trace {
                println!("Process {} arrived!", proc.pid);
            }
        } else {
            proc.trace.push_str("x ");
        }
    }
    for index in to_ready_indexes.iter() {
        ready.push(initial[*index].clone());
    }

    // Remove all processes that arrived from the initial list
    initial.retain(|x| x.arrival_time != tick);
}

pub fn assign_next_process(policy: &Policy, ready: &mut Vec<Proc>, cpu: &mut CPU) {
    let mut next_process_index = usize::MAX; // Index of process that should be moved into CPU

    for (i, proc) in ready.iter_mut().enumerate() {
        match policy {
            Policy::RR(quantum) => {
                // If there's no process being executed
                // OR
                // If the current process has used up its quantum time
                if !cpu.running || cpu.quantum_left == 0 {
                    next_process_index = i;
                    cpu.quantum_left = *quantum;
                    break;
                }
            }

            Policy::PSJF => {
                // If there's no process being executed
                // OR
                // If the process in loop has less burst time than the process in CPU
                if !cpu.running || proc.burst_time < cpu.current.remaining_burst_time {
                    next_process_index = i;
                    break;
                }
            }
            _ => {
                // FIFO or SJF
                // Processes are already sorted by arrival time in the case of FIFO
                // So the first process that's found is the one that arrived first
                //
                // Processes are already sorted by burst time in the case of SJF
                // So the first process that's found is the one that has the least burst time
                if !cpu.running {
                    next_process_index = i;
                    break;
                }
            }
        }
    }

    if next_process_index != usize::MAX { // Found new process to be executed
        if let Policy::PSJF = policy {
            if cpu.running {
                ready.push(cpu.current.clone());
            }
        }
        cpu.current = ready.remove(next_process_index);
        cpu.running = true;
    }

    // Update stats of processes in the ready queue
    for proc in ready.iter_mut() {
        proc.turnaround_time += 1;
        proc.waiting_time += 1;
        if let None = proc.trace.find("*") {
            proc.response_time += 1;
        }
        proc.trace.push_str(". ");
    }
}
