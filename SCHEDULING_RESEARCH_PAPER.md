# Comparative Study of CPU Scheduling Algorithms in RustOS

## 1. Abstract and Objectives

CPU scheduling is an important function of an operating system. It decides which process should receive CPU time and in what order.

This study explores four commonly used CPU scheduling algorithms through the RustOS educational project:

1. First Come First Served (FCFS)
2. Shortest Job First (SJF)
3. Priority Scheduling
4. Round Robin (RR)

The objective is to understand how different scheduling algorithms affect waiting time, turnaround time, fairness, and CPU utilization.

This study provides a theoretical and educational comparison of these algorithms.

## 2. Scheduling Metrics

The following measurements are used to compare scheduling algorithms.

### A. Turnaround Time (TAT)

The total time taken by a process from arrival to completion.

**Formula:**

TAT = Completion Time − Arrival Time

### B. Waiting Time (WT)

The total time a process spends waiting for CPU execution.

**Formula:**

WT = Turnaround Time − Burst Time

### C. Average Waiting Time

The average waiting time of all processes.

**Formula:**

Average WT = Total Waiting Time / Number of Processes

### D. Response Time

The time between a process arriving and receiving the CPU for the first time.

### E. Context Switch

A context switch occurs when the CPU changes from executing one process to another.

## 3. Example Workload

The following example workload is used to understand the scheduling algorithms.

| Process | Arrival Time | Burst Time | Priority |
|---|---:|---:|---:|
| P1 – Game Loop | 0 | 6 | 2 |
| P2 – Network Task | 2 | 2 | 1 |
| P3 – Package Task | 4 | 8 | 4 |
| P4 – Telemetry | 6 | 3 | 3 |
| P5 – Memory Task | 8 | 4 | 2 |

Lower priority numbers represent higher priority.

## 4. Comparison of CPU Scheduling Algorithms

### A. First Come First Served (FCFS)

FCFS executes processes in the order they arrive.

**Advantages:**
- Simple to understand.
- Easy to implement.
- Processes are handled in arrival order.

**Disadvantages:**
- Short processes may wait behind long processes.
- Can cause the convoy effect.

### B. Shortest Job First (SJF)

SJF selects the available process with the shortest CPU burst time.

**Advantages:**
- Can reduce average waiting time.
- Suitable for workloads where burst times are known.

**Disadvantages:**
- Burst time must be estimated.
- Longer processes may experience starvation.

### C. Priority Scheduling

Priority scheduling selects the process with the highest priority.

**Advantages:**
- Important tasks can execute earlier.
- Useful when tasks have different levels of importance.

**Disadvantages:**
- Low-priority processes may wait for a long time.
- Starvation can occur.

**Solution – Aging:**

Aging gradually improves the priority of waiting processes so that they eventually get CPU time.

### D. Round Robin (RR)

Round Robin gives each process a fixed amount of CPU time called a time quantum.

After its time quantum expires, the process moves to the back of the ready queue if it is not completed.

**Advantages:**
- Provides fair CPU time sharing.
- Suitable for interactive systems.
- Improves response-time fairness.

**Disadvantages:**
- Very small time quanta cause more context switches.
- Very large time quanta make it similar to FCFS.

## 5. Example Calculation

Consider the workload given in Section 3.

For non-preemptive FCFS, the execution order is:

P1 → P2 → P3 → P4 → P5

The average waiting time is:

(0 + 4 + 4 + 10 + 11) / 5 = **5.8 time units**

The average turnaround time is:

(6 + 6 + 12 + 13 + 15) / 5 = **10.4 time units**

For non-preemptive SJF, selecting the shortest available process gives this order:

P1 → P2 → P4 → P5 → P3

The average waiting time is:

(0 + 4 + 2 + 3 + 11) / 5 = **4.0 time units**

The average turnaround time is:

(6 + 6 + 5 + 7 + 19) / 5 = **8.6 time units**

These are calculated example results, not measurements from physical hardware or a live kernel benchmark.

## 6. Theoretical Comparison

| Algorithm | Main Advantage | Main Limitation |
|---|---|---|
| FCFS | Simple execution order | Convoy effect |
| SJF | Lower average waiting time in suitable workloads | Starvation |
| Priority | Handles important tasks first | Low-priority starvation |
| Round Robin | Fair CPU time sharing | Context-switch overhead |

The comparison shows that each algorithm has different strengths and limitations.

No single algorithm is ideal for every type of workload.

## 7. Proposed Scheduling Improvement

A possible improvement for a future version of RustOS is the Multi-Level Feedback Queue (MLFQ).

MLFQ uses multiple priority queues and allows processes to move between queues based on their execution behaviour.

For example:

- Interactive tasks can receive higher priority.
- CPU-intensive tasks can move to lower-priority queues.
- Waiting processes can receive priority improvements.

This approach can improve responsiveness and fairness.

MLFQ is discussed as a proposed improvement, not as a claim that it has already been implemented in RustOS.

## 8. Conclusion

This study explains four important CPU scheduling algorithms and compares their advantages and limitations.

FCFS is simple, SJF can reduce average waiting time, Priority Scheduling handles important tasks first, and Round Robin provides fair CPU time sharing.

Through this study, RustOS demonstrates important operating system concepts and provides a foundation for understanding how a CPU manages multiple processes.
