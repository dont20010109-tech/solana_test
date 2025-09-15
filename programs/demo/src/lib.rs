use anchor_lang::prelude::*;

declare_id!("9ruHXnS2UHdgr15s15a3ub4h59XfDNrDxs9EK6GHetMW");

#[program]
pub mod demo {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
