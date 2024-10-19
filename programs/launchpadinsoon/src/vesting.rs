use anchor_lang::prelude::*;
use crate::{Presale, PresaleError};

#[derive(Accounts)]
pub struct CreateVesting<'info> {
    #[account(mut)]
    pub sale: Account<'info, Presale>,
    pub admin: Signer<'info>,
    pub user: Signer<'info>,
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

    require!(
        !sale.vestings.iter().any(|(pubkey, _)| pubkey == &user_key),
        PresaleError::VestingAlreadyExists
    );

    // Verificar si hay espacio suficiente antes de añadir una nueva entrada
    require!(
        {
            let current_len = sale.vestings.len() as u64;
            current_len < sale.max_entries
        },
        PresaleError::InsufficientSpace
    );
    
    
    

    sale.vestings.push((
        user_key,
        VestingInfo {
            amount,
            release_time,
            claimed: false,
        },
    ));

    Ok(())
}