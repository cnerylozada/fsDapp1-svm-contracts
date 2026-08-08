use anchor_lang::prelude::*;

use crate::models::*;

#[derive(Accounts)]
#[instruction(_poll: String, _candidate_name: String)]
pub struct Vote<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, seeds = [b"candidate_account", signer.key().as_ref(), 
    _poll.as_bytes().as_ref(), _candidate_name.as_bytes().as_ref()], bump)]
    pub candidate_account: Account<'info, Candidate>,

    pub system_program: Program<'info, System>,
}

pub fn vote(_context: Context<Vote>, _poll: String, _candidate_name: String) -> Result<()> {
    let candidate_account = &mut _context.accounts.candidate_account;
    candidate_account.votes += 1;

    Ok(())
}
