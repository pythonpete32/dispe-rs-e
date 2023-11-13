// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    call, msg,
    prelude::*,
};

sol_storage! {
    #[entrypoint]
    pub struct Disperse {}
}

#[external]
impl Disperse {
    pub fn disperse_ether(
        &mut self,
        recipients: Vec<Address>,
        values: Vec<U256>,
    ) -> Result<(), Vec<u8>> {
        // Ensure the recipients and values vectors have the same length
        if recipients.len() != values.len() {
            return Err(b"Recipients and values vectors must have the same length".to_vec());
        }

        let mut total: Vec<U256> = Vec::new();

        for i in 0..recipients.len() {
            let result = call::transfer_eth(recipients[i], values[i]);
            if result.is_err() {
                return result;
            }

            total.push(values[i]);
        }

        if total.iter().sum::<U256>() != msg::value() {
            return Err(b"Sum of values is not equal to sent value".to_vec());
        }
        Ok(())
    }
}
