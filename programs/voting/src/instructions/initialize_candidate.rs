use anchor_lang::prelude::*;

use crate::constants::ACCOUNT_DISCRIMINATOR;
use crate::errors::VotingError;
use crate::models::*;

#[derive(Accounts)]
#[instruction(_poll: String, _name: String)]
pub struct InitializeCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, payer = signer, space = ACCOUNT_DISCRIMINATOR + Candidate::INIT_SPACE,
    seeds = [b"candidate_account", signer.key().as_ref(), 
    _poll.as_bytes().as_ref(), _name.as_bytes().as_ref()], bump)]
    pub candidate_account: Account<'info, Candidate>,

    #[account(mut, seeds = [b"poll_account", signer.key().as_ref(), _poll.as_bytes().as_ref()],
    bump = poll_account.bump)]
    pub poll_account: Account<'info, Poll>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_candidate(
    _context: Context<InitializeCandidate>,
    _poll: String,
    _name: String,
) -> Result<()> {
    require!(_name.len() <= MAX_CANDIDATE_LENGTH, VotingError::NameTooLong);

    let poll_account = &mut _context.accounts.poll_account;
    poll_account.candidate_amount += 1;

    let candidate_account = &mut _context.accounts.candidate_account;
    candidate_account.name = _name;
    candidate_account.votes = 0;
    poll_account
        .candidate_pda_list
        .push(candidate_account.key());

    Ok(())
}
