//! Implements a hello-world example for Arbitrum Stylus, providing a Solidity ABI-equivalent
//! Rust implementation of the Counter contract example provided by Foundry.
//! Warning: this code is a template only and has not been audited.
//! ```
//! contract Counter {
//!     uint256 public number;
//!     function setNumber(uint256 newNumber) public {
//!         number = newNumber;
//!     }
//!     function increment() public {
//!         number++;
//!     }
//! }
//! ```

// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::{alloy_primitives::U256, prelude::*};

use alloc::{string::String, vec::Vec};

// Define the entrypoint as a Solidity storage object, in this case a struct
// called `Counter` with a single uint256 value called `number`. The sol_storage! macro
// will generate Rust-equivalent structs with all fields mapped to Solidity-equivalent
// storage slots and types.
sol_storage! {
    #[entrypoint]
    pub struct Question {
        string ipfs;
        address owner;
        mapping(address => bool) hasAnswered;
        mapping(bool => uint256) answersCount;
    }

    // mapping(bytes32 => Question) public questions;
}

/// Define an implementation of the generated Counter struct, defining a set_number
/// and increment method using the features of the Stylus SDK.
#[external]
impl Question {
    /// Create a new question.
    pub fn createQuestion(&mut self) -> Result<String, Vec<u8>> {
        // Generate a unique identifier for the question.
        // let mut questionBytes = vec![];
        // questionBytes.extend_from_slice(self.ipfs.as_bytes()); // Convert string to bytes
        // let questionId = keccak256(questionBytes.as_slice());

        // Add the question to the storage.
        // self.questions.insert(questionId, *self);

        // Return the question ID.
        Ok(questionId.get_string())
    }

    /// Mark an address as having answered the question.
    // pub fn answerQuestion(&mut self, answer: bool) -> Result<(), Vec<u8>> {
    //     // Check if the address has already answered.
    //     if self.hasAnswered[msg.sender()] {
    //         return Err(vec![0]); // Error code for already answered.
    //     }

    //     // Mark the address as having answered.
    //     self.hasAnswered[msg.sender()] = true;

    //     // Update the answer counts.
    //     self.answerCounts[answer] += 1;

    //     Ok(())
    // }
}
