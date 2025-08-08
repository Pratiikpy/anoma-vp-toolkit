// This is a boilerplate Validity Predicate for the Anoma network.
// It is written in Rust and will be compiled to Wasm.

use serde::{Deserialize, Serialize};
use std::alloc::{alloc, dealloc, Layout};

// This struct represents the data our VP expects to receive.
#[derive(Deserialize, Serialize)]
pub struct TransferIntent {
    pub source: String,
    pub target: String,
    pub token: String,
    pub amount: u64,
}

// ===== Wasm Memory Allocation Helpers =====
// These functions are used by the host (our CLI tool) to manage memory
// inside the Wasm instance.

/// Allocates a block of memory of the given size and returns a pointer to it.
#[no_mangle]
pub extern "C" fn wasm_alloc(size: i32) -> i32 {
    let layout = Layout::from_size_align(size as usize, 1).unwrap();
    unsafe { alloc(layout) as i32 }
}

/// Frees the memory block pointed to by `ptr` with the given size.
#[no_mangle]
pub extern "C" fn wasm_dealloc(ptr: i32, size: i32) {
    let layout = Layout::from_size_align(size as usize, 1).unwrap();
    unsafe { dealloc(ptr as *mut u8, layout) };
}
// =========================================


// This is the main entry point for the VP.
#[no_mangle]
pub extern "C" fn validate_intent(ptr: i32, len: i32) -> i32 {
    // Read the data from the Wasm memory
    let intent_bytes = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };
    
    let intent: Result<TransferIntent, _> = serde_json::from_slice(intent_bytes);

    let is_valid = match intent {
        Ok(transfer) => {
            // If deserialization is successful, apply the validation logic.
            is_valid_transfer(&transfer)
        }
        Err(_) => {
            // If the data can't be parsed, the intent is invalid.
            false
        }
    };
    
    // Return 1 for true, 0 for false
    if is_valid { 1 } else { 0 }
}

// The core validation logic.
fn is_valid_transfer(intent: &TransferIntent) -> bool {
    // Rule 1: The transfer amount must not be zero.
    if intent.amount == 0 {
        return false;
    }

    // Rule 2: The source and target addresses cannot be the same.
    if intent.source == intent.target {
        return false;
    }

    // If all checks pass, the intent is valid.
    true
} 