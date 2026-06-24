use crate::constants::ACCOUNT_DISCRIMINATOR;
use crate::models::NewAccount;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer,
        space = ACCOUNT_DISCRIMINATOR + NewAccount::INIT_SPACE)]
    new_account: Account<'info, NewAccount>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

pub fn initialize_account(_ctx: Context<Initialize>, _name: String, _age: u64) -> Result<()> {
    let new_account = &mut _ctx.accounts.new_account;

    new_account.name = _name;
    new_account.age = _age;
    Ok(())
}
