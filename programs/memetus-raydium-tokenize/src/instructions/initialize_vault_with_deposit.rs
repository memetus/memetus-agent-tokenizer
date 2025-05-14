use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::events::InitializeVaultWithDepositEvent;
use crate::states::vault_state::*;
use crate::states::program_state::*;

#[derive(Accounts)]

pub struct InitializeVaultWithDeposit<'info> {
  #[account(mut)]
  pub owner: Signer<'info>,

  #[account(
    init,
    payer = owner,
    space = VaultState::INIT_SPACE,
    seeds = [
      b"vault_manager", 
      (program_state.total + 1).to_le_bytes().as_ref(), 
      owner.key().as_ref()
    ],
    bump
  )]
  pub vault_manager: Account<'info, VaultState>,

  #[account(
    seeds = [
      b"vault_treasury", 
      (program_state.total + 1).to_le_bytes().as_ref(), 
      owner.key().as_ref()
    ],
    bump
  )]
  pub vault_treasury: SystemAccount<'info>,

  pub program_state: Account<'info, ProgramState>,

  pub system_program: Program<'info, System>,
}

impl<'info> InitializeVaultWithDeposit<'info> {
  pub fn initialize_vault_with_deposit(
    &mut self,
    bumps: &InitializeVaultWithDepositBumps,
    amount: u64,
  ) -> Result<()> {
    let vault_manager = &mut self.vault_manager;
    let vault_treasury = &mut self.vault_treasury;
    let program_state = &mut self.program_state;

    vault_manager.set_inner(VaultState {
      id: program_state.total + 1,
      owner: self.owner.key(),
      vault_treasury: vault_treasury.key(),
      timestamp: Clock::get()?.unix_timestamp,
      amount,
      status: VaultStatus::Active,
      bump: bumps.vault_manager,
    });


    let cpi_accounts = system_program::Transfer {
      from: self.owner.to_account_info(),
      to: vault_treasury.to_account_info(),
    };

    let cpi_program = self.system_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    system_program::transfer(cpi_ctx, amount)?;

    emit!(InitializeVaultWithDepositEvent {
      id: vault_manager.id,
      owner: self.owner.key(),
      vault_manager: vault_manager.key(),
      vault_treasury: vault_treasury.key(),
      timestamp: Clock::get()?.unix_timestamp,
      status: VaultStatus::Active,
      amount,
    });

    msg!("InitializeVaultWithDepositEvent emitted: id={}, owner={} vault_manager={}, vault_treasury={}, amount={}, status={:?}", 
      vault_manager.id,
      self.owner.key(),
      vault_manager.key(),
      vault_treasury.key(),
      amount,
      VaultStatus::Active);

    program_state.total += 1;

    Ok(())
  }
}