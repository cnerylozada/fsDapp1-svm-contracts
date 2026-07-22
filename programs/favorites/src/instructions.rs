use crate::models::Favorites;
use anchor_lang::prelude::*;

const ACCOUNT_DISCRIMINATOR: usize = 8;

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(init, payer = signer,
        space = ACCOUNT_DISCRIMINATOR + Favorites::INIT_SPACE,
        seeds = [b"favorites", signer.key().as_ref()], bump
    )]
    favorites: Account<'info, Favorites>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

pub fn set_favorites(
    _ctx: Context<SetFavorites>,
    _number: u64,
    _color: String,
    _hobbies: Vec<String>,
) -> Result<()> {
    let new_account = &mut _ctx.accounts.favorites;
    new_account.number = _number;
    new_account.color = _color;
    new_account.hobbies = _hobbies;
    Ok(())
}
