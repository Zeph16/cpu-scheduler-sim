use super::{policy::Policy, process::Proc};

pub struct CPU {
    pub current: Proc,
    pub running: bool,
    pub quantum: u32,
    pub quantum_left: u32,
}

impl CPU {
    pub fn new(policy: &Policy) -> Self {
        let mut ret = Self {
            current: Proc::new(),
            running: false,
            quantum: 0,
            quantum_left: 0,
        };
        if let Policy::RR(quantum) = policy {
            ret.quantum_left = *quantum;
            ret.quantum = *quantum;
        }

        return ret;
    }
    pub fn execute_process(&mut self, trace: bool) -> bool {
        self.current.trace.push_str("* ");
        self.current.turnaround_time += 1;
        self.current.remaining_burst_time -= 1;
        if self.current.remaining_burst_time == 0 {
            if trace {
                println!("Process {} finished!", self.current.pid);
            }
            self.running = false;
            self.quantum_left = self.quantum; // Reset time quantum for RR
            return true;
        } else {
            if trace {
                println!(
                    "Process {} running...burst time left: {}",
                    self.current.pid, self.current.remaining_burst_time
                );
            }

            // To not overflow unsigned int for policies other than RR
            if self.quantum_left != 0 {
                self.quantum_left -= 1;
            }
            return false;
        }
    }
}
