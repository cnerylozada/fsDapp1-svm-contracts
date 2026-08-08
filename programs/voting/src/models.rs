use anchor_lang::prelude::*;

pub const MAX_POLL_LENGTH: usize = 25;
pub const MAX_DESCRIPTION_LENGTH: usize = 40;
pub const MAX_CANDIDATE_LENGTH: usize = 15;

#[account]
#[derive(InitSpace)]
pub struct Poll {
    pub id: Pubkey,
    #[max_len(MAX_POLL_LENGTH)]
    pub poll: String,
    #[max_len(MAX_DESCRIPTION_LENGTH)]
    pub description: String,
    pub poll_start: u64,
    pub poll_end: u64,
    pub candidate_amount: u64,
    pub bump: u8,
    #[max_len(2)]
    pub candidate_pda_list: Vec<Pubkey>,
}

#[account]
#[derive(InitSpace)]
pub struct Candidate {
    #[max_len(MAX_CANDIDATE_LENGTH)]
    pub name: String,
    pub votes: u64,
}
