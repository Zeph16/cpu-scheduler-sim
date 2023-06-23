use crate::data_types::Proc;


pub fn sort_by_arrival(list: &mut Vec<Proc>) {
    list.sort_by(|a, b| a.arrival_time.cmp(&b.arrival_time));
}

pub fn sort_by_burst(list: &mut Vec<Proc>) {
    list.sort_by(|a, b| a.burst_time.cmp(&b.burst_time));
}

pub fn sort_by_remaining_burst(list: &mut Vec<Proc>) {
    list.sort_by(|a, b| a.remaining_burst_time.cmp(&b.remaining_burst_time));
}

pub fn sort_by_pid(list: &mut Vec<Proc>) {
    list.sort_by(|a, b| a.pid.cmp(&b.pid));
}
