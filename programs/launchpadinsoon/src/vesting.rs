use anchor_lang::prelude::*;
use crate::{Presale, PresaleError, GlobalStats};

#[derive(Accounts)]
pub struct CreateVesting<'info> {
    #[account(mut)]
    pub sale: Account<'info, Presale>,
    pub admin: Signer<'info>,
    pub user: Signer<'info>,
    #[account(mut)]
    pub global_stats: Account<'info, GlobalStats>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct VestingInfo {
    pub amount: u64,
    pub release_time: i64,
    pub claimed: bool,
}

impl VestingInfo {
    pub const LEN: usize = 8 + 8 + 1; // u64 + i64 + bool
}

pub fn create_vesting(ctx: Context<CreateVesting>, amount: u64, release_time: i64) -> Result<()> {
    let sale = &mut ctx.accounts.sale;
    let user_key = ctx.accounts.user.key();
    let global_stats = &mut ctx.accounts.global_stats;

    require!(
        !sale.vestings.iter().any(|(pubkey, _)| pubkey == &user_key),
        PresaleError::VestingAlreadyExists
    );

    require!(
        sale.vestings.len() < sale.max_entries as usize,
        PresaleError::InsufficientSpace
    );

    // Verificar si el usuario ha comprado tokens
    require!(
        sale.buyer_purchases.iter().any(|(pubkey, _)| pubkey == &user_key),
        PresaleError::NoPurchaseFound
    );

    sale.vestings.push((
        user_key,
        VestingInfo {
            amount,
            release_time,
            claimed: false,
        },
    ));

    // Actualizar estadísticas globales
    if let Some(presale_info) = global_stats.presales.iter_mut().find(|p| p.id == sale.id) {
        presale_info.total_investors = sale.total_investors as u64;
    }
    global_stats.total_investors = global_stats.total_investors.saturating_add(1);

    Ok(())
}
