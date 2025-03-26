use memtrack::track_mem;

#[track_mem]
fn allocate_memory() {
    let data: Vec<u8> = vec![0; 1_000_000];
    println!("Allocated {} bytes", data.len());
}

fn main() {
    println!("Starting memory tracking demo");
    allocate_memory();
    println!("Demo completed");
}
