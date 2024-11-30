use crate::constants::ANCHOR_DISCRIMINATOR;
use anchor_lang::prelude::*;
#[account]
pub struct ContestCounter {
    pub count: u64,
}

impl ContestCounter {
    //u64 requires 8 Bytes
    pub const SPACE: usize = ANCHOR_DISCRIMINATOR + 8;
}
