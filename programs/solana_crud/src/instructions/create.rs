use crate::constants::{ACCOUNT_DISCRIMINATOR, MESSAGE_ACCOUNT_TAG};
use crate::errors::MessageAccountError;
use crate::models::MessageAccount;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(_title: String)]
pub struct Create<'info> {
    #[account(
        init,
        space = ACCOUNT_DISCRIMINATOR + MessageAccount::INIT_SPACE,
        payer = signer,
        seeds = [MESSAGE_ACCOUNT_TAG, signer.key().as_ref(), _title.as_bytes()],
        bump
    )]
    message_account: Account<'info, MessageAccount>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

pub fn handler(_ctx: Context<Create>, _title: String, _message: String) -> Result<()> {
    if _title.is_empty() {
        return Err(MessageAccountError::TitleIsRequired.into());
    }
    if _title.len() > 15 {
        return Err(MessageAccountError::TitleTooLong.into());
    }

    if _message.is_empty() {
        return Err(MessageAccountError::MessageIsRequired.into());
    }
    if _message.len() > 20 {
        return Err(MessageAccountError::MesageTooLong.into());
    }

    let message_account = &mut _ctx.accounts.message_account;

    message_account.user = _ctx.accounts.signer.key();
    message_account.title = _title;
    message_account.message = _message;
    message_account.bump = _ctx.bumps.message_account;

    Ok(())
}
