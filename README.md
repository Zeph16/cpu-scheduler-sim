# CPU Scheduler Simulator

- Simple CPU scheduler built completely in Rust that implements primitive CPU scheduling policies. Built for demo and educational purposes.

## Usage
1. Clone this repository.
2. If you have [cargo](https://crates.io/) installed on your system, go to the cloned directory and run ```cargo build --release```. You can find the compiled executable in ```./bin//cpu-sim``` (or ```cpu-sim.exe``` if you are on Windows).
3. If you don't have cargo installed, there are already two compiled binaries in this repository's root.
	- ```cpu-sim``` was compiled on an Artix Linux x86_64 system. It should execute as expected on most 64-bit linux distributions. Navigate to the ```bin``` directory and run ```./cpu-sim --help```
	
	- ```cpu-sim.exe``` was compiled on a Windows 11 64-bit system. This should also execute as expected on Windows 10 or 11 64-bit systems. Navigate to the ```bin``` directory using your command prompt and run ```cpu-sim.exe --help```
	
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


# Main takeaways

This scheduler supports the following scheduling policies as of now:<br/>
  - FIFO/FCFS: **First-In-First-Out**
  - SJF: **Shortest-Job-First**
  - PSJF/STCF/SRTF: **Preemptive-Shortest-Job-First**
  - RR: **Round-Robin**<br/>

  It will simulate the whole timeline of process execution based on the given "job" or process list and give you a summary plus something resembling a Gantt chart. The summary is a table by default, but you can also pass the --minimal or -m flag to show a simpler summary.

- This repository was made purely for educational purposes, so code optimization was NOT an intention. Speed and memory efficiency were exchanged for more declarative code at any place possible.
- The code currently needs refactoring because I decided to change the way I approached the algorithms mid way while developing them. Pieces of code should be extracted as functions to be more declarative and redundant/unnecessary ones should be removed.
- As of now, the code for the **Round-Robin** policy is the most declaratively written algo out of the 4 available. The code for the others should be changed to be similarly more declarative as well, hopefully soon.

### Sample execution
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

**P.S.** This project was a spontaneous decision and built with a sudden burst of energy in a day so there are bound to be some code smells here and there. If I decide to focus on it, I'll definitely refactor the code to make it a bit more declarative and comfortable to go through and understand even if you're not experienced with Rust.

