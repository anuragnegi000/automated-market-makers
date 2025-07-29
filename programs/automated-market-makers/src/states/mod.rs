use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]

pub struct Config{
    pub authority:Option<Pubkey>,
    pub mint_a:Pubkey,
    pub mint_b:Pubkey,
    pub mint_lp:Pubkey,
    pub fee:u16,
    pub config_bump:u8,
    pub lp_bump:u8
}