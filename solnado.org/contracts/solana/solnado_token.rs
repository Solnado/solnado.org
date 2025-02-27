use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, InitializeMint, Transfer};

declare_id!("solnado11111111111111111111111");

#[program]
pub mod sheriff_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, decimals: u8) -> Result<()> {
        let mint = &ctx.accounts.mint;
        let mint_authority = &ctx.accounts.mint_authority;

        anchor_spl::token::initialize_mint(
            &ctx.accounts.token_program,
            mint,
            mint_authority,
            None,
            decimals,
        )?;

        Ok(())
    }

    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        let mint = &ctx.accounts.mint;
        let mint_authority = &ctx.accounts.mint_authority;
        let recipient = &ctx.accounts.recipient;

        anchor_spl::token::mint_to(
            &ctx.accounts.token_program,
            mint,
            recipient,
            mint_authority,
            &[],
            amount,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, mint::decimals = decimals, mint::authority = mint_authority)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub mint_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,
    pub mint_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
