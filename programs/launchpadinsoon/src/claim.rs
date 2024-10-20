use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer as TokenTransfer};
use crate::{Presale, PresaleError, GlobalStats};

#[derive(Accounts)]
pub struct ClaimTokens<'info> {
    #[account(mut)]
    pub presale: Account<'info, Presale>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub global_stats: Account<'info, GlobalStats>,
}

pub fn claim_tokens(ctx: Context<ClaimTokens>) -> Result<()> {
    let presale = &mut ctx.accounts.presale;
    let user_key = ctx.accounts.user.key();
    let global_stats = &mut ctx.accounts.global_stats;

    // Obtener la cantidad a reclamar
    let amount = presale.vestings.iter()
        .find(|(pubkey, _)| pubkey == &user_key)
        .map(|(_, vesting)| vesting.amount)
        .unwrap_or(0);

    require!(amount > 0, PresaleError::NoTokensToClaim);

    // Transferir tokens del token_vault a la cuenta del usuario
    let presale_key = presale.key();
    let authority_seeds = &[presale_key.as_ref(), &[presale.bump]];
    let signer = &[&authority_seeds[..]];
    
    let cpi_accounts = TokenTransfer {
        from: ctx.accounts.token_vault.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: presale.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    token::transfer(cpi_ctx, amount)?;

    // Actualizar el estado de reclamación del usuario
    if let Some((_pubkey, ref mut claimed)) = presale.claims.iter_mut().find(|(pubkey, _)| pubkey == &user_key) {
        *claimed = true;
    } else {
        require!(
            presale.claims.len() < presale.max_entries as usize,
            PresaleError::InsufficientSpace
        );
        presale.claims.push((user_key, true));
    }

    // Actualizar estadísticas globales
    if let Some(presale_info) = global_stats.presales.iter_mut().find(|p| p.id == presale.id) {
        presale_info.total_raised = presale.total_raised;
    }

    // Verificar si el tiempo de vesting ha pasado
    let current_time = Clock::get()?.unix_timestamp;
    let vesting_info = presale.vestings.iter()
        .find(|(pubkey, _)| pubkey == &user_key)
        .map(|(_, info)| info)
        .ok_or(PresaleError::NoVestingFound)?;
    require!(
        current_time >= vesting_info.release_time,
        PresaleError::VestingPeriodNotEnded
    );

    Ok(())
}
