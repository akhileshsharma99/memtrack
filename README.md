# memtrack

A procedural macro for tracking memory usage of Rust functions.

## Overview

`memtrack` provides a simple way to monitor memory usage of functions in Rust programs. It works by wrapping functions with memory tracking code that measures the resident set size (RSS) before and after function execution.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
memtrack = "0.1.0"
```

## Usage

Add the `#[track_mem]` attribute to the functions you want to track:

```rust
use memtrack::track_mem;

#[track_mem]
fn allocate_memory() {
    let data: Vec<u8> = vec![0; 1_000_000];
    println!("Allocated {} bytes", data.len());
}
```

This will print the memory usage before and after the function execution.
