use super::*;
use anchor_spl::token::{self, Token, Transfer};

pub fn buy_tokens(ctx: Context<BuyTokens>, amount: u64) -> Result<()> {
    let sale = &mut ctx.accounts.sale;

    require!(!sale.paused, SaleError::SalePaused);
    require!(sale.whitelist.contains(&ctx.accounts.buyer.key()), SaleError::NotWhitelisted);
    
    let allocation = sale.allocations.iter()
        .find(|(pubkey, _)| pubkey == &ctx.accounts.buyer.key())
        .map(|(_, allocation)| *allocation)
        .unwrap_or(0);
    
    let buyer_purchase = sale.buyer_purchases.iter()
        .find(|(pubkey, _)| pubkey == &ctx.accounts.buyer.key())
        .map(|(_, purchase)| *purchase)
        .unwrap_or(0);
    
    require!(amount <= allocation.saturating_sub(buyer_purchase), SaleError::AllocationExceeded);
    let cost = sale.price.checked_mul(amount).ok_or(SaleError::CalculationError)?;

    **ctx.accounts.buyer.to_account_info().lamports.borrow_mut() -= cost;
    **ctx.accounts.treasury.to_account_info().lamports.borrow_mut() += cost;

    let cpi_accounts = Transfer {
        from: ctx.accounts.token_vault.to_account_info(),
        to: ctx.accounts.buyer_token_account.to_account_info(),
        authority: ctx.accounts.sale_authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    sale.tokens_sold = sale.tokens_sold.checked_add(amount).ok_or(SaleError::CalculationError)?;
    
    if let Some((_, purchase)) = sale.buyer_purchases.iter_mut().find(|(pubkey, _)| pubkey == &ctx.accounts.buyer.key()) {
        *purchase = purchase.checked_add(amount).ok_or(SaleError::CalculationError)?;
    } else {
        sale.buyer_purchases.push((ctx.accounts.buyer.key(), amount));
    }

    Ok(())
}

#[derive(Accounts)]
pub struct BuyTokens<'info> {
    #[account(mut)]
    pub sale: Account<'info, Sale>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub buyer_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub token_vault: Account<'info, token::TokenAccount>,
    /// CHECK: This is the treasury account that receives the payment
    #[account(mut)]
    pub treasury: AccountInfo<'info>,
    /// CHECK: This is the sale authority
    pub sale_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
