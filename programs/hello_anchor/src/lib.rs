use anchor_lang::prelude::*;

mod constants;
mod instructions;
mod models;
use instructions::*;
declare_id!("4M5JG4Sq3jFMKMTxK7KLRqU3g2fozYLn2LJGCj4ZjSF7");

#[program]
mod hello_anchor {
    use super::*;
    pub fn initialize_account(_ctx: Context<Initialize>, _name: String, _age: u64) -> Result<()> {
        instructions::initialize_account(_ctx, _name, _age)
    }
}
