use crate::constants::ANCHOR_DISCRIMINATOR;
use anchor_lang::prelude::*;
#[account]
pub struct Vault {
    pub authority: Pubkey,
}

impl Vault {
    //PubKey Requires 32 Bytes
    pub const SPACE: usize = ANCHOR_DISCRIMINATOR + 32;
}
