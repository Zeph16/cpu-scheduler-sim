# CPU Scheduler Simulator

- Simple CPU scheduler built completely in Rust that implements primitive CPU scheduling policies. Built for demo and educational purposes.

## Usage
1. Clone this repository.
2. If you have [cargo](https://crates.io/) installed on your system, go to the cloned directory and run ```cargo build --release```. You can find the compiled executable in a newly created directory```target/release/cpu-sim``` (or ```cpu-sim.exe``` if you are on Windows).
3. If you don't have cargo installed, there are already two compiled binaries in the ```bin``` directory.
	- ```cpu-sim``` was compiled on an Artix Linux x86_64 system. It should execute as expected on most 64-bit linux distributions.
    Navigate to the ```bin``` directory, give execute permissions for ```cpu-sim``` with ```chmod +x cpu-sim```, and run ```./cpu-sim --help```
	
	- ```cpu-sim.exe``` was compiled on a Windows 11 64-bit system. This should also work as expected on Windows 10 or 11 64-bit systems. <br />
 		- Navigate to the ```bin``` directory in cmd or powershell
   		- Run ```.\cpu-sim.exe --help```
	
	If these binaries refuse to show any output when executed, then install [cargo](https://crates.io/) on your system and follow step 2 to compile the program on your own OS.

```
$ ./cpu-sim --help
Usage: cpu-sim [OPTIONS] --jobs <JOBS> [POLICY]

Arguments:
  [POLICY]
          [default: fifo]

          Possible values:
          - fifo: First-In-First-Out
          - fcfs: First-Come-First-Serve (Same as FIFO)
          - sjf:  Shortest-Job-First
          - psjf: Preemptive-Shortest-Job-First
          - stcf: Shortest-Time-to-Completion-First (Same as PSJF)
          - srtf: Shortest-Remaining-Time-First (Same as PSJF)
          - rr:   Round-Robin

Options:
  -j, --jobs <JOBS>
          Specify list of processes as <ARRIVAL_TIME>:<BURST_TIME>, separated only by a comma

          e.g. --jobs 0:5,2:4,5:10    // ARRIVAL_TIME will be 0 if not specified

  -q, --quantum <QUANTUM>
          Time quantum in the case of Round Robin scheduling policy

          [default: 2]

  -t, --trace
          Dump complete cpu trace

  -m, --minimal
          Show only minimal report without a table

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```


# Main Takeaways

This scheduler supports the following scheduling policies as of now:<br/>
  - FIFO/FCFS: **First-In-First-Out**
  - SJF: **Shortest-Job-First**
  - PSJF/STCF/SRTF: **Preemptive-Shortest-Job-First**
  - RR: **Round-Robin**<br/>

  It will simulate the whole timeline of process execution based on the given "job" or process list and give you a summary plus something resembling a Gantt chart. The summary is a table by default, but you can also pass the --minimal or -m flag to show a simpler summary.

- This repository was made purely for educational purposes, so code optimization was NOT an intention. Speed and memory efficiency were exchanged for more declarative code at any place possible.
- The code is more or less self documenting but I've added comments in key steps of the algo to aid non-rustaceans that want to look into the code. So don't be intimidated to go through the code (especially the [scheduler](https://github.com/Zeph16/cpu-scheduler-sim/blob/main/src/handler/mod.rs) function) and see how it's written!
- ~~As of now, the code for the **Round-Robin** policy is the most declaratively written algo out of the 4 available. The code for the others should be changed to be similarly more declarative as well, hopefully soon.~~ The algorithms for all policies are now meshed into one scheduler function, which follows even more declarative methods than before.

### Sample Execution
```
$ ./cpu-sim fifo --jobs 0:4,2:2,3:5 --minimal
*** CPU Scheduling Policy: FIFO/FCFS - First Come First Serve ***

 (PID is assigned in the order of submission, NOT execution.)
PID 0:  Turnaround time - 4.00ms | Waiting time - 0.00ms | Response time - 0.00ms
PID 1:  Turnaround time - 4.00ms | Waiting time - 2.00ms | Response time - 2.00ms
PID 2:  Turnaround time - 8.00ms | Waiting time - 3.00ms | Response time - 3.00ms

           0  1  2  3  4  5  6  7  8  9 10
PID 0  -   *  *  *  *
PID 1  -         .  .  *  *
PID 2  -            .  .  .  *  *  *  *  *
   (Hint: *-RUNNING | .-READY)


Statistics:
- Average turnaround time:  5.33ms
- Average response time:    1.67ms
- Average waiting time:     1.67ms
- Overall time taken:       11.00ms
```
<br/>
<br/>

### Library Crates Used
- [clap](https://github.com/clap-rs/clap)
- [tabled](https://github.com/zhiburt/tabled)

