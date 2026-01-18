use std::time::Instant;
use std::mem;

/// Defines a single node in the linked list.
/// 'T' is the type of data the node holds.
struct Node<T> {
    data: T,
    next: Link<T>,
}

/// Type alias for an optional box containing the next node.
/// This is the "link" in the linked list.
type Link<T> = Option<Box<Node<T>>>;

/// Defines the linked list structure.
struct LinkedList<T> {
    head: Link<T>,
    // Storing the count separately makes it O(1) to retrieve the size,
    // though the challenge is to measure traversal, which is O(n).
    count: usize, 
}

impl<T> LinkedList<T> {
    /// Creates a new, empty linked list.
    fn new() -> Self {
        LinkedList {
            head: None,
            count: 0,
        }
    }

    /// Prepends a new node with the given data to the list (O(1) insertion).
    fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            // Replace the current head with None, and put the old head
            // into the 'next' field of the new node.
            next: mem::replace(&mut self.head, None),
        });
        
        self.head = Some(new_node);
        self.count += 1;
    }

    /// **The core function for measuring traversal time.**
    /// Traverses the entire list and returns the number of nodes visited 
    /// (which should equal `self.count`).
    fn traverse_and_time(&self) -> (usize, std::time::Duration) {
        let start_time = Instant::now();
        let mut current = &self.head;
        let mut visited_count = 0;

        // Loop until 'current' is None (end of the list)
        while let Some(node) = current {
            // Do a minimal operation on the data to ensure the compiler
            // doesn't optimize away the entire loop. 
            // Here, we just count the nodes.
            visited_count += 1;

            // Move to the next node
            current = &node.next;
        }

        let elapsed_time = start_time.elapsed();
        (visited_count, elapsed_time)
    }
}

/// Main function to execute the program.
fn main() {
    // --- Argument Parsing ---
    let args: Vec<String> = std::env::args().collect();
    
    // Check if a count is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <number_of_nodes>", args[0]);
        // Example help:
        eprintln!("Example: {} 1000000", args[0]); 
        return;
    }

    // Parse the number of nodes from the command line argument
    let num_nodes: usize = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: The argument must be a valid positive integer.");
            return;
        }
    };

    println!("--- Linked List Traversal Benchmark ---");
    println!("List Size (N): {}", num_nodes);
    
    // --- List Creation ---
    
    // Start timing the *list creation* process
    let creation_start = Instant::now();
    let mut list = LinkedList::new();

    // Populate the linked list
    for i in 0..num_nodes {
        // We use the loop index 'i' as the data for demonstration
        list.push(i); 
    }
    let creation_time = creation_start.elapsed();
    
    println!("Time to create list: {:?}", creation_time);
    
    // --- List Traversal and Timing ---
    
    // Perform the traversal and get the results
    let (visited, traversal_time) = list.traverse_and_time();
    
    // --- Results Output ---
    
    println!("\n--- Traversal Results ---");
    
    if visited == num_nodes {
        println!("✅ Traversal successful: {} nodes visited.", visited);
    } else {
        println!("❌ Traversal failed: Expected {} nodes, but visited {}.", num_nodes, visited);
    }

    println!("Total Traversal Time: {:?}", traversal_time);
    
    // Optionally convert to nanoseconds for high precision
    let time_in_ns = traversal_time.as_nanos();
    
    if visited > 0 {
        // Calculate the average time per node access
        let avg_time_per_node = time_in_ns as f64 / visited as f64;
        println!("Average time per node: {:.3} nanoseconds", avg_time_per_node);
    }
    
    // 
}
// This code is designed to be run from the command line:
// $ cargo run -- 100000

