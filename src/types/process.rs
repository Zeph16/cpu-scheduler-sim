use tabled::Tabled;

#[derive(Debug, Clone, Tabled)]
pub struct Proc {
    pub pid: u32,
    pub arrival_time: u32,
    pub burst_time: u32,
    pub turnaround_time: u32,
    pub waiting_time: u32,
    pub response_time: u32,
    pub remaining_burst_time: u32,
    pub trace: String,
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
            remaining_burst_time: 0,
            trace: String::new(),
        }
    }
    pub fn from(
        pid: u32,
        arrival_time: u32,
        burst_time: u32,
        turnaround_time: u32,
        waiting_time: u32,
        response_time: u32,
    ) -> Self {
        Self {
            pid,
            arrival_time,
            burst_time,
            turnaround_time,
            waiting_time,
            response_time,
            remaining_burst_time: burst_time,
            trace: String::new(),
        }
    }
}
