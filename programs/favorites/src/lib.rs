use anchor_lang::prelude::*;
mod instructions;

mod models;

use instructions::*;

declare_id!("3V15teoWR8bvuTrDdBSWVfVAzkrPfYjVRND52NSEvzBL");

#[program]
pub mod favorites {

    use super::*;

    pub fn set_favorites(
        _context: Context<SetFavorites>,
        _number: u64,
        _color: String,
        _hobbies: Vec<String>,
    ) -> Result<()> {
        instructions::set_favorites(_context, _number, _color, _hobbies)
    }
}
