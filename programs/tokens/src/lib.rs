use anchor_lang::prelude::*;

pub mod instructions;
use instructions::initialize::*;

declare_id!("6s2aRCb8CdM793yTt8Xd8yfZ7b8xjd7tDwPWUYmiKkgp");

#[program]
pub mod tokens {
    use super::*;

    pub fn initialize(ctx: Context<InitializeContext>) -> Result<()> {
        _initialize(ctx)
    }
}
