use super::*;

#[derive(Accounts)]
pub struct GetPresaleStats<'info> {
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
    pub total_investors: u32,
    pub start_time: i64,
    pub end_time: i64,
    pub raise_goal: u64,
    pub is_active: bool,
    pub time_remaining: i64,
}

pub fn get_presale_stats(ctx: Context<GetPresaleStats>, presale_id: u64) -> Result<PresaleStats> {
    let presale = &ctx.accounts.presale;
    require!(presale.id == presale_id, PresaleError::InvalidPresaleId);

    let current_time = Clock::get()?.unix_timestamp;

    Ok(PresaleStats {
        id: presale.id,
        raise_token: presale.raise_token,
        sale_token: presale.sale_token,
        total_tokens: presale.total_tokens,
        tokens_sold: presale.tokens_sold,
        price: presale.price,
        total_raised: presale.total_raised,
        total_investors: presale.total_investors,
        start_time: presale.start_time,
        end_time: presale.end_time,
        raise_goal: presale.raise_goal,
        is_active: current_time >= presale.start_time && current_time <= presale.end_time,
        time_remaining: presale.end_time.saturating_sub(current_time),
    })
}
