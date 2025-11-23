use anchor_lang::prelude::*;

mod instructions;
use instructions::*;

declare_id!("6s2aRCb8CdM793yTt8Xd8yfZ7b8xjd7tDwPWUYmiKkgp");

#[program]
pub mod tokens {
    use super::*;

    pub fn initialize(ctx: Context<InitializeContext>, fee_bps: u16, max_fee: u64) -> Result<()> {
        _initialize(ctx, fee_bps, max_fee)
    }

    pub fn mint(ctx: Context<MintContext>, amount: u64) -> Result<()> {
        _mint(ctx, amount)
    }

    pub fn transfer(ctx: Context<TransferContext>, amount: u64) -> Result<()> {
        _transfer(ctx, amount)
    }

    pub fn withdraw(ctx: Context<WithdrawContext>) -> Result<()> {
        _withdraw(ctx)
    }
}
