use anchor_lang::prelude::*;

declare_id!("AyRNVdUiPGJdATnfV2CHkMVgkU3n3PgFatDBCCubW7fG");

#[program]
pub mod nft_swap_pool {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
