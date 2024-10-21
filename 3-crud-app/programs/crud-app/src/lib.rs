use anchor_lang::prelude::*;

declare_id!("4f1n3riAidKogQF5REcLXSN3Bc9veGRBJ6d5wZhevpbz");

#[program]
pub mod crud_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
