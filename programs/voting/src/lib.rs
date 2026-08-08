use anchor_lang::prelude::*;

mod constants;
mod errors;
mod instructions;
mod models;

use instructions::*;

declare_id!("5RoyCHwwWp1o2i7bu82tCPfdCmekgv5XrANupP7o8AGN");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_poll(
        _context: Context<InitializePoll>,
        _poll: String,
        _description: String,
        _poll_end_in_seconds: u64,
    ) -> Result<()> {
        initialize_poll::initialize_poll(_context, _poll, _description, _poll_end_in_seconds)
    }

    pub fn initialize_candidate(
        _context: Context<InitializeCandidate>,
        _poll: String,
        _name: String,
    ) -> Result<()> {
        initialize_candidate::initialize_candidate(_context, _poll, _name)
    }

    pub fn vote(_context: Context<Vote>, _poll: String, _candidate_name: String) -> Result<()> {
        vote::vote(_context, _poll, _candidate_name)
    }
}
