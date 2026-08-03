use anchor_lang::prelude::*;
mod constants;
mod errors;
mod instructions;
mod models;

use instructions::*;

declare_id!("5Zy3JjtfHLug3EXWWST4vaCdsG2JYhvbug8r5Xk6RCGt");

#[program]
pub mod solana_crud {
    use super::*;

    pub fn create(_ctx: Context<Create>, _title: String, _message: String) -> Result<()> {
        create::handler(_ctx, _title, _message)
    }

    pub fn update(_ctx: Context<Update>, _title: String, _message: String) -> Result<()> {
        update::handler(_ctx, _title, _message)
    }

    pub fn delete(_ctx: Context<Delete>, _title: String) -> Result<()> {
        delete::handler(_ctx, _title)
    }
}
