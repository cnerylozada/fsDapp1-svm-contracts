use anchor_lang::prelude::*;

#[error_code]
pub enum MessageAccountError {
    #[msg("Title is required")]
    TitleIsRequired,
    #[msg("Title too long")]
    TitleTooLong,

    #[msg("Message is required")]
    MessageIsRequired,
    #[msg("Message too long")]
    MesageTooLong,
}
