use anchor_lang::prelude::*;

declare_id!("71CGpNHh9383Un2LSUdDhHHDuZZhiKWvGHcXUUukVhWW");

#[program]
pub mod anchor_tutorial {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
