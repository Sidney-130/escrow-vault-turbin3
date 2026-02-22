use anchor_lang::prelude::*;

pub mod constants;
pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("AtgfV9JbP4BWByEiwbYNbZKu8ztq7M7SshBPnCcY8Rt5");

#[program]
pub mod escrow_v_tubin3 {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.deposit(deposit)?;
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close_vault()
    }
}