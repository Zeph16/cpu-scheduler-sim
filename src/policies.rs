use crate::{data_types::{Proc, Status}, utils::{sort_by_arrival, sort_by_burst, sort_by_remaining_burst}};

pub fn rr(initial: &mut Vec<Proc>, quantum: u32, trace: bool) -> u32 {
    let mut tick = 0;
    let mut quantum_left = quantum;
    let mut current: Proc = Proc::new();
    let mut running = false;
    let total = initial.len();
    let mut to_ready_indexes: Vec<usize> = Vec::new();
    let mut to_current_index: usize;

    let mut ready: Vec<Proc> = Vec::new();
    let mut finished: Vec<Proc> = Vec::new();

    loop {
        if finished.len() == total {
            break;
        }
        if trace {
            println!("- Tick {} -", tick + 1);
        }

        to_current_index = usize::MAX;


        for (i, proc) in initial.iter_mut().enumerate() {
            if proc.arrival_time == tick {
                to_ready_indexes.push(i);
            } else {
                proc.trace.push_str("x ");
            }
        }

        for index in to_ready_indexes.iter() {
            ready.push(initial[*index].clone());
        }
        if quantum_left == 0 {
            ready.push(current.clone());
        }

        initial.retain(|x| x.arrival_time != tick);
        to_ready_indexes.clear();

        for (i, proc) in ready.iter_mut().enumerate() {
            if !running || quantum_left == 0 {
                to_current_index = i;
                running = true;
                quantum_left = quantum;
                continue;
            }

            proc.turnaround_time += 1;
            proc.waiting_time += 1;
            match proc.trace.find("*") {
                None => proc.response_time += 1,
                _ => ()
            }
            proc.trace.push_str(". ");
        }

        if to_current_index != usize::MAX {
            current = ready.remove(to_current_index);
        }

        if running {
            current.trace.push_str("* ");
            current.turnaround_time += 1;
            current.remaining_burst_time -= 1;
            if current.remaining_burst_time == 0 {
                if trace {
                    println!("Process {} finished!", current.pid);
                }
                running = false;
                quantum_left = quantum;
                finished.push(current.clone());
            } else {
                if trace {
                    println!("Process {} running...burst time left: {}", current.pid, current.remaining_burst_time);
                }
            }
        }
        quantum_left -= 1;

        tick += 1;
        if trace {
            println!("");
        }
    }

    for i in 0..finished.len() {
        initial.push(finished[i].clone());
    }

    return tick;
}


pub fn psjf(proc_list: &mut Vec<Proc>, trace: bool) -> u32 {
    let mut tick = 0;
    let mut current: Proc = Proc::new();
    let mut current_index;
    let mut finished = 0;
    let mut running: bool = false;
    let total = proc_list.len();
    sort_by_burst(proc_list);
    loop {
        if finished == total{
            break;
        }
        if trace {
            println!("- Tick {} -", tick + 1);
        }
        current_index = usize::MAX;
        finished = 0;
        for (i, proc) in proc_list.iter_mut().enumerate() {
            match proc.status {
                Status::Initial => {
                    if proc.arrival_time == tick {
                        if trace {
                            println!("Process {} arrived!", proc.pid);
                        }
                        proc.status = Status::Ready;
                        

                        if current_index == usize::MAX && (!running || proc.burst_time < current.remaining_burst_time) {
                            proc.status = Status::Running;
                            current_index = i;
                        } else {
                            proc.trace.push_str(". ");
                        }
                    } else {
                        proc.trace.push_str("x ");
                    }
                },
                Status::Ready => {
                    proc.turnaround_time += 1;
                    proc.waiting_time += 1;
                    match proc.trace.find("*") {
                        None => proc.response_time += 1,
                        _ => ()
                    }
                    if !running && current_index == usize::MAX {
                        proc.status = Status::Running;
                        current_index = i;
                    } else {
                        proc.trace.push_str(". ");
                    }
                },
                Status::Done => {
                    finished += 1;
                },
                _ => ()
            }
        }
        if current_index != usize::MAX {
            if running {
                current.trace.push_str(". ");
                current.status = Status::Ready;
                proc_list.push(current);
                sort_by_remaining_burst(proc_list);
            }
            current = proc_list.remove(current_index);
            running = true;
        }
        if running {
            current.trace.push_str("* ");
            current.turnaround_time += 1;
            current.remaining_burst_time -= 1;
            if current.remaining_burst_time == 0 {
                if trace {
                    println!("Process {} finished!", current.pid);
                }
                current.status = Status::Done;
                running = false;
                finished += 1;
                proc_list.push(current.clone());
            } else {
                if trace {
                    println!("Process {} running...burst time left: {}", current.pid, current.remaining_burst_time);
                }
            }
        }
        tick += 1;
        if trace {
            println!("");
        }
    }

    return tick;
}


pub fn sjf(proc_list: &mut Vec<Proc>, trace: bool) -> u32 {
    let mut tick = 0;
    let mut current: Proc = Proc::new();
    let mut current_index;
    let mut finished = 0;
    let mut running: bool = false;
    let total = proc_list.len();
    sort_by_burst(proc_list);
    loop {
        if finished == total{
            break;
        }
        if trace {
            println!("- Tick {} -", tick + 1);
        }
        current_index = usize::MAX;
        finished = 0;
        for (i, proc) in proc_list.iter_mut().enumerate() {
            match proc.status {
                Status::Initial => {
                    if proc.arrival_time == tick {
                        if trace {
                            println!("Process {} arrived!", proc.pid);
                        }
                        proc.status = Status::Ready;
                        if !running {
                            proc.status = Status::Running;
                            current_index = i;
                            running = true;
                            proc.trace.push_str("* ");
                        } else {
                            proc.trace.push_str(". ");
                        }
                    } else {
                        proc.trace.push_str("x ");
                    }
                },
                Status::Ready => {
                    if !running {
                        proc.status = Status::Running;
                        current_index = i;
                        running = true;
                        proc.trace.push_str("* ");
                    } else {
                        proc.trace.push_str(". ");
                    }
                    proc.turnaround_time += 1;
                    proc.response_time += 1;
                    proc.waiting_time += 1;
                },
                Status::Done => {
                    finished += 1;
                },
                _ => ()
            }
        }
        if current_index != usize::MAX {
            current = proc_list.remove(current_index);
            running = true;
        }
        if running {
            current.turnaround_time += 1;
            current.remaining_burst_time -= 1;
            if current.remaining_burst_time == 0 {
                if trace {
                    println!("Process {} finished!", current.pid);
                }
                current.status = Status::Done;
                running = false;
                finished += 1;
                proc_list.push(current.clone());
            } else {
                current.trace.push_str("* ");
                if trace {
                    println!("Process {} running...burst time left: {}", current.pid, current.remaining_burst_time);
                }
            }
        }
        tick += 1;
        if trace {
            println!("");
        }
    }

    return tick;
}


pub fn fifo(proc_list: &mut Vec<Proc>, trace: bool) -> u32 {
    let mut tick = 0;
    let mut current: Proc = Proc::new();
    let mut current_index;
    let mut finished = 0;
    let mut running: bool = false;
    let total = proc_list.len();
    sort_by_arrival(proc_list);
    loop {
        if finished == total{
            break;
        }
        if trace {
            println!("- Tick {} -", tick + 1);
        }
        current_index = usize::MAX;
        finished = 0;
        for (i, proc) in proc_list.iter_mut().enumerate() {
            match proc.status {
                Status::Initial => {
                    if proc.arrival_time == tick {
                        if trace {
                            println!("Process {} arrived!", proc.pid);
                        }
                        proc.status = Status::Ready;
                        if !running {
                            proc.status = Status::Running;
                            current_index = i;
                            running = true;
                            proc.trace.push_str("* ");
                        } else {
                            proc.trace.push_str(". ");
                        }
                    } else {
                        proc.trace.push_str("x ");
                    }
                },
                Status::Ready => {
                    if !running {
                        proc.status = Status::Running;
                        current_index = i;
                        running = true;
                        proc.trace.push_str("* ");
                    } else {
                        proc.trace.push_str(". ");
                    }
                    proc.turnaround_time += 1;
                    proc.response_time += 1;
                    proc.waiting_time += 1;
                },
                Status::Done => {
                    finished += 1;
                },
                _ => ()
            }
        }
        if current_index != usize::MAX {
            current = proc_list.remove(current_index);
            running = true;
        }
        if running {
            current.turnaround_time += 1;
            current.remaining_burst_time -= 1;
            if current.remaining_burst_time == 0 {
                if trace {
                    println!("Process {} finished!", current.pid);
                }
                current.status = Status::Done;
                running = false;
                finished += 1;
                proc_list.push(current.clone());
            } else {
                current.trace.push_str("* ");
                if trace {
                    println!("Process {} running...burst time left: {}", current.pid, current.remaining_burst_time);
                }
            }
        }
        tick += 1;
        if trace {
            println!("");
        }
    }

    return tick;
}
