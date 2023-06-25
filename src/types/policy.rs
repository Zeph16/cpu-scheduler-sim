use core::fmt;

#[derive(Debug, Clone)]
pub enum Policy {
    FIFO,
    SJF,
    PSJF,
    RR(u32),
}

impl fmt::Display for Policy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FIFO => write!(f, "FIFO/FCFS - First Come First Serve"),
            Self::SJF => write!(f, "SJF - Shortest Job First"),
            Self::PSJF => write!(f, "PSJF - Preemptive Shortest Job First"),
            Self::RR(t) => write!(f, "RR - Round Robin with {t} time quantum"),
        }
    }
}
