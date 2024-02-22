// use this import to gain access to common anchor features
use anchor_lang::prelude::*;

use anchor_spl::token::TokenAccount;

declare_id!("71CGpNHh9383Un2LSUdDhHHDuZZhiKWvGHcXUUukVhWW");

// write your business logic here
#[program]
pub mod anchor_tutorial {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }


    pub fn set_data(ctx: Context<SetData>, data: MyAccount) -> Result<()> {
        if data.data >= 100 {
            return err!(MyError::DataTooLarge);
        }

        require!(data.data < 100, MyError::DataTooLarge);


        if ctx.accounts.token_account.amount > 0 {
            
            ctx.accounts.my_account.set_inner(data);
            // ctx.accounts.my_account.data = data;
            // ctx.accounts.my_account.age = data.age;
        }
        Ok(())
    }

}




// validate incoming accounts here
#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
    #[account(
        constraint = my_account.mint == token_account.mint,
        has_one = owner
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub owner: Signer<'info>
}


#[account]
#[derive(Default)]
pub struct MyAccount {
    data: u64,
    pub age: u8,
    mint: Pubkey

}


#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub potentially_dangerous: UncheckedAccount<'info>
}




#[error_code]
pub enum MyError {
    #[msg("MyAccount may only hold data below 100")]
    DataTooLarge,

    #[msg("MyAccount may only hold data above 1")]
    DataTooSmall



}


