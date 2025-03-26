use memtrack::track_mem;

#[track_mem]
fn allocate_memory() {
    let _data: Vec<u8> = vec![0; 1_000_000];
}

#[track_mem]
fn create_memory_leak() {
    let mut data = vec![0u8; 5_000_000];

    for i in 0..data.len() {
        data[i] = (i % 255) as u8;
    }

    // Explicitly leak the memory - this will not be freed
    let boxed_data = Box::new(data);
    Box::leak(boxed_data);

    // Force memory stats to update
    std::thread::sleep(std::time::Duration::from_millis(100));
}

fn main() {
    println!("\n=== MEMORY TRACKING DEMO ===");

    // Second allocation
    println!("\n=== Testing normal allocation ===");
    allocate_memory();

    // Memory leak test
    println!("=== Creating a memory leak ===");
    create_memory_leak();

    println!("\n=== Demo completed ===");
}
