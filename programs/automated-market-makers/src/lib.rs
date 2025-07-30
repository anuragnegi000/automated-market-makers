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
    pub fn deposit(
        ctx:Context<Deposit>,
        amount:u64,
        max_a:u64,
        max_b:u64
    )->Result<()>{
        ctx.accounts.deposit(amount, max_a, max_b)
    }
    pub fn swap(
        ctx:Context<Swap>,
        is_x:bool,
        amount_in:u64,
        min_amount_out:u64
    )->Result<()>{
        ctx.accounts.swap(is_x, amount_in, min_amount_out)
    }
    pub fn withdraw(
        ctx:Context<Withdraw>,
        amount:u64,
        min_a:u64,
        min_b:u64
    )->Result<()>{
        ctx.accounts.withdraw(amount, min_a, min_b)
    }
}
