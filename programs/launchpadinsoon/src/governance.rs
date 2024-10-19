use super::*;

#[derive(Accounts)]
pub struct UpdateParameters<'info> {
    #[account(mut)]
    pub sale: Account<'info, Presale>,
    pub admin: Signer<'info>,
}

pub fn update_parameters(ctx: Context<UpdateParameters>, new_price: Option<u64>, new_total_tokens: Option<u64>) -> Result<()> {
    let sale = &mut ctx.accounts.sale;
    if let Some(price) = new_price {
        sale.price = price;
    }
    if let Some(total_tokens) = new_total_tokens {
        sale.total_tokens = total_tokens;
    }
    Ok(())
}

#[derive(Accounts)]
pub struct PauseContract<'info> {
    #[account(mut)]
    pub sale: Account<'info, Presale>,
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct UnpauseContract<'info> {
    #[account(mut)]
    pub sale: Account<'info, Presale>,
    pub admin: Signer<'info>,
}

pub fn pause_contract(ctx: Context<PauseContract>) -> Result<()> {
    let sale = &mut ctx.accounts.sale;
    sale.paused = true;
    Ok(())
}

pub fn unpause_contract(ctx: Context<UnpauseContract>) -> Result<()> {
    let sale = &mut ctx.accounts.sale;
    sale.paused = false;
    Ok(())
}
