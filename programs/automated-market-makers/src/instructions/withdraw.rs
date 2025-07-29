use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount,transfer_checked,Burn,burn,TransferChecked},
};


use constant_product_curve::ConstantProduct;
use crate::errors::AmmError;
use crate::states::Config;

#[derive(Accounts)]
pub struct Withdraw<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,

    pub mint_a:Account<'info,Mint>,
    pub mint_b:Account<'info,Mint>,

    #[account(
        mut,
        seeds=[b"config",mint_a.key().as_ref(),mint_b.key().as_ref()],
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
    pub system_program:Program<'info,System>

}

impl <'info> Withdraw <'info>{
    pub fn withdraw(&mut self,amount:u64,min_a:u64,min_b:u64)->Result<()>{
        require!(amount!=0,AmmError::InvalidAmount);
        let (a , b)=match self.mint_lp.supply==0
        && self.vault_a.amount==0
        && self.vault_b.amount==0
        {
            true=>(min_a,min_b),
            false=>{
                let amount=ConstantProduct::xy_deposit_amounts_from_l(
                    self.vault_a.amount,
                    self.vault_b.amount,
                    self.mint_lp.supply,
                    amount,
                    6
                ).unwrap();
                (amount.x,amount.y)
            }
        };
        require!(a>=min_a && b>=min_b,AmmError::SlippageExceeded);
        self.burn_lp(amount);
        self.withdraw_token_a(a);
        self.withdraw_token_b(b);
        Ok(())
    }

    pub fn burn_lp(&mut self,amount:u64)->Result<()>{
        let cpi_program=self.token_program.to_account_info();
        let cpi_accounts=Burn{
            mint:self.mint_lp.to_account_info(),
            from:self.user_token_account_lp.to_account_info(),
            authority:self.signer.to_account_info()
        };
        let mint_a_key=self.mint_a.key();
        let mint_b_key=self.mint_b.key();
        let signer_seeds:&[&[&[u8]]]=&[&[
            b"config",
            mint_a_key.as_ref(),
            mint_b_key.as_ref(),
            &[self.config.config_bump],
        ]];

        let cpi_ctx=CpiContext::new_with_signer(cpi_program,cpi_accounts,signer_seeds);
        burn(cpi_ctx,amount);
        Ok(())
    }

    pub fn withdraw_token_a(&mut self,amount:u64)->Result<()>{
        let cpi_program=self.token_program.to_account_info();
        let cpi_accounts=TransferChecked{
            from:self.vault_a.to_account_info(),
            to:self.user_token_account_a.to_account_info(),
            mint:self.mint_a.to_account_info(),
            authority:self.config.to_account_info()
        };
        let decimals=self.mint_a.decimals;
        let mint_a_key=self.mint_a.key();
        let mint_b_key=self.mint_b.key();
        let signer_seeds:&[&[&[u8]]]=&[&[
            b"config",
            mint_a_key.as_ref(),
            mint_b_key.as_ref(),
            &[self.config.config_bump],
        ]];
        let cpi_ctx=CpiContext::new_with_signer(cpi_program,cpi_accounts,signer_seeds);
        transfer_checked(cpi_ctx,amount,decimals);
        Ok(())
    }
    pub fn withdraw_token_b(&mut self,amount:u64)->Result<()>{
        let cpi_program=self.token_program.to_account_info();
        let cpi_accounts=TransferChecked{
            from:self.vault_b.to_account_info(),
            to:self.user_token_account_b.to_account_info(),
            mint:self.mint_b.to_account_info(),
            authority:self.config.to_account_info()
        };
        let decimals=self.mint_b.decimals;
        let mint_a_key=self.mint_a.key();
        let mint_b_key=self.mint_b.key();
        let signer_seeds:&[&[&[u8]]]=&[&[
            b"config",
            mint_a_key.as_ref(),
            mint_b_key.as_ref(),
            &[self.config.config_bump],
        ]];
        let cpi_ctx=CpiContext::new_with_signer(cpi_program,cpi_accounts,signer_seeds);
        transfer_checked(cpi_ctx,amount,decimals);
        Ok(())
    }
}