use anchor_lang::prelude::*;
pub mod state;
pub use state::*;

pub mod context;
pub use context::*;

mod errors;
pub use errors::CustomErrors;

declare_id!("CAxcPk6uf5a92YswtUVB94LCWD3sUJinpYJ8qLdpPCZA");

#[program]
pub mod cega {

    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        expiry: u64,
        authority: Option<Pubkey>,
        amount: u64,
        price: u64,
    ) -> Result<()> {
        ctx.accounts
            .init(&ctx.bumps, seed, expiry, authority, amount, price)
    }

    pub fn transfer_token_to_vault(ctx: Context<TransferTokenToVault>) -> Result<()> {
        ctx.accounts.transfer_token()
    }

    pub fn transfer_token_to_buyer(ctx: Context<TransferBuyer>, amount: u64) -> Result<()> {
        ctx.accounts.transfer_token_to_buyer_and_make_trade(amount)
    }

    pub fn update(ctx: Context<Update>, new_price: u64, new_expiry: u64) -> Result<()> {
        ctx.accounts.update(new_price, new_expiry)
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        ctx.accounts.cancel()
    }
}
