use crate::state::Config;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

#[derive(Accounts)]
pub struct TransferBuyer<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    /// CHECK:
    pub seller: AccountInfo<'info>,

    pub mint_x: Box<InterfaceAccount<'info, Mint>>,
    pub mint_usdc: Box<InterfaceAccount<'info, Mint>>,

    #[account(
    mut,
    associated_token::mint = mint_x,
    associated_token::authority = auth
    )]
    pub vault_x: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
    init_if_needed,
    payer = buyer,
    associated_token::mint = mint_x,
    associated_token::authority = buyer
    )]
    pub buyer_vault_x: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = mint_usdc,
        associated_token::authority = buyer
    )]
    pub buyer_vault_usdc: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = mint_usdc,
        associated_token::authority = seller
    )]
    pub seller_vault_usdc: Box<InterfaceAccount<'info, TokenAccount>>,

    ///CHECKED: This is not dangerous. It's just used for signing.
    #[account(
    seeds = [b"auth"],
    bump = config.auth_bump
    )]
    pub auth: UncheckedAccount<'info>,

    #[account(
    seeds = [
    b"config",
    config.seed.to_le_bytes().as_ref()
    ],
    bump = config.config_bump,
    )]
    pub config: Box<Account<'info, Config>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> TransferBuyer<'info> {
    pub fn transfer_token_to_buyer_and_make_trade(&mut self, amount: u64) -> Result<()> {
        self.check_expiry()?;
        self.send_to_buyer(amount)?;
        self.send_usdc_to_seller(amount)
    }

    pub fn check_expiry(&self) -> Result<()> {
        self.config.check_expiry()
    }

    pub fn send_to_buyer(&mut self, amount: u64) -> Result<()> {
        let cpi_accounts = TransferChecked {
            from: self.vault_x.to_account_info(),
            mint: self.mint_x.to_account_info(),
            to: self.buyer_vault_x.to_account_info(),
            authority: self.auth.to_account_info(),
        };

        let seeds = &[&b"auth"[..], &[self.config.auth_bump]];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        transfer_checked(ctx, amount, self.mint_x.decimals)
    }

    pub fn send_usdc_to_seller(&mut self, amount: u64) -> Result<()> {
        let amount = amount * self.config.price;

        let cpi_accounts = TransferChecked {
            from: self.buyer_vault_usdc.to_account_info(),
            mint: self.mint_usdc.to_account_info(),
            to: self.seller_vault_usdc.to_account_info(),
            authority: self.buyer.to_account_info(),
        };

        let ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        transfer_checked(ctx, amount, self.mint_usdc.decimals)
    }
}
