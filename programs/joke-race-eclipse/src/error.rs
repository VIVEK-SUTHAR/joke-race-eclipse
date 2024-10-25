use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    AlreadyVoted,
    VotingEnded,
    Unauthorized,
}
