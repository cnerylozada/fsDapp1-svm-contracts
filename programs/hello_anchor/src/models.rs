use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct NewAccount {
    #[max_len(10)]
    pub name: String,
    pub age: u64,
    pub likes: u64,
}
