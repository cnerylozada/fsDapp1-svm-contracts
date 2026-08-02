use crate::constants::MESSAGE_ACCOUNT_TAG;
use crate::models::MessageAccount;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(_title: String)]
pub struct Update<'info> {
    #[account(
        seeds = [MESSAGE_ACCOUNT_TAG, signer.key().as_ref(), _title.as_bytes()],
        bump = message_account.bump,
        mut
    )]
    message_account: Account<'info, MessageAccount>,

    #[account(mut)]
    signer: Signer<'info>,
}

pub fn handler(_ctx: Context<Update>, _title: String, _message: String) -> Result<()> {
    let message_account = &mut _ctx.accounts.message_account;
    message_account.message = _message;

    Ok(())
}
