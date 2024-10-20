use super::*;
use crate::GlobalStats;
use crate::presale::Presale;

#[derive(Accounts)]
pub struct GetPresaleStats<'info> {
    pub presale: Account<'info, Presale>,
}

#[derive(Accounts)]
pub struct GetUserStats<'info> {
    pub presale: Account<'info, Presale>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateGlobalStats<'info> {
    #[account(mut)]
    pub global_stats: Account<'info, GlobalStats>,
    pub admin: Signer<'info>,
    pub presale: Account<'info, Presale>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct PresaleStats {
    pub id: u64,
    pub raise_token: Pubkey,
    pub sale_token: Pubkey,
    pub total_tokens: u64,
    pub tokens_sold: u64,
    pub price: u64,
    pub total_raised: u64,
    pub total_investors: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub raise_goal: u64,
    pub is_active: bool,
    pub time_remaining: i64,
    pub percentage_sold: f64,
    pub percentage_raised: f64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct UserStats {
    pub allocation: u64,
    pub tokens_purchased: u64,
    pub vesting_amount: u64,
    pub vesting_release_time: i64,
    pub claimed: bool,
}

#[derive(Accounts)]
pub struct GetGlobalStats<'info> {
    pub global_stats: Account<'info, GlobalStats>,
}

#[derive(Accounts)]
pub struct InitializeGlobalStats<'info> {
    #[account(init, payer = admin, space = 8 + GlobalStats::LEN)]
    pub global_stats: Account<'info, GlobalStats>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_global_stats(ctx: Context<InitializeGlobalStats>) -> Result<()> {
    let global_stats = &mut ctx.accounts.global_stats;
    global_stats.total_presales = 0;
    global_stats.total_raised = 0;
    global_stats.total_investors = 0;
    global_stats.presales = Vec::new();
    Ok(())
}

pub fn update_global_stats(ctx: Context<UpdateGlobalStats>) -> Result<()> {
    let global_stats = &mut ctx.accounts.global_stats;
    let presale = &ctx.accounts.presale;
    
    if let Some(presale_info) = global_stats.presales.iter_mut().find(|p| p.id == presale.id) {
        presale_info.total_raised = presale.total_raised;
        presale_info.total_investors = presale.total_investors as u64; // Asegúrate de convertir a u64
    } else {
        require!(
            global_stats.presales.len() < GlobalStats::MAX_PRESALES,
            PresaleError::TooManyPresales
        );
        global_stats.presales.push(PresaleInfo {
            id: presale.id,
            sale_token: presale.sale_token, // Asegúrate de que este campo exista en PresaleInfo
            total_raised: presale.total_raised,
            total_investors: presale.total_investors as u64, // Asegúrate de convertir a u64
        });
        global_stats.total_presales += 1; // Asegúrate de que este campo exista
    }

    global_stats.total_raised = global_stats.presales.iter().map(|p| p.total_raised).sum();
    global_stats.total_investors = global_stats.presales.iter().map(|p| p.total_investors).sum(); // Cambiar a u64 si es necesario

    Ok(())
}

pub fn get_global_stats(ctx: Context<GetGlobalStats>) -> Result<GlobalStats> {
    Ok((*ctx.accounts.global_stats).clone())
}

pub fn get_presale_stats(ctx: Context<GetPresaleStats>) -> Result<PresaleStats> {
    let presale = &ctx.accounts.presale;
    let current_time = Clock::get()?.unix_timestamp;

    let is_active = current_time >= presale.start_time && current_time <= presale.end_time;
    let time_remaining = if is_active {
        presale.end_time - current_time
    } else {
        0
    };

    let percentage_sold = if presale.total_tokens > 0 {
        (presale.tokens_sold as f64 / presale.total_tokens as f64) * 100.0
    } else {
        0.0
    };

    let percentage_raised = if presale.raise_goal > 0 {
        (presale.total_raised as f64 / presale.raise_goal as f64) * 100.0
    } else {
        0.0
    };

    Ok(PresaleStats {
        id: presale.id,
        raise_token: presale.raise_token,
        sale_token: presale.sale_token,
        total_tokens: presale.total_tokens,
        tokens_sold: presale.tokens_sold,
        price: presale.price,
        total_raised: presale.total_raised,
        total_investors: presale.total_investors as u64, // Convertir a u64
        start_time: presale.start_time,
        end_time: presale.end_time,
        raise_goal: presale.raise_goal,
        is_active,
        time_remaining,
        percentage_sold,
        percentage_raised,
    })
}

pub fn get_user_stats(ctx: Context<GetUserStats>) -> Result<UserStats> {
    let presale = &ctx.accounts.presale;
    let user_key = ctx.accounts.user.key();

    let allocation = presale.allocations.iter()
        .find(|(pubkey, _)| pubkey == &user_key)
        .map(|(_, allocation)| *allocation)
        .unwrap_or(0);

    let tokens_purchased = presale.buyer_purchases.iter()
        .find(|(pubkey, _)| pubkey == &user_key)
        .map(|(_, amount)| *amount)
        .unwrap_or(0);

    let vesting_info = presale.vestings.iter()
        .find(|(pubkey, _)| pubkey == &user_key)
        .map(|(_, info)| info);

    let vesting_amount = vesting_info.map(|info| info.amount).unwrap_or(0);
    let vesting_release_time = vesting_info.map(|info| info.release_time).unwrap_or(0);
    let claimed = presale.claims.iter()
        .find(|(pubkey, _)| pubkey == &user_key)
        .map(|(_, claimed)| *claimed)
        .unwrap_or(false);

    Ok(UserStats {
        allocation,
        tokens_purchased,
        vesting_amount,
        vesting_release_time,
        claimed,
    })
}
