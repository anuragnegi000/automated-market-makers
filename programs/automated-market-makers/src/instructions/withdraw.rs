use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount,transfer_checked,Burn,burn,TransferChecked},
};


#[derive(Accounts)]
pub struct Withdraw<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,

    pub mint_a:Account<'info,Mint>,
    pub mint_b:Account<'info,Mint>,

    #[account(
        mut,
        seeds=[b"config"],
        bump=config.config_bump,
        has_one=mint_a,
        has_one=mint_b
    )]

}