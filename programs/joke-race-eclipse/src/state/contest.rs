use crate::constants::ANCHOR_DISCRIMINATOR;
use anchor_lang::prelude::*;

#[account]
pub struct Contest {
    pub id: u64,
    pub metadata_uri: String,
    pub upvotes: u64,
    pub author: Pubkey,
    pub start_time: i64,
    pub end_time: i64,
}

impl Contest {
    pub const SPACE: usize = ANCHOR_DISCRIMINATOR +
        8 + //8 for id u64
        256 + //256 For String(Max 256 Chars)
        8 +  //8 Bytes for upvotes (u64)
        32 + //32 Bytes for PubKey
        8 + //8 Bytes for start_time(i64)
        8; //8 Bytes for end_time(i64)
}
