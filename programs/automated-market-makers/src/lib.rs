use anchor_lang::prelude::*;

declare_id!("Ahb2NPqt6wqcLJydV2asqydAuctHPAFhUskXDzYSF61x");

#[program]
pub mod automated_market_makers {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
