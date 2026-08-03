use crate::constants::MESSAGE_ACCOUNT_TAG;
use crate::models::MessageAccount;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(_title: String)]
pub struct Delete<'info> {
    #[account(
        seeds = [MESSAGE_ACCOUNT_TAG, signer.key().as_ref(), _title.as_bytes()],
        bump = message_account.bump,
        mut,
        close = signer
    )]
    message_account: Account<'info, MessageAccount>,

    #[account(mut)]
    signer: Signer<'info>,
}

pub fn handler(_ctx: Context<Delete>, _title: String) -> Result<()> {
    Ok(())
}
