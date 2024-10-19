use super::*;

pub fn pause_sale(ctx: Context<PauseSale>) -> Result<()> {
    let sale = &mut ctx.accounts.sale;
    sale.paused = true;
    Ok(())
}

pub fn unpause_sale(ctx: Context<UnpauseSale>) -> Result<()> {
    let sale = &mut ctx.accounts.sale;
    sale.paused = false;
    Ok(())
}

#[derive(Accounts)]
pub struct PauseSale<'info> {
    #[account(mut)]
    pub sale: Account<'info, Presale>,
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct UnpauseSale<'info> {
    #[account(mut)]
    pub sale: Account<'info, Presale>,
    pub admin: Signer<'info>,
}
