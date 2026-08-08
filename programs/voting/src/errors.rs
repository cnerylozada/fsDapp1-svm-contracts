use anchor_lang::prelude::*;

#[error_code]
pub enum VotingError {
    #[msg("Poll name too long")]
    PollTooLong,
    #[msg("Poll description too long")]
    DescriptionTooLong,
    #[msg("Name too long")]
    NameTooLong,
}
