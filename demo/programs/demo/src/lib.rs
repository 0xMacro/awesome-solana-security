use anchor_lang::prelude::*;

declare_id!("2tnoRcFtuFybTubwZf1hJpMbLaCoK3mhcFVHWuPVtNFL");

#[program]
pub mod demo {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
