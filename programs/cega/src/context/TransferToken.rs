use crate::Config;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::TokenInterface;
use anchor_spl::token_interface::{transfer_checked, Mint, TokenAccount, TransferChecked};

#[derive(Accounts)]
pub struct TransferTokenToVault<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint_x: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_x,
        associated_token::authority = user
    )]
    pub user_vault_x: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = auth
    )]
    pub vault_x: Box<InterfaceAccount<'info, TokenAccount>>,

    ///CHECKED: This is not dangerous. It's just for signing.
    #[account(
        seeds = [b"auth"],
        bump = config.auth_bump
    )]
    pub auth: UncheckedAccount<'info>,

    #[account(
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    pub config: Box<Account<'info, Config>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> TransferTokenToVault<'info> {
    pub fn transfer_token(&mut self, amount: u64) -> Result<()> {
        let cpi_accounts = TransferChecked {
            from: self.user_vault_x.to_account_info(),
            mint: self.mint_x.to_account_info(),
            to: self.mint_x.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        transfer_checked(ctx, amount, self.mint_x.decimals)
    }
}
