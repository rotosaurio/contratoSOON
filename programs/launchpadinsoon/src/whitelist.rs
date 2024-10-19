use super::*;

pub fn add_to_whitelist(ctx: Context<AddToWhitelist>, user: Pubkey) -> Result<()> {
    let sale = &mut ctx.accounts.sale;
    if !sale.whitelist.contains(&user) {
        sale.whitelist.push(user);
    }
    Ok(())
}

#[derive(Accounts)]
pub struct AddToWhitelist<'info> {
    #[account(mut)]
    pub sale: Account<'info, Presale>,
    pub admin: Signer<'info>,
}
