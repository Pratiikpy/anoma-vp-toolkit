use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::path::PathBuf;
use wasmtime::*;

pub fn run(vp_path: PathBuf, intent_path: PathBuf) -> Result<()> {
    println!("{} Setting up Wasm runtime...", "⏳".yellow());

    // 1. Setup wasmtime engine and store
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());

    // 2. Load the Wasm module
    let module = Module::from_file(&engine, &vp_path)
        .with_context(|| format!("Failed to load Wasm module from {:?}", vp_path))?;

    // 3. Create an instance of the module
    let instance = Instance::new(&mut store, &module, &[])
        .context("Failed to instantiate Wasm module")?;

    // 4. Read the intent data from the JSON file
    let intent_bytes = fs::read(&intent_path)
        .with_context(|| format!("Failed to read intent file from {:?}", intent_path))?;
    let intent_len = intent_bytes.len() as i32;

    // 5. Get the exported memory and `validate_intent` function from the Wasm instance
    let memory = instance
        .get_memory(&mut store, "memory")
        .context("Failed to find 'memory' export")?;
    let validate_intent_func = instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "validate_intent")
        .context("Failed to find 'validate_intent' function export")?;

    // This is a helper function to allocate memory inside the Wasm module
    let wasm_alloc = instance
        .get_typed_func::<i32, i32>(&mut store, "wasm_alloc")
        .context("Failed to find 'wasm_alloc' function export. Ensure your VP template has it.")?;
        
    // 6. Allocate memory in Wasm for the intent data and get the pointer
    let intent_ptr = wasm_alloc.call(&mut store, intent_len)?;

    // 7. Write the intent data into the Wasm instance's memory
    memory.write(&mut store, intent_ptr as usize, &intent_bytes)?;

    println!("{} Executing VP logic...", "🚀".yellow());

    // 8. Call the `validate_intent` function
    let result_code = validate_intent_func.call(&mut store, (intent_ptr, intent_len))?;

    // 9. Deallocate the memory in the Wasm instance
    let wasm_dealloc = instance
        .get_typed_func::<(i32, i32), ()>(&mut store, "wasm_dealloc")
        .context("Failed to find 'wasm_dealloc' function export. Ensure your VP template has it.")?;

    wasm_dealloc.call(&mut store, (intent_ptr, intent_len))?;

    // 10. Interpret and print the result
    let is_valid = result_code != 0;

    if is_valid {
        println!("\n✅ {}", "Intent is VALID".green().bold());
    } else {
        println!("\n❌ {}", "Intent is INVALID".red().bold());
    }

    Ok(())
} 