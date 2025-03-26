extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn track_mem(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input function
    let input_fn = parse_macro_input!(item as ItemFn);

    // Extract function details
    let fn_name = &input_fn.sig.ident;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_output = &input_fn.sig.output;
    let fn_body = &input_fn.block;
    let fn_vis = &input_fn.vis;
    let fn_generics = &input_fn.sig.generics;

    // Generate a new function that wraps the original one with memory tracking
    let output = quote! {
        #fn_vis fn #fn_name #fn_generics(#fn_inputs) #fn_output {
            // Get current memory usage before function execution
            let mem_before = {
                let process = psutil::process::Process::new(std::process::id())
                    .expect("Failed to get current process");
                let memory_info = process.memory_info()
                    .expect("Failed to get memory info");
                memory_info.rss() // Resident Set Size in bytes
            };

            println!("Memory before executing {}: {} bytes", stringify!(#fn_name), mem_before);

            // Execute the original function body
            let result = {
                #fn_body
            };

            // Get memory usage after function execution
            let mem_after = {
                let process = psutil::process::Process::new(std::process::id())
                    .expect("Failed to get current process");
                let memory_info = process.memory_info()
                    .expect("Failed to get memory info");
                memory_info.rss() // Resident Set Size in bytes
            };

            println!("Memory after executing {}: {} bytes", stringify!(#fn_name), mem_after);
            println!("Memory difference: {} bytes", mem_after as i64 - mem_before as i64);

            result
        }
    };

    output.into()
}
