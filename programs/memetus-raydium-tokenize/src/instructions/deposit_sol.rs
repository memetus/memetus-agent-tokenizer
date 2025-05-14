use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::errors::deposit_sol_error::DepositSolError;
use crate::states::vault_state::*;
use crate::events::deposit_sol_event::DepositSolEvent;

#[derive(Accounts)]
#[instruction(id: u64)]

pub struct DepositSol<'info> {
  #[account(mut)]
  pub owner: Signer<'info>,

  #[account(
    init,
    payer = owner,
    space = VaultState::INIT_SPACE,
    seeds = [
      b"vault_manager", 
      id.to_le_bytes().as_ref(),
      owner.key().as_ref()
    ],
    bump
  )]
  pub vault_manager: Account<'info, VaultState>,

  #[account(
    seeds = [
      b"vault_treasury", 
      id.to_le_bytes().as_ref(),
      owner.key().as_ref()
    ],
    bump
  )]
  pub vault_treasury: SystemAccount<'info>,

  pub system_program: Program<'info, System>,
}

impl<'info> DepositSol<'info> {
  pub fn deposit_sol(
    &mut self,
    id: u64,
    amount: u64,
  ) -> Result <()> {
    require!(
      self.vault_manager.status == VaultStatus::Active,
      DepositSolError::VaultAlreadyInitialized
    );

    // require!(
    //   self.vault_manager.status == VaultStatus::Settled,
    //   DepositSolError::VaultNotActive
    // );

    let cpi_accounts = system_program::Transfer {
      from: self.owner.to_account_info(),
      to: self.vault_treasury.to_account_info(),
    };

    let cpi_program = self.system_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    system_program::transfer(cpi_ctx, amount)?;

    self.vault_manager.amount += amount;
    self.vault_manager.status = VaultStatus::Active;

    emit!(DepositSolEvent {
      id: id,
      owner: self.owner.key(),
      vault_manager: self.vault_manager.key(),  
      vault_treasury: self.vault_treasury.key(),
      timestamp: Clock::get()?.unix_timestamp,
      status: VaultStatus::Active,
      amount: amount,
    });

    msg!(
    "DepositSolEvent emitted: id={}, owner={}, amount={}",
    id,
    self.owner.key(),
    amount
    );

    Ok(())
  }
}