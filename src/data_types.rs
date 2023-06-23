use core::fmt;

use clap::ValueEnum;
use tabled::Tabled;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Tabled)]
pub struct Proc {
    pub pid: u32,
    pub arrival_time: u32,
    pub burst_time: u32,
    pub turnaround_time: u32,
    pub waiting_time: u32,
    pub response_time: u32,
    pub status: Status,
    pub remaining_burst_time: u32,
    pub trace: String
}
impl Proc {
    pub fn new() -> Self {
        Self {
            pid: 0,
            arrival_time: 0,
            burst_time: 0,
            turnaround_time: 0,
            waiting_time: 0,
            response_time: 0,
            status: Status::Initial,
            remaining_burst_time: 0,
            trace: String::new()
        }
    }
    pub fn from(
        pid: u32, arrival_time: u32, burst_time: u32, turnaround_time: u32, waiting_time: u32, response_time: u32, status: Status
    ) -> Self {
        Self {
            pid, 
            arrival_time, 
            burst_time,
            turnaround_time,
            waiting_time,
            response_time,
            status,
            remaining_burst_time: burst_time,
            trace: String::new()
        }
    }
}

#[derive(Debug)]
pub enum Policy {
    FIFO,
    SJF,
    PSJF,
    RR(u32)
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Status {
    Initial,
    Ready,
    Running,
    Blocked,
    Done
}

#[derive(ValueEnum, Clone, Debug)]
pub enum PolicyArgs {
    /// First-In-First-Out
    FIFO, 
    /// First-Come-First-Serve (Same as FIFO)
    FCFS, 
    /// Shortest-Job-First
    SJF, 
    /// Preemptive-Shortest-Job-First
    PSJF,
    /// Round-Robin
    RR
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Initial => write!(f, "Initial"),
            Self::Ready => write!(f, "Ready"),
            Self::Running => write!(f, "Running"),
            Self::Blocked => write!(f, "Blocked"),
            Self::Done => write!(f, "Done")
        }
    }
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
impl fmt::Display for PolicyArgs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FIFO => write!(f, "fifo"),
            Self::FCFS => write!(f, "fcfs"),
            Self::SJF => write!(f, "sjf"),
            Self::PSJF => write!(f, "psjf"),
            Self::RR => write!(f, "rr"),
        }
    }
}
