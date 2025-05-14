use anchor_lang::prelude::*;

use crate::events::initialize_vault_event::InitializeVaultEvent;
use crate::states::vault_state::*;
use crate::states::program_state::*;

#[derive(Accounts)]

pub struct InitializeVault<'info> {
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

impl<'info> InitializeVault<'info> {
  pub fn initialize_vault(
    &mut self,
    bumps: &InitializeVaultBumps,
  ) -> Result<()> {
    let vault_manager = &mut self.vault_manager;
    let vault_treasury = &mut self.vault_treasury;
    let program_state = &mut self.program_state;

    vault_manager.set_inner(VaultState {
      id: program_state.total + 1,
      owner: self.owner.key(),
      vault_treasury: vault_treasury.key(),
      timestamp: Clock::get()?.unix_timestamp,
      amount: 0,
      status: VaultStatus::Uninitialized,
      bump: bumps.vault_manager,
    });

    emit!(InitializeVaultEvent {
      id: vault_manager.id,
      owner: self.owner.key(),
      vault_manager: vault_manager.key(),
      vault_treasury: vault_treasury.key(),
      timestamp: Clock::get()?.unix_timestamp,
      status: VaultStatus::Active,
      amount: 0,
    });

    msg!("InitializeVault emitted: id={}, owner={} vault_manager={}, vault_treasury={}, amount={}, status={:?}", 
      vault_manager.id,
      self.owner.key(),
      vault_manager.key(),
      vault_treasury.key(),
      0,
      VaultStatus::Uninitialized);

    program_state.total += 1;

    Ok(())
  }
}