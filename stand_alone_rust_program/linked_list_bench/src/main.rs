use std::time::Instant;
use std::mem;

// These are specific to x86_64 processors
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::{_rdtsc, _mm_lfence};

struct Node<T> {
    data: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct LinkedList<T> {
    head: Link<T>,
    count: usize,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None, count: 0 }
    }

    fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: mem::replace(&mut self.head, None),
        });
        self.head = Some(new_node);
        self.count += 1;
    }

    /// Performs traversal while measuring both wall-time and CPU cycles
    fn benchmark_traversal(&self) -> (usize, std::time::Duration, u64) {
        let start_time = Instant::now();
        let start_cycles: u64;
        let end_cycles: u64;

        let mut current = &self.head;
        let mut visited_count = 0;

        unsafe {
            // Serializing fence: ensures all previous instructions 
            // are finished before the first rdtsc.
            _mm_lfence(); 
            start_cycles = _rdtsc();
        }

        while let Some(node) = current {
            visited_count += 1;
            current = &node.next;
        }

        unsafe {
            // Serializing fence: ensures the loop is 100% finished
            // before we read the final cycle count.
            _mm_lfence();
            end_cycles = _rdtsc();
        }

        let elapsed_time = start_time.elapsed();
        let elapsed_cycles = end_cycles - start_cycles;

        (visited_count, elapsed_time, elapsed_cycles)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run -- <num_nodes>");
        return;
    }

    let num_nodes: usize = args[1].parse().unwrap_or(100_000);

    let mut list = LinkedList::new();
    for i in 0..num_nodes {
        list.push(i);
    }

    println!("--- x86_64 Hardware Benchmark ---");
    println!("List Size: {}", num_nodes);

    let (visited, time, cycles) = list.benchmark_traversal();

    // --- Statistics ---
    let time_ns = time.as_nanos() as f64;
    let cycles_f = cycles as f64;

    println!("\n[Results]");
    println!("Total Time:   {:?}", time);
    println!("Total Cycles: {}", cycles);

    if visited > 0 {
        println!("\n[Efficiency Metrics]");
        println!("Time per Node:   {:.2} ns", time_ns / visited as f64);
        println!("Cycles per Node: {:.2} ticks", cycles_f / visited as f64);
        
        // This calculates the effective frequency during the test
        let ghz = (cycles_f / time_ns); 
        println!("Effective Speed: {:.2} GHz", ghz);
    }
}

