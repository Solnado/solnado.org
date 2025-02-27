use anchor_lang::prelude::*;
use solana_program::program::invoke;
use solana_program::system_instruction;
use rand::seq::SliceRandom;

declare_id!("SolnadoMixer11111111111111111111111111111111");

#[program]
pub mod sheriff_mixer {
    use super::*;

    pub fn deposit_and_mix(ctx: Context<DepositAndMix>, amount: u64) -> Result<()> {
        let user = &ctx.accounts.user;
        let mix_account = &ctx.accounts.mix_account;
        let system_program = &ctx.accounts.system_program;

        require!(amount > 0, SheriffError::InvalidAmount);

        invoke(
            &system_instruction::transfer(user.to_account_info().key, mix_account.to_account_info().key, amount),
            &[user.to_account_info(), mix_account.to_account_info(), system_program.to_account_info()],
        )?;

        Ok(())
    }

    pub fn process_mixing(ctx: Context<ProcessMixing>, final_wallets: Vec<Pubkey>) -> Result<()> {
        let mix_account = &ctx.accounts.mix_account;
        let destination_wallet = final_wallets.choose(&mut rand::thread_rng()).unwrap();

        let amount = mix_account.to_account_info().lamports();
        let fee = amount * 7 / 1000; // %0.7 Mixing Fee
        let mix_amount = amount - fee;

        **mix_account.to_account_info().try_borrow_mut_lamports()? -= mix_amount;
        **ctx.accounts.clean_wallet.to_account_info().try_borrow_mut_lamports()? += mix_amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct DepositAndMix<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub mix_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessMixing<'info> {
    #[account(mut)]
    pub mix_account: AccountInfo<'info>,
    #[account(mut)]
    pub clean_wallet: AccountInfo<'info>,
}

#[error_code]
pub enum SheriffError {
    #[msg("Invalid amount")]
    InvalidAmount,
}
