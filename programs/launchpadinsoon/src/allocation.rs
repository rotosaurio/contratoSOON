use super::*;

pub fn set_allocation(ctx: Context<SetAllocation>, user: Pubkey, allocation: u64) -> Result<()> {
    let sale = &mut ctx.accounts.sale;
    if let Some((_, existing_allocation)) = sale.allocations.iter_mut().find(|(pubkey, _)| pubkey == &user) {
        *existing_allocation = allocation;
    } else {
        sale.allocations.push((user, allocation));
    }
    Ok(())
}

#[derive(Accounts)]
pub struct SetAllocation<'info> {
    #[account(mut)]
    pub sale: Account<'info, Sale>,
    pub admin: Signer<'info>,
}
