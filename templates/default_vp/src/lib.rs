// This is a boilerplate Validity Predicate for the Anoma network.
// It is written in Rust and will be compiled to Wasm.

use serde::{Deserialize, Serialize};

// This struct represents the data our VP expects to receive.
// It's a simplified "intent" for transferring a token.
#[derive(Deserialize, Serialize)]
pub struct TransferIntent {
    pub source: String,
    pub target: String,
    pub token: String,
    pub amount: u64,
}

// This is the main entry point for the VP.
// The Anoma node will call this function with the intent data.
// It returns a boolean indicating if the intent is valid.
//
// We use `#[no_mangle]` to ensure the function name is not changed by the compiler.
#[no_mangle]
pub extern "C" fn validate_intent(intent_bytes: &[u8]) -> bool {
    // Try to deserialize the incoming byte slice into our TransferIntent struct.
    let intent: Result<TransferIntent, _> = serde_json::from_slice(intent_bytes);

    match intent {
        Ok(transfer) => {
            // If deserialization is successful, apply the validation logic.
            // For this boilerplate, we'll just check if the amount is greater than 0.
            is_valid(&transfer)
        }
        Err(_) => {
            // If the data can't be parsed into our struct, the intent is invalid.
            false
        }
    }
}

// The core validation logic.
fn is_valid(intent: &TransferIntent) -> bool {
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