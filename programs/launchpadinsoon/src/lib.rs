use anchor_lang::prelude::*;

declare_id!("74VT9QxrMB8gWYynpS7m9bxAygWy6tTAqHELeDsVmRNV");

pub mod whitelist;
pub mod allocation;
pub mod pricing;
pub mod security;
pub mod vesting;
pub mod governance;
pub mod presale;
pub mod claim;
pub mod sale;

use crate::whitelist::*;
use crate::allocation::*;
use crate::pricing::*;
use crate::security::*;
use crate::vesting::*;
use crate::governance::*;
use crate::presale::*;
use crate::claim::*;
use crate::sale::*;

#[program]
pub mod launchpadinsoon {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        total_tokens: u64,
        price: u64,
        start_time: i64,
        end_time: i64,
        vesting_end_time: i64,
        raise_goal: u64,
        bump: u8,
    ) -> Result<()> {
        sale::initialize(ctx, total_tokens, price, start_time, end_time, vesting_end_time, raise_goal, bump)
    }

    pub fn add_to_whitelist(ctx: Context<AddToWhitelist>, user: Pubkey) -> Result<()> {
        whitelist::add_to_whitelist(ctx, user)
    }

    pub fn pause_sale(ctx: Context<PauseSale>) -> Result<()> {
        security::pause_sale(ctx)
    }

    pub fn unpause_sale(ctx: Context<UnpauseSale>) -> Result<()> {
        security::unpause_sale(ctx)
    }

    pub fn set_allocation(ctx: Context<SetAllocation>, user: Pubkey, allocation: u64) -> Result<()> {
        allocation::set_allocation(ctx, user, allocation)
    }

    pub fn buy_tokens(ctx: Context<BuyTokens>, presale_id: u64, amount: u64) -> Result<()> {
        pricing::buy_tokens(ctx, presale_id, amount)
    }

    pub fn create_vesting(ctx: Context<CreateVesting>, amount: u64, release_time: i64) -> Result<()> {
        vesting::create_vesting(ctx, amount, release_time)
    }

    pub fn claim_tokens(ctx: Context<ClaimTokens>) -> Result<()> {
        claim::claim_tokens(ctx)
    }

    pub fn pause_contract(ctx: Context<PauseContract>) -> Result<()> {
        governance::pause_contract(ctx)
    }

    pub fn unpause_contract(ctx: Context<UnpauseContract>) -> Result<()> {
        governance::unpause_contract(ctx)
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
        presale::initialize_presale(ctx, id, total_tokens, price, start_time, end_time, vesting_end_time, raise_goal, bump, max_entries)
    }
}

#[error_code]
pub enum PresaleError {
    #[msg("La preventa está pausada.")]
    PresalePaused,
    #[msg("El usuario no está en la lista blanca.")]
    NotWhitelisted,
    #[msg("La cantidad solicitada excede la asignación.")]
    AllocationExceeded,
    #[msg("Error en los cálculos.")]
    CalculationError,
    #[msg("ID de preventa inválido.")]
    InvalidPresaleId,
    #[msg("El período de vesting no ha terminado.")]
    VestingPeriodNotEnded,
    #[msg("Ya has reclamado tus tokens.")]
    AlreadyClaimed,
    #[msg("No tienes tokens para reclamar.")]
    NoTokensToClaim,
    #[msg("No hay suficiente espacio en la cuenta para añadir más entradas.")]
    InsufficientSpace,
    #[msg("Vesting ya existe para este usuario.")]
    VestingAlreadyExists,
}