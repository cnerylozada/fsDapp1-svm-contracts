use anchor_lang::prelude::*;

use crate::constants::ACCOUNT_DISCRIMINATOR;
use crate::errors::VotingError;
use crate::models::*;

#[derive(Accounts)]
#[instruction(_poll: String)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, payer = signer, space = ACCOUNT_DISCRIMINATOR + Poll::INIT_SPACE,
    seeds = [b"poll_account", signer.key().as_ref(), _poll.as_bytes().as_ref()], bump)]
    pub poll_account: Account<'info, Poll>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_poll(
    _context: Context<InitializePoll>,
    _poll: String,
    _description: String,
    _poll_end_in_seconds: u64,
) -> Result<()> {
    if _poll.len() > MAX_POLL_LENGTH {
        return Err(VotingError::PollTooLong.into());
    }

    if _description.len() > MAX_DESCRIPTION_LENGTH {
        return Err(VotingError::DescriptionTooLong.into());
    }

    let poll_account = &mut _context.accounts.poll_account;
    poll_account.id = poll_account.key();

    poll_account.poll = _poll;
    poll_account.description = _description;
    poll_account.candidate_amount = 0;
    poll_account.bump = _context.bumps.poll_account;

    let now = (Clock::get().unwrap().unix_timestamp) as u64;
    poll_account.poll_start = now;
    poll_account.poll_end = now + _poll_end_in_seconds;

    Ok(())
}
