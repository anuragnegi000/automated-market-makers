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
    pub config:Account<'info,Config>,

    #[account(
        mut,
        seeds=[b"lp",config.key().as_ref()],
        bump=config.lp_bump
    )]
    pub mint_lp:Account<'info,Mint>,

    #[account(
        mut,
        associated_token::mint=mint_a,
        associated_token::authority=config
    )]
    pub vault_a:Account<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint=mint_b,
        associated_token::authority=config
    )]
    pub vault_b:Account<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint=mint_a,
        associated_token::authority=signer
    )]
    pub user_token_account_a:Account<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint=mint_b,
        associated_token::authority=signer
    )]
    pub user_token_account_b:Account<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint=mint_lp,
        associated_token::authority=signer
    )]
    pub user_token_account_lp:Account<'info,TokenAccount>,

    pub associated_token_program:Program<'info,AssociatedToken>,
    pub token_program:Program<'info,Token>,
    pub system_program:Program<'info,Program>

}