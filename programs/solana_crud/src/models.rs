use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MessageAccount {
    pub user: Pubkey,

    #[max_len(15)]
    pub title: String,

    #[max_len(20)]
    pub message: String,

    pub bump: u8,
}

impl MessageAccount {
    pub fn set_message(&mut self, _message: String) {
        self.message = _message;
    }
}
