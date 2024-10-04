use anchor_lang::prelude::*;

declare_id!("7wWfAXexLPgeUs9F543Ag92qyxxdU4NacDkDqQaY2BWa");

#[program]
pub mod learnv1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
