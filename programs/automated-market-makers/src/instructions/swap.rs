use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked,Mint,Token,TokenAccount,TransferChecked}
};

use constant_product_curve::{ConstantProduct,LiquidityPair};

use crate::errors::AmmError;
use crate::states::Config;

#[derive(Accounts)]
pub struct Swap<'info>{
    #[account(mut)]
    pub user:Signer<'info>,

    pub mint_a:Account<'info,Mint>,
    pub mint_b:Account<'info,Mint>,

    #[account(
        mut,
        seeds=[b"config"],
        bump=config.config_bump,
        has_one=mint_a,
        has_one=mint_b
    )]
    pub config:Account<'info,Config>,

    #[account(
        seeds=[b"lp",config.key().as_ref],
        bump=config.lp_bump
    )]
    pub mint_lp:Account<'info,Mint>,

    #[account(
        mut,
        associated_token::mint=mint_a,
        associated_token::authority=config
    )]
    pub vault_a:Account<'info,Mint>,

    #[account(
        mut,
        associated_token::mint=mint_b,
        associated_token::authority=config
    )]
    pub vault_b:Account<'info,Mint>,

    #[account(
        mut,
        associated_token::mint=mint_a,
        associated_token::authority=signer
    )]
    pub user_token_account_a:Account<'info,Mint>,

    #[account(
        mut,
        associated_token::mint=mint_b,
        associated_token::authority=signer
    )]
    pub user_token_account_b:Account<'info,Mint>,

    pub system_program:Program<'info,Program>,
    pub associated_token_program:Program<'info,AssociatedToken>,
    pub token_program:Program<'info,Token>
}