use super::*;
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = admin, space = 8 + Sale::LEN)]
    pub sale: Account<'info, Sale>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub token_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Sale {
    pub admin: Pubkey,
    pub token_mint: Pubkey,
    pub total_tokens: u64,
    pub tokens_sold: u64,
    pub price: u64,
    pub paused: bool,
    pub start_time: i64,
    pub end_time: i64,
    pub vesting_end_time: i64,
    pub raise_goal: u64,
    pub total_investors: u32,
    pub bump: u8,
}

impl Sale {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 8 + 8 + 1 + 8 + 8 + 8 + 8 + 4 + 1;
}

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
    let sale = &mut ctx.accounts.sale;
    sale.admin = *ctx.accounts.admin.key;
    sale.token_mint = ctx.accounts.token_mint.key();
    sale.total_tokens = total_tokens;
    sale.tokens_sold = 0;
    sale.price = price;
    sale.paused = false;
    sale.start_time = start_time;
    sale.end_time = end_time;
    sale.vesting_end_time = vesting_end_time;
    sale.raise_goal = raise_goal;
    sale.total_investors = 0;
    sale.bump = bump;

    Ok(())
}
