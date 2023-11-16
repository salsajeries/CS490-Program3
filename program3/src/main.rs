/*
/*
CS 490 Program 2
Salwa Jeries
10/17/2023

Dev Environment Used: VScode

This program simulated creating a Process node and puts it into both a FIFO queue and a Binary Min Heap.
The user will be prompted to input the number of process nodes to be randomly generated. Then, using a
defined Process struct, a node is generated with a process ID, priority (randomly generated integer between
0-100), sleep time in milliseconds (randomly generated integer between 100-2000), and a description string.
Once this process node is generated, it is pushed to a VecDeque based on FIFO as well as a Binary Min Heap,
ordered based on priority. The pushes are verified by checking the size of the queue and heap. Then, the nodes
are dequeued and popped from the queue/heap respectively in the proper order. As each item is dequeued/popped,
the process node fields are printed to the screen. This demonstrates that they were added to the queue/heap in
the correct order. The sizes of the queue/heap are printed to the screen to verify that they have both been
drained correctly before quitting the program.
*/
use std::io;
use rand::Rng;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

/*
Process Node Struct

process_id: unsigned int
priority: unsigned int between 0-100
sleep_time: (in ms) unsigned int between 100-2000
description: string
*/
#[derive(Debug, Eq, PartialEq, Clone)]
struct Process {
    process_id: u32,
    priority: u32,
    sleep_time: u32,
    description: String,
}

/*
Generate New Process Node 

Randomly generates a new process node based on a given process ID, random
priority value (0-100), random sleep time in ms (100-2000), and description
(generated based on process_id).

process_id: u32 = process ID of node to be generated
*/
fn generate_process(process_id: u32) -> Process {
    let priority = rand::thread_rng().gen_range(0..=100);
    let sleep_time = rand::thread_rng().gen_range(200..=1000);
    let description = format!("Process Node: {}", process_id);

    Process {
        process_id,
        priority,
        sleep_time,
        description,
    }
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

/* Main Function */
fn main() {

    // User input number of nodes to generate
    println!("Enter the number of process nodes to generate:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("You entered an invalid measure.\n");
    let nodes: u32 = input.trim().parse().expect("You entered an invalid measure.\n");

    // Create a new VecDeque & Binary Min Heap
    let mut vec_deque: VecDeque<Process> = VecDeque::new();
    let mut binary_heap: BinaryHeap<Reverse<Process>> = BinaryHeap::new();
    
    // Initialize queue and heap with new process nodes
    println!("\nNow creating and adding {} process nodes to a Queue and to a binary minheap", nodes);
    for i in 1..=nodes {
        let current: Process = generate_process(i);
        
        vec_deque.push_back(current.clone());
        binary_heap.push(std::cmp::Reverse(current.clone()));
    }

    // Verify size of queue and heap
    println!("Verifying. The queue contains {} elements", vec_deque.len());
    println!("Verifying. The heap contains {} elements", binary_heap.len());

    // Drain queue and print process nodes as they are removed
    println!("\nNow, draining the Queue, one process at a time ...");
    for _i in 1..=nodes {
        if let Some(item) = vec_deque.pop_front() {
            println!("Pid: {:10}, pri: {:10}, sleep: {:10}, desc: {:10}", item.process_id, item.priority, item.sleep_time, item.description);
        }
    }

    // Drain min heap and print process nodes as they are removed
    println!("\nNow, draining the MinHeap, one process at a time ...");
    for _i in 1..=nodes {
        if let Some(item) = binary_heap.pop() {
            println!("Pid: {:10}, pri: {:10}, sleep: {:10}, desc: {:10}", item.0.process_id, item.0.priority, item.0.sleep_time, item.0.description);
        }
    }
    
    // Verify that the queue and heap are drained
    println!("\nVecDeque Size: {}", vec_deque.len());
    println!("Binary Min Heap Size: {}", binary_heap.len());

    // Quit statement
    println!("\nGoodbye!");


}*/

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::collections::BinaryHeap;
use rand::{thread_rng, Rng};

// A struct to represent a process
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Process {
    id: u32,
    priority: u32,
    time_slice: u32,
}

// Function to simulate the execution of a process
fn execute_process(process: &Process, thread_name: &str) {
    println!(
        "{}: executed process {}, pri: {}, for {} ms",
        thread_name, process.id, process.priority, process.time_slice
    );
    thread::sleep(Duration::from_millis(process.time_slice as u64));
}

// Consumer function for each thread
fn consumer_thread(heap: Arc<Mutex<BinaryHeap<Process>>>, thread_name: &'static str, completed_processes: &mut u32) {
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
                //let mut count = completed_processes.lock().unwrap();
                *completed_processes += 1;
            }
            // Process does not exist, heap must be empty
            None => {
                println!("{} has completed and executed {} processes", thread_name, completed_processes);
                break;
            }
        }
    }
}

// Producer function
fn producer_thread(heap: Arc<Mutex<BinaryHeap<Process>>>, n: u32, s: u64, m: u32) {
    let mut rng = thread_rng();

    println!("... producer is starting its work ...");

    for phase in 1..=m {
        println!("... producer is sleeping ...");

        for _ in 0..n {
            let process = Process {
                id: rng.gen(),
                priority: rng.gen_range(0..100),
                time_slice: rng.gen_range(200..1001),
            };

            let mut heap = heap.lock().unwrap();
            heap.push(process);
        }

        thread::sleep(Duration::from_millis(s));
    }

    println!("... producer has finished: {} nodes were generated ...", n * m);
}

fn main() {
    // User input
    let n = 10; // number of process nodes to generate each time
    let s = 1000; // sleep time in ms between generations
    let m = 4; // number of times the producer should generate N nodes.

    // Create a shared heap wrapped in a Mutex
    let heap = Arc::new(Mutex::new(BinaryHeap::new()));

    // Create a counter for completed processes shared between consumers
    let mut completed_processes_1: u32 = 0;
    let mut completed_processes_2: u32 = 0;

    // Spawn producer thread
    let heap_clone = Arc::clone(&heap);
    let producer = thread::spawn(move || producer_thread(heap_clone, n, s, m));
    // Sleep briefly before starting the producer to ensure there are nodes in the heap
    thread::sleep(Duration::from_millis(1000));

    // Spawn consumer threads
    let heap_clone = Arc::clone(&heap);
    let consumer1 = thread::spawn(move || consumer_thread(heap_clone, "Consumer 1", &mut completed_processes_1));

    let heap_clone = Arc::clone(&heap);
    let consumer2 = thread::spawn(move || consumer_thread(heap_clone, "Consumer 2", &mut completed_processes_2));

    // Wait for all threads to finish
    producer.join().unwrap();
    consumer1.join().unwrap();
    consumer2.join().unwrap();

    println!("Both consumers have completed.");
}
