use super::*;

pub fn initialize(ctx: Context<Initialize>, total_tokens: u64, price: u64) -> Result<()> {
    let sale = &mut ctx.accounts.sale;
    sale.admin = *ctx.accounts.admin.key;
    sale.total_tokens = total_tokens;
    sale.tokens_sold = 0;
    sale.price = price;
    sale.paused = false;
    sale.whitelist = Vec::new();
    sale.allocations = Vec::new();
    sale.buyer_purchases = Vec::new();
    sale.vestings = Vec::new();
    Ok(())
}

