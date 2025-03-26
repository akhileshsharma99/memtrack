extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn track_mem(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_output = &input_fn.sig.output;
    let fn_body = &input_fn.block;
    let fn_vis = &input_fn.vis;
    let fn_generics = &input_fn.sig.generics;

    let output = quote! {
        #fn_vis fn #fn_name #fn_generics(#fn_inputs) #fn_output {
            fn get_memory_usage_mb() -> f64 {
                #[cfg(target_os = "linux")]
                {
                    // Try /proc/self/statm first - this gives resident set size in pages
                    if let Ok(statm) = std::fs::read_to_string("/proc/self/statm") {
                        let values: Vec<&str> = statm.split_whitespace().collect();
                        if values.len() >= 2 {
                            if let Ok(resident_pages) = values[1].parse::<u64>() {
                                // Convert pages to MB (typically 4KB pages)
                                return resident_pages as f64 * 4.0 / 1024.0;
                            }
                        }
                    }

                    // Fallback to /proc/self/status
                    if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
                        for line in status.lines() {
                            if line.starts_with("VmRSS:") {
                                if let Some(kb_str) = line.split_whitespace().nth(1) {
                                    if let Ok(kb) = kb_str.parse::<u64>() {
                                        return kb as f64 / 1024.0;
                                    }
                                }
                            }
                        }
                    }
                }

                // Fallback to ps command for non-Linux platforms
                let output = std::process::Command::new("ps")
                    .args(&["-o", "rss=", "-p", &std::process::id().to_string()])
                    .output();

                match output {
                    Ok(output) => {
                        if let Ok(mem_str) = String::from_utf8(output.stdout) {
                            if let Ok(mem_kb) = mem_str.trim().parse::<u64>() {
                                return mem_kb as f64 / 1024.0;
                            }
                        }
                    }
                    Err(_) => {}
                }

                0.0 // Return 0 if all methods fail
            }

            // Force memory stats to update
            fn force_memory_update() {
                // Allocate a small amount of memory to ensure OS updates memory stats
                let v = Box::new([0u8; 4096]);
                // Access the memory to ensure it's committed
                let _ = v[0];
                // Let the memory drop here
            }

            force_memory_update();
            let mem_before = get_memory_usage_mb();
            println!("Memory before executing {}: {:.2} MB", stringify!(#fn_name), mem_before);

            // Execute the actual function body
            let result = {
                #fn_body
            };

            force_memory_update();
            let mem_after = get_memory_usage_mb();
            println!("Memory after executing {}: {:.2} MB", stringify!(#fn_name), mem_after);
            println!("Memory difference: {:.2} MB", mem_after - mem_before);

            result
        }
    };

    output.into()
}
