use super::*;

#[derive(Accounts)]
pub struct CreateVesting<'info> {
    #[account(mut)]
    pub sale: Account<'info, Sale>,
    pub admin: Signer<'info>,
    pub user: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct VestingInfo {
    pub amount: u64,
    pub release_time: i64,
    pub claimed: bool,
}

pub fn create_vesting(ctx: Context<CreateVesting>, amount: u64, release_time: i64) -> Result<()> {
    let sale = &mut ctx.accounts.sale;
    let user_key = ctx.accounts.user.key();

    require!(
        !sale.vestings.iter().any(|(pubkey, _)| pubkey == &user_key),
        VestingError::VestingAlreadyExists
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

#[error_code]
pub enum VestingError {
    #[msg("Vesting already exists for this user")]
    VestingAlreadyExists,
}

// Definir estructuras adicionales si es necesario
