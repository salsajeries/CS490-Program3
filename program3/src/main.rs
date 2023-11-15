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
    let sleep_time = rand::thread_rng().gen_range(100..=2000);
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


}