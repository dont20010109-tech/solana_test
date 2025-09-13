// use anchor_lang::prelude::*;

// declare_id!("9ruHXnS2UHdgr15s15a3ub4h59XfDNrDxs9EK6GHetMW");

// #[program]
// pub mod demo {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         msg!("Greetings from: {:?}", ctx.program_id);
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize {}

use anchor_lang::prelude::*;
 
declare_id!("BYFW1vhC1ohxwRbYoLbAWs86STa25i9sD5uEusVjTYNd");
 
#[program]
mod hello_anchor {
    use super::*;



    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        ctx.accounts.new_account.data = data;
        msg!("Changed data to: {}!", data);
        Ok(())
    }
}
 

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
 
#[account]
pub struct NewAccount {
    data: u64,
}