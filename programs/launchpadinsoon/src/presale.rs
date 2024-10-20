use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer as TokenTransfer};
use crate::{GlobalStats, PresaleError, PresaleInfo};
use crate::vesting::VestingInfo;

#[derive(Accounts)]
pub struct InitializePresale<'info> {
    #[account(init, payer = creator, space = Presale::MIN_LEN)]
    pub presale: Account<'info, Presale>,
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub creator_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub commission_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub global_stats: Account<'info, GlobalStats>,
}

#[account]
pub struct Presale {
    pub id: u64,
    pub creator: Pubkey,
    pub raise_token: Pubkey,
    pub sale_token: Pubkey,
    pub total_tokens: u64,
    pub tokens_sold: u64,
    pub price: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub vesting_end_time: i64,
    pub raise_goal: u64,
    pub total_raised: u64,
    pub total_investors: u32,
    pub paused: bool,
    pub whitelist: Vec<Pubkey>,
    pub allocations: Vec<(Pubkey, u64)>,
    pub buyer_purchases: Vec<(Pubkey, u64)>,
    pub vestings: Vec<(Pubkey, VestingInfo)>,
    pub commission_paid: u64,
    pub bump: u8,
    pub token_vault: Pubkey,
    pub claims: Vec<(Pubkey, bool)>,
    pub max_entries: u64,
}

impl Presale {
    pub const MIN_LEN: usize = 8 + // discriminator
        8 + // id
        32 + // creator
        32 + // raise_token
        32 + // sale_token
        8 + // total_tokens
        8 + // tokens_sold
        8 + // price
        8 + // start_time
        8 + // end_time
        8 + // vesting_end_time
        8 + // raise_goal
        8 + // total_raised
        4 + // total_investors
        1 + // paused
        (4 + 32 * 1000) + // whitelist (assume max 1000 entries)
        (4 + (32 + 8) * 1000) + // allocations (max 1000)
        (4 + (32 + 8) * 1000) + // buyer_purchases (max 1000)
        (4 + (32 + VestingInfo::LEN) * 1000) + // vestings (max 1000)
        8 + // commission_paid
        1 + // bump
        32 + // token_vault
        (4 + (32 + 1) * 1000) + // claims (max 1000)
        8; // max_entries
}

pub fn initialize_presale(
    ctx: Context<InitializePresale>,
    id: u64,
    total_tokens: u64,
    price: u64,
    start_time: i64,
    end_time: i64,
    vesting_end_time: i64,
    raise_goal: u64,
    bump: u8,
    max_entries: u64,
) -> Result<()> {
    let presale = &mut ctx.accounts.presale;
    let creator = ctx.accounts.creator.key();
    let global_stats = &mut ctx.accounts.global_stats;

    // Verificar si el ID ya existe
    require!(
        !global_stats.presales.iter().any(|p| p.id == id),
        PresaleError::DuplicatePresaleId
    );

    // Verificar que la comisión vault es una cuenta de token
    require!(
        ctx.accounts.commission_vault.mint == ctx.accounts.token_program.key(),
        PresaleError::InvalidCommissionVault
    );

    presale.id = id;
    presale.creator = creator;
    presale.total_tokens = total_tokens;
    presale.tokens_sold = 0;
    presale.price = price;
    presale.start_time = start_time;
    presale.end_time = end_time;
    presale.vesting_end_time = vesting_end_time;
    presale.raise_goal = raise_goal;
    presale.total_raised = 0;
    presale.total_investors = 0;
    presale.paused = false;
    presale.whitelist = Vec::new();
    presale.allocations = Vec::new();
    presale.buyer_purchases = Vec::new();
    presale.vestings = Vec::new();
    presale.commission_paid = 0;
    presale.bump = bump;
    presale.token_vault = ctx.accounts.token_vault.key();
    presale.claims = Vec::new();
    presale.max_entries = max_entries;
    presale.raise_token = ctx.accounts.token_program.key();
    presale.sale_token = ctx.accounts.token_vault.mint;

    // Calcular la comisión
    let duration_weeks = (end_time - start_time) / (7 * 24 * 60 * 60) + 1;
    let commission = duration_weeks as u64 * 100_000_000u64; // 0.1 SOL por semana en lamports

    // Transferir la comisión
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        anchor_lang::system_program::Transfer {
            from: ctx.accounts.creator.to_account_info(),
            to: ctx.accounts.commission_vault.to_account_info(),
        },
    );
    anchor_lang::system_program::transfer(cpi_context, commission.try_into().unwrap())?;

    // Transferir los tokens del creador al token_vault
    let cpi_accounts = TokenTransfer {
        from: ctx.accounts.creator_token_account.to_account_info(),
        to: ctx.accounts.token_vault.to_account_info(),
        authority: ctx.accounts.creator.to_account_info(),
    };
    token::transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
        total_tokens,
    )?;

    // Actualizar estadísticas globales
    require!(
        global_stats.presales.len() < GlobalStats::MAX_PRESALES,
        PresaleError::TooManyPresales
    );
    global_stats.presales.push(PresaleInfo {
        id: presale.id,
        sale_token: presale.sale_token,
        total_raised: 0,
        total_investors: 0,
    });
    global_stats.total_presales += 1;
    global_stats.total_investors = global_stats.total_investors.saturating_add(1);

    Ok(())
}

