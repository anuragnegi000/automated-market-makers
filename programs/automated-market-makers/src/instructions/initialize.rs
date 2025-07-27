use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint,Token,TokenAccount}
};

use crate::states::Config;

#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(mut)]
    pub owner:Signer<'info>,

    pub mint_a:Account<'info,Mint>,
    pub mint_b:Account<'info,Mint>,

    #[account(
        init,
        payer=owner,
        seeds=[b"config"],
        bump,
        space=Config::INIT_SPACE
    )]
    pub config:Account<'info,Config>,

    #[account(
        init,
        payer=owner,
        mint::decimals=6,
        mint::authority=config.key(),
        seeds=[b"lp",config.key().as_ref()],
        bump
    )]
    pub mint_lp:Account<'info,Mint>,

    #[account(
        init,
        payer=owner,
        associated_token::mint=mint_a,
        associated_token::authority=config
    )]
    pub vault_a:Account<'info,TokenAccount>,

    #[account(
        init,
        payer=owner,
        associated_token::mint=mint_b,
        associated_token::authority=config
    )]
    pub vault_b:Account<'info,TokenAccount>,
    pub token_program:Program<'info,Token>,
    pub associated_token_program:Program<'info,AssociatedToken>,
    pub system_program:Program<'info,System>
}


impl <'info>Initialize<'info>{
    pub fn initialize(
        &mut self,
        fee:u16,
        authority:Option<Pubkey>,
        bumps:&InitializeBumps,
    )->Result<()>{
        self.config.set_inner(Config{
            authority,
            mint_a:self.mint_a.key(),
            mint_b:self.mint_b.key(),
            config_bump:bumps.config,
            lp_bump:bumps.mint_lp,
            fee
        });
        Ok(())
    }
}
