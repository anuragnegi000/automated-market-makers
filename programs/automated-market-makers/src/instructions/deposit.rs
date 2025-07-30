use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        mint_to,transfer_checked,Mint,TokenAccount,MintTo,Token,TransferChecked
    }
};

use constant_product_curve::ConstantProduct;

use crate::{AmmError, Config};



#[derive(Accounts)]
pub struct Deposit<'info>{
    #[account(mut)]
    pub user:Signer<'info>,

    pub mint_a:Account<'info,Mint>,
    pub mint_b:Account<'info,Mint>,

    #[account(  
        has_one=mint_a,
        has_one=mint_b,
        seeds=[b"config",mint_a.key().as_ref(),mint_b.key().as_ref()],
        bump=config.config_bump
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
        associated_token::authority=user
    )]
    pub user_token_account_a:Account<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint=mint_b,
        associated_token::authority=user
    )]
    pub user_token_account_b:Account<'info,TokenAccount>,

    #[account(
        init_if_needed,
        payer=user,
        associated_token::mint=mint_lp,
        associated_token::authority=user
    )]
    pub user_token_account_lp:Account<'info,TokenAccount>,

    pub associated_token_program:Program<'info,AssociatedToken>,
    pub token_program:Program<'info,Token>,
    pub system_program:Program<'info,System>
}

impl <'info>Deposit<'info>{
    pub fn deposit(&mut self,amount:u64,max_a:u64,max_b:u64)->Result<()>{
        let (a,b)=if self.mint_lp.supply==0{
            (max_a,max_b)
        }else{
            let amounts=ConstantProduct::xy_deposit_amounts_from_l(
                self.vault_a.amount, //1st vault
                self.vault_b.amount, //2nd vault
                self.mint_lp.supply, // current LP supply
                amount, //user demand of the LP tokens
                6
            ).unwrap();
            (amounts.x,amounts.y)
        };

        require!(a<=max_a && b<=max_b,AmmError::SlippageExceeded);

        self.deposit_token_a(a)?;
        self.deposit_token_b(b)?;
        self.mint_lp_tokens(amount)?;
        Ok(())
    }

    pub fn deposit_token_a(&mut self,amount:u64)->Result<()>{
        let cpi_program=self.token_program.to_account_info();
        let cpi_account=TransferChecked{
            from:self.user_token_account_a.to_account_info(),
            to:self.vault_a.to_account_info(),
            mint:self.mint_a.to_account_info(),
            authority:self.user.to_account_info()
        };
        let cpi_ctx=CpiContext::new(cpi_program,cpi_account);
        transfer_checked(cpi_ctx,amount,self.mint_a.decimals);
        Ok(())
    }
    pub fn deposit_token_b(&mut self, amount:u64)->Result<()>{
        let cpi_program=self.token_program.to_account_info();
        let cpi_accounts=TransferChecked{
            from:self.user_token_account_b.to_account_info(),
            to:self.vault_b.to_account_info(),
            mint:self.mint_b.to_account_info(),
            authority:self.user.to_account_info(),
        };
        let cpi_ctx=CpiContext::new(cpi_program,cpi_accounts);
        transfer_checked(cpi_ctx,amount,self.mint_b.decimals);
        Ok(())
    }

    pub fn mint_lp_tokens(&mut self,amount:u64)->Result<()>{
        let cpi_program=self.token_program.to_account_info();
        let cpi_accounts=MintTo{
            mint:self.mint_lp.to_account_info(),
            to:self.user_token_account_lp.to_account_info(),
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
        let cpi_ctx=CpiContext::new_with_signer(cpi_program,cpi_accounts,signer_seeds);
        mint_to(cpi_ctx, amount);
        Ok(())
    }
}