use anchor_lang::prelude::*;
pub mod states;
pub mod instructions;
declare_id!("BfvAstRx4vfgffP7PmoFKZYdRZ66MeexN6sfAtGrBs2w");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
