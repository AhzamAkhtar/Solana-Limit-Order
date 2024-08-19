use crate::state::Config;
use crate::errors::CustomErrors;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

#[derive(Accounts)]

pub struct Cancel<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    pub mint_x: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = seller,
        associated_token::mint = mint_x,
        associated_token::authority = seller
    )]
    pub seller_vault_x: Box<InterfaceAccount<'info, TokenAccount>>,

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

impl<'info> Cancel<'info> {

    pub fn cancel(&mut self) -> Result<()> {

        require!(
            self.config.amount == self.config.amount_preserve,
            CustomErrors::CannotClose
        );

        self.send_to_seller();
        self.close_user_vault()
    }

    pub fn send_to_seller(&mut self) -> Result<()> {
        let cpi_accounts = TransferChecked {
            from: self.vault_x.to_account_info(),
            mint: self.mint_x.to_account_info(),
            to: self.seller_vault_x.to_account_info(),
            authority: self.auth.to_account_info(),
        };

        let seeds = &[&b"new_auth"[..], &[self.config.auth_bump]];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        transfer_checked(ctx, self.config.amount, self.mint_x.decimals)
    }

    pub fn close_user_vault(&mut self) -> Result<()> {
        let cpi_account = CloseAccount {
            account: self.vault_x.to_account_info(),
            destination: self.seller.to_account_info(),
            authority: self.auth.to_account_info(),
        };

        let seeds = &[&b"new_auth"[..], &[self.config.auth_bump]];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_account,
            signer_seeds,
        );

        close_account(ctx)
    }
}
