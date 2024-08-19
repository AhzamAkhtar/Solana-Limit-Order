use anchor_lang::prelude::*;
pub mod state;
pub use state::*;

pub mod context;
pub use context::*;

declare_id!("CAxcPk6uf5a92YswtUVB94LCWD3sUJinpYJ8qLdpPCZA");

#[program]
pub mod cega {

    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.init(&ctx.bumps, seed, authority)
    }

    pub fn transfer_token_to_vault(ctx: Context<TransferTokenToVault>, amount: u64) -> Result<()> {
        ctx.accounts.transfer_token(amount)
    }

    pub fn transfer_token_to_buyer(ctx: Context<TransferBuyer>, amount: u64) -> Result<()> {
        ctx.accounts.send_to_buyer(amount)
    }
}
