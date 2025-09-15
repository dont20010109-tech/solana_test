use anchor_lang::prelude::*;

declare_id!("GEBib1Xh3fiXTUBiZjmkNXtfH5vLfA5eoHy36tw78tc4");

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
