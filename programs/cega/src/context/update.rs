use crate::state::Config;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::TokenInterface;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount},
};

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(
        seeds = [
        b"config",
        config.seed.to_le_bytes().as_ref()
        ],
        bump = config.config_bump,
        )]
    pub config: Box<Account<'info, Config>>,
}

impl<'info> Update<'info> {
    pub fn update(&mut self, new_price: u64 , new_expiry : u64) -> Result<()> {
        self.config.price = new_price;
        self.config.expiry = new_expiry;

        Ok(())
    }
}
