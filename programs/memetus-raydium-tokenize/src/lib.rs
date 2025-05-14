use anchor_lang::prelude::*;

mod states;
mod events;
mod errors;

use instructions::*;

pub mod instructions;


declare_id!("C1Knh3YFfMwr2LLaoT9JXcVZHHH52YSm8gBNyCinGABg");

#[program]
pub mod memetus_raydium {
    use super::*;

    pub fn initialize_factory(
        ctx: Context<InitializeFactory>
    ) -> Result<()> {
        let _ = ctx.accounts.initialize_factory(&ctx.bumps);
        Ok(())
    }

    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
    ) -> Result<()> {
        let _ = ctx.accounts.initialize_vault(&ctx.bumps)?;
        Ok(())
    }

    pub fn initialize_vault_with_deposit(
        ctx: Context<InitializeVaultWithDeposit>,
        amount: u64,
    ) -> Result<()> {
        let _ = ctx.accounts.initialize_vault_with_deposit(&ctx.bumps, amount)?;
        Ok(())
    }

    pub fn update_owner(
        ctx: Context<UpdateOwner>,
        new_owner: Pubkey,
    ) -> Result<()> {
        let _ = ctx.accounts.update_owner(new_owner)?;
        Ok(())
    }

    pub fn withdraw_sol(
        ctx: Context<WithdrawSol>,
        id: u64,
    ) -> Result<()> {
        let _ = ctx.accounts.withdraw_sol(id)?;
        Ok(())
    }

    pub fn deposit_sol(ctx: Context<DepositSol>, id: u64, amount: u64) -> Result<()> {
        let _ = ctx.accounts.deposit_sol(id, amount)?;
        Ok(())
    }

    pub fn buy_token(ctx: Context<BuyToken>, amount_in: u64, minimum_amount_out: u64) -> Result<()> {
        let _ = instructions::buy_token(ctx, amount_in, minimum_amount_out);
        Ok(())
    }

    pub fn sell_token(ctx: Context<SellToken>, amount_out: u64, maximum_amount_oin: u64) -> Result<()> {
        let _ = instructions::sell_token(ctx, amount_out, maximum_amount_oin);
        Ok(())
    }
}