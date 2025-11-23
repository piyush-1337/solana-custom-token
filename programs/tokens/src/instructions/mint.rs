use anchor_lang::{
    prelude::*,
    system_program::{create_account, CreateAccount},
};
use anchor_spl::{
    token_2022::{
        initialize_mint2,
        spl_token_2022::{extension::ExtensionType, pod::PodMint},
        InitializeMint2, Token2022,
    },
    token_interface::{transfer_fee_initialize, TransferFeeInitialize},
};

pub fn _mint(ctx: Context<MintContext>, amount: u64) -> Result<()> {
    let system_program = &ctx.accounts.system_program;
    let token_program = &ctx.accounts.token_program;
    let creator = &ctx.accounts.creator;
    let to = &ctx.accounts.mint;

    let space =
        ExtensionType::try_calculate_account_len::<PodMint>(&[ExtensionType::TransferFeeConfig])?;
    let lamports = Rent::get()?.minimum_balance(space);

    let create_account_ctx = CpiContext::new(
        system_program.to_account_info(),
        CreateAccount {
            from: creator.to_account_info(),
            to: to.to_account_info(),
        },
    );

    create_account(
        create_account_ctx,
        lamports,
        space as u64,
        &token_program.key(),
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct MintContext<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub mint: Signer<'info>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}
