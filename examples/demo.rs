use memtrack::track_mem;
use std::future::Future;

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

// Simple custom future implementation
struct Sleep {
    duration: std::time::Duration,
}

impl Future for Sleep {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        // This is a simple blocking implementation for demo purposes
        std::thread::sleep(self.duration);
        std::task::Poll::Ready(())
    }
}

#[track_mem]
async fn async_memory_leak() {
    // Allocate memory
    let mut data = vec![0u8; 5_000_000];

    for i in 0..data.len() {
        data[i] = (i % 255) as u8;
    }
    // A simple "async" operation that doesn't require external crates
    Sleep {
        duration: std::time::Duration::from_millis(200),
    }
    .await;

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

    // Async test
    println!("\n=== Testing async allocation ===");
    // Simple executor that just blocks on the future
    futures_executor::block_on(async_memory_leak());

    println!("\n=== Demo completed ===");
}
