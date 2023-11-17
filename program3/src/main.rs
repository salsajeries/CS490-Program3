/*
CS 490 Program 3
Salwa Jeries
11/16/2023

Dev Environment Used: VScode

This program simulated creating a number of Process nodes, placing them in a Binary Heap, and "executing"
the processes with 2 consumer threads.
*/
use std::io;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use rand::{thread_rng, Rng};

/*
Process Node Struct

id: unsigned int
priority: unsigned int between 0-100
time_slice: (in ms) unsigned int between 200-1000
*/
#[derive(Debug, PartialEq, Eq)]
struct Process {
    id: u32,
    priority: u32,
    time_slice: u32,
}

// Enforces ordering for MinHeap based on priority value of process node
impl Ord for Process {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

// Enforces partial ordering rules for MinHeap for process nodes
impl PartialOrd for Process {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/*
Execute Process

Simulate executing a process by sleeping for the "time_slice" duration (ms),
the current thread cannot work on another process during this time.

process: &Reverse<Process> = current process to be "executed"
thread_name: &str = name of current thread calling the function
*/
fn execute_process(process: &Reverse<Process>, thread_name: &str) {
    println!(
        "{}: executed process {}, pri: {}, for {} ms",                              // Display current thread, process, and process info
        thread_name, process.0.id, process.0.priority, process.0.time_slice
    );
    thread::sleep(Duration::from_millis(process.0.time_slice as u64));              // Sleep for "time_slice" duration to simulate execution
}

/*
Consumer Thread

Control request process to utilize resources to execute a process. Pulls highest priority process
(lowest value) from the top of the heap to process next. Thread is complete when the heap is
empty and there are no more processes to pull.

heap: Arc<Mutex<BinaryHeap<Reverse<Process>>>> = binary heap of current processes
thread_name: &'static str = name of thread (Consumer 1 or 2)
completed_processes: &mut u32 = total number of processes completed so far for the current thread
*/
fn consumer_thread(heap: Arc<Mutex<BinaryHeap<Reverse<Process>>>>, thread_name: &'static str, completed_processes: &mut u32) {
    
    loop {

        // Let process be the next item in the priority queue (binary heap top element)
        let process;
        {
            let mut heap = heap.lock().unwrap();
            process = heap.pop();
        }

        match process {
            // Process exists and is valid, execute process
            Some(p) => {
                execute_process(&p, thread_name);
                *completed_processes += 1;
            }
            // Process does not exist, heap must be empty
            None => {
                // Signal that thread is completed, display how many processes executed total for this thread
                println!("{} has completed and executed {} processes", thread_name, completed_processes);
                break;
            }
        }
    }
}

/*
Producer Thread

Randomly generates "n" new process nodes for "m" generations. Delay between
generations for "s" milliseconds. When nodes are generated, push them to the
binary heap where they will be sorted by priority (ascending order)

heap: Arc<Mutex<BinaryHeap<Reverse<Process>>>> = binary heap of current processes
n: u32 = number of process nodes to generate each time
s: u64 = sleep time (ms) between generations
m: u32 = number of times the producer should generate "n" nodes
*/
fn producer_thread(heap: Arc<Mutex<BinaryHeap<Reverse<Process>>>>, n: u32, s: u64, m: u32) {
    
    let mut rng = thread_rng();     // Random number generator
    let mut counter: u32 = 0;       // Num of threads created, used for "id"
    println!("\n... producer is starting its work ...");

    for _phase in 1..=m {

        // Beginning of new generation phase
        println!("\n... producer is sleeping ...\n");

        // Generate "n" process nodes
        for _ in 0..n {
            let process = Process {
                id: counter,
                priority: rng.gen_range(0..100),
                time_slice: rng.gen_range(200..1000),
            };

            let mut heap = heap.lock().unwrap();    // Unlock heap to add new process
            heap.push(Reverse(process));            // Push process to the heap
            counter += 1;                           // Increment process count
        }

        thread::sleep(Duration::from_millis(s));    // Delay between generation phases
    }

    // Completed producer thread
    println!("\n... producer has finished: {} nodes were generated ...\n", n * m);
}


/* Main Function */
fn main() {
    
    // TEST INPUTS
    //let n = 10;   // number of process nodes to generate each time
    //let s = 1000; // sleep time in ms between generations
    //let m = 4;    // number of times the producer should generate N nodes.

    // User input number of times the producer should generate N nodes
    println!("Enter number of generation phases for the producer:");
    let mut phases = String::new();
    io::stdin()
        .read_line(&mut phases)
        .expect("You entered an invalid measure.\n");
    let m: u32 = phases.trim().parse().expect("You entered an invalid measure.\n");

    // User input sleep time in ms between generations
    println!("Enter sleep time in ms for the producer to pause between generation phases:");
    let mut sleeptime = String::new();
    io::stdin()
        .read_line(&mut sleeptime)
        .expect("You entered an invalid measure.\n");
    let s: u64 = sleeptime.trim().parse().expect("You entered an invalid measure.\n");

    // User input number of process nodes to generate each time
    println!("Enter number of processes to generate each phase:");
    let mut num_proc = String::new();
    io::stdin()
        .read_line(&mut num_proc)
        .expect("You entered an invalid measure.\n");
    let n: u32 = num_proc.trim().parse().expect("You entered an invalid measure.\n");

    println!("Starting Simulation");

    // Create a shared heap wrapped in a Mutex
    let heap = Arc::new(Mutex::new(BinaryHeap::new()));

    // Create a counter for completed processes shared between consumers
    let mut completed_processes_1: u32 = 0;
    let mut completed_processes_2: u32 = 0;

    // Spawn producer thread
    let heap_clone = Arc::clone(&heap);
    let producer = thread::spawn(move || producer_thread(heap_clone, n, s, m));
    // Sleep briefly before starting the producer to ensure there are nodes in the heap
    thread::sleep(Duration::from_millis(500));

    // Spawn consumer1 thread
    let heap_clone = Arc::clone(&heap);
    let consumer1 = thread::spawn(move || consumer_thread(heap_clone, "Consumer 1", &mut completed_processes_1));

    // Spawn consumer2 thread
    let heap_clone = Arc::clone(&heap);
    let consumer2 = thread::spawn(move || consumer_thread(heap_clone, "\tConsumer 2", &mut completed_processes_2));     // "\t" for print purposes

    // Process all threads
    producer.join().unwrap();
    consumer1.join().unwrap();
    consumer2.join().unwrap();

    // Threads completed, print completion message
    println!("\nBoth consumers have completed.");
}
