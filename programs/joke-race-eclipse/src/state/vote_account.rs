use crate::constants::ANCHOR_DISCRIMINATOR;
use anchor_lang::prelude::*;
#[account]
pub struct VoterContestRecord {
    pub contest_id: u64,
    pub contestant_id: u64,
    pub voter: Pubkey,
    pub total_votes: u64,
}

impl VoterContestRecord {
    pub const SPACE: usize = ANCHOR_DISCRIMINATOR +
        8 + //8 Bytes for contest_id (u64)
        8 + //8 Bytes for contestant_id(u64)
        32 + //32 Bytes for PubKey
        8; //8 Bytes for total_votes(u64)
}
