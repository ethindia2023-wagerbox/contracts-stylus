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

use alloy_sol_types::sol;
/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::{alloy_primitives::U256, evm::log, prelude::*};

// Define the entrypoint as a Solidity storage object, in this case a struct
// called `Counter` with a single uint256 value called `number`. The sol_storage! macro
// will generate Rust-equivalent structs with all fields mapped to Solidity-equivalent
// storage slots and types.
sol_storage! {
    #[entrypoint]
    pub struct Match {
        bool is_init;
        uint256 id;
        string details_ipfs_url;
        address owner;
        uint256 numPlayers;
        uint256 player_stake;
        uint256 user_stake;
        mapping(uint256 => Question) questions;
        uint256 nextQuestionId;
        State state;
    }

    pub struct Question {
        bool is_ended;
        bool correct_answer;
        string yn_question;
        mapping(address => bool) answers;
        uint256 totalStaked;
    }

    pub struct State {
        mapping(uint256 => Match) matches;
        uint256 nextMatchId;
    }
}

sol! {
    event MatchCreated(uint256 indexed matchId, address owner);
    event GameStarted(uint256 indexed matchId);
    event QuestionCreated(uint256 indexed matchId, uint256 questionId);
    event QuestionAnswered(
        uint256 indexed matchId,
        uint256 questionId,
        address user,
        bool answer
    );
    event QuestionEnded(uint256 indexed matchId, uint256 questionId);
    event AnswerDeclared(
        uint256 indexed matchId,
        uint256 questionId,
        bool correctAnswer
    );
    event WinnersPayout(uint256 indexed matchId, uint256 questionId);

    // Event emitted when a message is received from another chain.
    event QuestionReceived(
        bytes32 indexed messageId, // The unique ID of the message.
        uint64 indexed sourceChainSelector, // The chain selector of the source chain.
        address sender, // The address of the sender from the source chain.
        string text // The text that was received.
    );
    error NotEnoughBalance(uint256 currentBalance, uint256 calculatedFees);
    event QuestionSent(
        bytes32 indexed messageId, // The unique ID of the CCIP message.
        uint64 indexed destinationChainSelector, // The chain selector of the destination chain.
        address receiver, // The address of the receiver on the destination chain.
        string text, // The text being sent.
        address feeToken, // the token address used to pay CCIP fees.
        uint256 fees // The fees paid for sending the CCIP message.
    );

}

/// Define an implementation of the generated Counter struct, defining a set_number
/// and increment method using the features of the Stylus SDK.
#[external]
impl Match {
    pub fn initialize(&self) -> Result<(), Vec<u8>> {
        if self.is_init.get() {
            // error
        }
        // self.isInit = false;
        Ok(())
    }

    fn create_match(&mut self, details_ipfs_url: String, player_stake: u64) -> Result<(), Vec<u8>> {
        let match_id = self.next_match_id;
        let new_match = Match {
            is_init: true,
            id: match_id,
            details_ipfs_url,
            owner: self.env.caller(),
            player_stake,
            user_stake: 0,
        };
        self.state.matches.insert(match_id, new_match);
        self.state.next_match_id += 1;

        // Emit equivalent log statement
        log::info!(
            "Match created: id={}, owner={}",
            match_id,
            self.env.caller().to_string()
        );
        Ok(())
    }

    fn player_stake(&mut self, match_id: u64) -> u64 {
        let match_ = self.state.matches.get(&match_id);
        if match_.is_none() {
            // Handle error: match not found
            panic!("Match not initialized");
        }
        let match_ = match_.unwrap();
        if !match_.is_init {
            // Handle error: match not initialized
            panic!("Match not initialized");
        }
        return match_.player_stake;
    }

    fn start_game(&mut self, match_id: u64) -> Result<(), Vec<u8>> {
        let match_ = self.matches.get_mut(&match_id);
        if match_.is_none() {
            // Handle error: match not found
            panic!("Match not initialized");
        }
        let match_ = match_.unwrap();
        if !match_.is_init {
            // Handle error: match not initialized
            panic!("Match not initialized");
        }
        if match_.owner != self.env.caller() {
            // Handle error: only owner can start the game
            panic!("Only owner can start the game");
        }
        match_.is_started = true;

        // Emit equivalent log statement
        // log::info!("Match started: id={}", match_id);
        Ok(())
    }

    fn watch_game(&self, match_id: u64) -> Result<(), Vec<u8>> {
        let match_ = self.state.matches.get(&match_id);
        if match_.is_none() {
            // Handle error: match not found
            panic!("Match not initialized");
        }
        let match_ = match_.unwrap();
        if !match_.is_init {
            // Handle error: match not initialized
            panic!("Match not initialized");
        }

        // Provide access to relevant match information (details, etc.)
        // ...

        // Emit equivalent log statement
        // log::info!("User viewed match: id={}", match_id);
        Ok(())
    }

    fn create_question(&mut self, match_id: u64, yn_question: String) -> Result<(), Vec<u8>> {
        let match_ = self.state.matches.get_mut(&match_id);
        if match_.is_none() {
            // Handle error: match not found
            panic!("Match not initialized");
        }
        let match_ = match_.unwrap();
        if !match_.is_init {
            // Handle error: match not initialized
            panic!("Match not initialized");
        }
        if match_.owner == self.env.caller() {
            // Handle error: owner cannot create questions
            panic!("Owner cannot create questions");
        }

        let current_question_id = match_.next_question_id;
        match_.questions.insert(
            current_question_id,
            Question {
                is_ended: false,
                yn_question,
                total_staked: 0,
            },
        );
        match_.next_question_id += 1;

        // Emit equivalent log statement
        // log::info!(
        //     "Question created: match_id={}, question_id={}",
        //     match_id,
        //     current_question_id
        // );
        Ok(())
    }

    fn answer_question(&mut self, match_id: u64, question_id: u64, answer: bool) -> Result<(), Vec<u8>> {
        let match_ = self.state.matches.get_mut(&match_id);
        if match_.is_none() {
            // Handle error: match not found
            panic!("Match not initialized");
        }
        let match_ = match_.unwrap();
        if !match_.is_init {
            // Handle error: match not initialized
            panic!("Match not initialized");
        }
        if match_.questions.get(&question_id).is_none() {
            // Handle error: question not found
            panic!("Question not found");
        }
        let question = match_.questions.get_mut(&question_id).unwrap();
        if question.is_ended {
            // Handle error: question already ended
            panic!("Question already ended");
        }

        // Check if the user has already answered the question
        let has_answered = question.answers.get(&self.env.caller()).is_some();
        if has_answered {
            // Handle error: user already answered
            panic!("User already answered this question");
        }

        question.answers.insert(self.env.caller(), answer);
        // Handle staking or token transfer logic here

        // Emit equivalent log statement
        // log::info!(
        //     "User answered question: match_id={}, question_id={}",
        //     match_id,
        //     question_id
        // );
        Ok(())
    }

    fn end_question(&mut self, match_id: u64, question_id: u64) -> Result<(), Vec<u8>> {
        let match_ = self.state.matches.get_mut(&match_id);
        if match_.is_none() {
            // Handle error: match not found
            panic!("Match not initialized");
        }
        let match_ = match_.unwrap();
        if !match_.is_init {
            // Handle error: match not initialized
            panic!("Match not initialized");
        }
        if match_.owner != self.env.caller() {
            // Handle error: only owner can end questions
            panic!("Only owner can end questions");
        }

        let question = match_.questions.get_mut(&question_id).unwrap();
        question.is_ended = true;

        // Emit equivalent log statement
        // log::info!(
        //     "Question ended: match_id={}, question_id={}",
        //     match_id,
        //     question_id
        // );
        Ok(())

        // Implement additional logic for ending a question here
    }

    fn declare_answer(&mut self, match_id: u64, question_id: u64, correct_answer: bool) -> Result<(), Vec<u8>> {
        let match_ = self.state.matches.get_mut(&match_id);
        if match_.is_none() {
            // Handle error: match not found
            panic!("Match not initialized");
        }
        let match_ = match_.unwrap();
        if !match_.is_init {
            // Handle error: match not initialized
            panic!("Match not initialized");
        }
        if match_.owner != self.env.caller() {
            // Handle error: only owner can declare answers
            panic!("Only owner can declare answers");
        }

        let question = match_.questions.get_mut(&question_id).unwrap();
        question.correct_answer = correct_answer;

        // Emit equivalent log statement
        // log::info!(
        //     "Answer declared: match_id={}, question_id={}, correct_answer={}",
        //     match_id,
        //     question_id,
        //     correct_answer
        // );
        Ok(())

        // Implement additional logic for declaring an answer here
    }

    fn payout_winners(&mut self, match_id: u64, question_id: u64) -> Result<(), Vec<u8>> {
        let match_ = self.state.matches.get_mut(&match_id);
        if match_.is_none() {
            // Handle error: match not found
            panic!("Match not initialized");
        }
        let match_ = match_.unwrap();
        if !match_.is_init {
            // Handle error: match not initialized
            panic!("Match not initialized");
        }
        if !match_.questions[&question_id].is_ended {
            // Handle error: question not ended
            panic!("Question not ended");
        }

        let question = &match_.questions[&question_id];

        // Emit equivalent log statement
        // log::info!(
        //     "Payout winners: match_id={}, question_id={}",
        //     match_id,
        //     question_id
        // );
        Ok(())

    }
}
