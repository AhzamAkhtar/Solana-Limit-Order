use crate::state::Config;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::TokenInterface;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount},
};

#[derive(Accounts)]
#[instruction(seed:u64)]

pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_x: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        init,
        payer = user,
        associated_token::mint = mint_x,
        associated_token::authority = auth
    )]
    pub vault_x: Box<InterfaceAccount<'info, TokenAccount>>,
    ///CHECKED: This is not dangerous. It's just used for signing.
    #[account(
        seeds = [b"auth"],
        bump
    )]
    pub auth: UncheckedAccount<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"config", seed.to_le_bytes().as_ref()],
        bump,
        space = Config::LEN
    )]
    pub config: Box<Account<'info, Config>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn init(
        &mut self,
        bumps: &InitializeBumps,
        seed: u64,
        expiry: u64,
        authority: Option<Pubkey>,
        amount: u64,
        price: u64,
    ) -> Result<()> {
        self.config.set_inner(Config {
            seed: seed,
            expiry: expiry,
            authority: authority,
            mint_x: self.mint_x.key(),
            auth_bump: bumps.auth,
            config_bump: bumps.config,
            price: price,
            amount: amount,
        });

        Ok(())
    }
}
