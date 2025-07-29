use anchor_lang::prelude::*;

declare_id!("Ahb2NPqt6wqcLJydV2asqydAuctHPAFhUskXDzYSF61x");

pub mod instructions;
pub mod states;
pub mod errors;

pub use instructions::*;
pub use states::*;
pub use errors::*;

#[program]
pub mod automated_market_makers {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        fee: u16,
        authority: Option<Pubkey>
    ) -> Result<()> {
        ctx.accounts.initialize(fee, authority, &ctx.bumps)
    }
}
