use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer as TokenTransfer};
use crate::{Presale, PresaleError};

#[derive(Accounts)]
pub struct BuyTokens<'info> {
    #[account(mut)]
    pub presale: Account<'info, Presale>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_vault: Account<'info, TokenAccount>,
    /// CHECK: Este es la cuenta que recibe el pago
    #[account(mut)]
    pub treasury: AccountInfo<'info>,
    /// CHECK: Esta es la autoridad de la venta
    pub sale_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub global_stats: Account<'info, GlobalStats>,
}

pub fn buy_tokens(ctx: Context<BuyTokens>, presale_id: u64, amount: u64) -> Result<()> {
    let presale = &mut ctx.accounts.presale;
    let global_stats = &mut ctx.accounts.global_stats;

    // Verificar que el ID de preventa coincide
    require!(presale.id == presale_id, PresaleError::InvalidPresaleId);

    // Verificar que la preventa no esté pausada
    require!(!presale.paused, PresaleError::PresalePaused);

    // Verificar que el comprador está en la lista blanca
    require!(presale.whitelist.contains(&ctx.accounts.buyer.key()), PresaleError::NotWhitelisted);

    // Obtener la asignación para el comprador
    let allocation = presale.allocations.iter()
        .find(|(pubkey, _)| pubkey == &ctx.accounts.buyer.key())
        .map(|(_, allocation)| *allocation)
        .unwrap_or(0);

    // Obtener la compra actual del comprador
    let buyer_purchase = presale.buyer_purchases.iter()
        .find(|(pubkey, _)| pubkey == &ctx.accounts.buyer.key())
        .map(|(_, purchase)| *purchase)
        .unwrap_or(0);

    // Verificar que la cantidad no exceda la asignación disponible
    require!(amount <= allocation.saturating_sub(buyer_purchase), PresaleError::AllocationExceeded);

    // Calcular el costo total
    let cost = presale.price.checked_mul(amount).ok_or(PresaleError::CalculationError)?;

    // Transferir SOL del comprador al treasury
    {
        let buyer_info = ctx.accounts.buyer.to_account_info();
        let buyer_lamports = &mut **buyer_info.lamports.borrow_mut();
        *buyer_lamports = buyer_lamports.checked_sub(cost).ok_or(PresaleError::CalculationError)?;
    }
    {
        let treasury_info = ctx.accounts.treasury.to_account_info();
        let treasury_lamports = &mut **treasury_info.lamports.borrow_mut();
        *treasury_lamports = treasury_lamports.checked_add(cost).ok_or(PresaleError::CalculationError)?;
    }

    // Transferir tokens del token_vault al comprador
    let cpi_accounts = TokenTransfer {
        from: ctx.accounts.token_vault.to_account_info(),
        to: ctx.accounts.buyer_token_account.to_account_info(),
        authority: ctx.accounts.sale_authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    // Antes de añadir una nueva entrada, verificar si hay espacio suficiente
    require!(
        {
            let current_len = presale.buyer_purchases.len() as u64;
            current_len < presale.max_entries
        },
        PresaleError::InsufficientSpace
    );

    // Actualizar el estado de la preventa
    presale.tokens_sold = presale.tokens_sold.checked_add(amount).ok_or(PresaleError::CalculationError)?;

    if let Some((_, purchase)) = presale.buyer_purchases.iter_mut().find(|(pubkey, _)| pubkey == &ctx.accounts.buyer.key()) {
        *purchase = purchase.checked_add(amount).ok_or(PresaleError::CalculationError)?;
    } else {
        presale.buyer_purchases.push((ctx.accounts.buyer.key(), amount));
    }

    presale.total_raised = presale.total_raised.checked_add(cost).ok_or(PresaleError::CalculationError)?;
    presale.total_investors = presale.total_investors.saturating_add(1);

    // Actualizar estadísticas globales
    if let Some(presale_info) = global_stats.presales.iter_mut().find(|p| p.id == presale.id) {
        presale_info.total_raised = presale.total_raised;
        presale_info.total_investors = presale.total_investors as u64;
    }
    global_stats.total_raised = global_stats.total_raised.checked_add(cost).ok_or(PresaleError::CalculationError)?;
    global_stats.total_investors = global_stats.total_investors.saturating_add(1);

    // Verificar que la preventa está activa
    let current_time = Clock::get()?.unix_timestamp;
    require!(
        current_time >= presale.start_time && current_time <= presale.end_time,
        PresaleError::PresaleNotActive
    );

    Ok(())
}

#[account]
#[derive(Default)]
pub struct GlobalStats {
    pub total_raised: u64,
    pub total_investors: u64,
    pub total_presales: u32,
    pub presales: Vec<PresaleInfo>,
}

impl GlobalStats {
    pub const MAX_PRESALES: usize = 100;
    pub const LEN: usize = 8 + 8 + 4 + (4 + (8 + 32 + 8 + 8) * Self::MAX_PRESALES);
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct PresaleInfo {
    pub id: u64,
    pub sale_token: Pubkey,
    pub total_raised: u64,
    pub total_investors: u64,
}
