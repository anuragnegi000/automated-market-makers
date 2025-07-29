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
        seeds=[b"config",mint_a.key().as_ref(),mint_b.key().as_ref()],
        bump=config.config_bump,
        has_one=mint_a,
        has_one=mint_b
    )]
    pub config:Account<'info,Config>,

    #[account(
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
        associated_token::authority=user
    )]
    pub user_token_account_a:Account<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint=mint_b,
        associated_token::authority=user
    )]
    pub user_token_account_b:Account<'info,TokenAccount>,

    pub system_program:Program<'info,System>,
    pub associated_token_program:Program<'info,AssociatedToken>,
    pub token_program:Program<'info,Token>
}

impl <'info>Swap<'info>{
    pub fn swap(&mut self,is_x:bool,amount_in:u64,min_amount_out:u64)->Result<()>{
        require!(amount_in>0,AmmError::InvalidAmount);

        let mut curve = ConstantProduct::init(
            self.vault_a.amount,
            self.vault_b.amount,
            self.mint_lp.supply,
            self.config.fee,
            None
        ).map_err(|_| AmmError::InvalidAmount)?;
        let p = if is_x {
            LiquidityPair::X
        } else {
            LiquidityPair::Y
        };
        let swap_result = curve.swap(p, amount_in, min_amount_out).map_err(|_| AmmError::InvalidAmount)?;
        require!(swap_result.deposit!=0,AmmError::InvalidAmount);
        require!(swap_result.withdraw!=0,AmmError::InvalidAmount);
        self.deposit_token(is_x, swap_result.deposit)?;
        self.withdraw_token(!is_x, swap_result.withdraw)?;
        Ok(())
    }

    pub fn deposit_token(&mut self,is_a:bool,amount:u64)->Result<()>{
        let(from,to,mint,decimals)=if is_a{(
            self.user_token_account_a.to_account_info(),
            self.vault_a.to_account_info(),
            self.mint_a.to_account_info(),
            self.mint_a.decimals
        )}else{(
            self.user_token_account_b.to_account_info(),
            self.vault_b.to_account_info(),
            self.mint_b.to_account_info(),
            self.mint_b.decimals
        )};
        let cpi_program=self.token_program.to_account_info();
        let cpi_accounts=TransferChecked{
            from,
            to,
            authority:self.user.to_account_info(),
            mint
        };
        let cpi_context=CpiContext::new(cpi_program,cpi_accounts);
        transfer_checked(cpi_context,amount,decimals);
        Ok(())
    }

    pub fn withdraw_token(&mut self,is_x:bool,amount:u64)->Result<()>{
        let (from,to,mint,decimals)=if is_x{(
            self.vault_a.to_account_info(),
            self.user_token_account_a.to_account_info(),
            self.mint_a.to_account_info(),
            self.mint_a.decimals
        )}else{(
            self.vault_b.to_account_info(),
            self.user_token_account_a.to_account_info(),
            self.mint_b.to_account_info(),
            self.mint_b.decimals
        )};
        let cpi_program=self.token_program.to_account_info();
        let cpi_accounts=TransferChecked{
            from,
            to,
            mint,
            authority:self.config.to_account_info()
        };
        let mint_a_key=self.mint_a.key();
        let mint_b_key=self.mint_b.key();
        let signer_seeds:&[&[&[u8]]]=&[&[
            b"config",
            mint_a_key.as_ref(),
            mint_b_key.as_ref(),
            &[self.config.config_bump],
        ]];
        let cpi_context=CpiContext::new_with_signer(cpi_program,cpi_accounts,signer_seeds);
        transfer_checked(cpi_context,amount,decimals);
        Ok(())
    }

    // pub fn burn_lp(&mut self,amount:u64)->Result<()>{
    //     let cpi_program=self.token_program.to_account_info();
    //     let cpi_accounts=Burn{
    //         mint:self.mint_lp.to_account_info(),
    //         from:self.user_token_account_lp.to_account_info(),
    //         authority:self.user.to_account_info()
    //     };

    //     let cpi_ctx=CpiContext::new(cpi_program,cpi_accounts);
    //     burn(cpi_ctx,amount,decimals);
    // }


}