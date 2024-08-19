use crate::errors::CustomErrors;
use anchor_lang::prelude::*;
#[account]

pub struct Config {
    pub seed: u64,
    pub authority: Option<Pubkey>,
    pub mint_x: Pubkey,
    pub auth_bump: u8,
    pub config_bump: u8,
    pub expiry: u64,
    pub amount : u64,
    pub price : u64,
    pub amount_preserve : u64,
}

impl Config {
    pub const LEN: usize = 8 + 8 + (1 + 32) + 32 + (2 * 1);

    pub fn check_expiry(&self) -> Result<()> {
        require!(
            self.expiry > Clock::get()?.slot,
            CustomErrors::TimeHasExpired
        );
        Ok(())
    }

}
