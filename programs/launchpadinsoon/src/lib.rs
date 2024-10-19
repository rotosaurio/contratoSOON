use anchor_lang::prelude::*;

declare_id!("74VT9QxrMB8gWYynpS7m9bxAygWy6tTAqHELeDsVmRNV");

pub mod whitelist;
pub mod allocation;
pub mod pricing;
pub mod security;
pub mod vesting;
pub mod governance;
pub mod sale;

use crate::whitelist::*;
use crate::allocation::*;
use crate::pricing::*;
use crate::security::*;
use crate::vesting::*;
use crate::governance::*;
use crate::sale::*;

#[program]
pub mod launchpadinsoon {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>, total_tokens: u64, price: u64) -> Result<()> {
        sale::initialize(ctx, total_tokens, price)
    }

    pub fn add_to_whitelist(ctx: Context<AddToWhitelist>, user: Pubkey) -> Result<()> {
        whitelist::add_to_whitelist(ctx, user)
    }

    pub fn set_allocation(ctx: Context<SetAllocation>, user: Pubkey, allocation: u64) -> Result<()> {
        allocation::set_allocation(ctx, user, allocation)
    }

    pub fn buy_tokens(ctx: Context<BuyTokens>, amount: u64) -> Result<()> {
        pricing::buy_tokens(ctx, amount)
    }

    pub fn pause_sale(ctx: Context<PauseSale>) -> Result<()> {
        security::pause_sale(ctx)
    }

    pub fn unpause_sale(ctx: Context<UnpauseSale>) -> Result<()> {
        security::unpause_sale(ctx)
    }

    pub fn create_vesting(ctx: Context<CreateVesting>, amount: u64, release_time: i64) -> Result<()> {
        vesting::create_vesting(ctx, amount, release_time)
    }

    pub fn update_parameters(ctx: Context<UpdateParameters>, new_price: Option<u64>, new_total_tokens: Option<u64>) -> Result<()> {
        governance::update_parameters(ctx, new_price, new_total_tokens)
    }

    pub fn pause_contract(ctx: Context<PauseContract>) -> Result<()> {
        governance::pause_contract(ctx)
    }

    pub fn unpause_contract(ctx: Context<UnpauseContract>) -> Result<()> {
        governance::unpause_contract(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = admin, space = 8 + Sale::LEN)]
    pub sale: Account<'info, Sale>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct Sale {
    pub admin: Pubkey,
    pub total_tokens: u64,
    pub tokens_sold: u64,
    pub price: u64,
    pub paused: bool,
    pub whitelist: Vec<Pubkey>,
    pub allocations: Vec<(Pubkey, u64)>,
    pub buyer_purchases: Vec<(Pubkey, u64)>,
    pub vestings: Vec<(Pubkey, VestingInfo)>,
}

impl Sale {
    const LEN: usize = 32 + 8 + 8 + 8 + 1 + (32 * 100) + (40 * 100) + (40 * 100) + (72 * 100);
}

#[error_code]
pub enum SaleError {
    #[msg("La venta está pausada.")]
    SalePaused,
    #[msg("El usuario no está en la lista blanca.")]
    NotWhitelisted,
    #[msg("La cantidad solicitada excede la asignación.")]
    AllocationExceeded,
    #[msg("Error en los cálculos.")]
    CalculationError,
}