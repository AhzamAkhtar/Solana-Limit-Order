use crate::errors::CustomErrors;
use anchor_lang::prelude::*;
#[account]

pub struct Config {
    pub seed: u64,                 //8
    pub authority: Option<Pubkey>, //(1+32)
    pub mint_x: Pubkey,            //32
    pub auth_bump: u8,             //1
    pub config_bump: u8,           //1
    pub expiry: u64,               //8
    pub amount: u64,               //8
    pub price: u64,                //8
    pub amount_preserve: u64,      //8
}

impl Config {
    pub const LEN: usize = 8 + 8 + (1 + 32) + 32 + (2 * 1) + (4 * 8);

    pub fn check_expiry(&self) -> Result<()> {
        msg!("Checking expiry: {}", self.expiry);
        require!(
            self.expiry > Clock::get()?.slot,
            CustomErrors::TimeHasExpired
        );
        Ok(())
    }

    pub fn set_expiry(&mut self, expiry: u64) -> Result<()> {
        let current_slot = Clock::get()?.slot;
        msg!("Current Slot{} - EXP Time{}", current_slot, expiry);
        let new_expiry = if expiry > 0 { current_slot + expiry } else { 0 };
        self.expiry = new_expiry;

        Ok(())
    }
}
