use anchor_lang::prelude::*;

declare_id!("CAxcPk6uf5a92YswtUVB94LCWD3sUJinpYJ8qLdpPCZA");

#[program]
pub mod cega {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
