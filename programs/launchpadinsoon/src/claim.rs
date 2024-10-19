use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer as TokenTransfer};
use crate::{Presale, PresaleError};

#[derive(Accounts)]
pub struct ClaimTokens<'info> {
    #[account(mut)]
    pub presale: Account<'info, Presale>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn claim_tokens(ctx: Context<ClaimTokens>) -> Result<()> {
    let presale = &mut ctx.accounts.presale;
    let user_key = ctx.accounts.user.key();

    // Obtener la cantidad a reclamar
    let amount = presale.vestings.iter()
        .find(|(pubkey, _)| pubkey == &user_key)
        .map(|(_, vesting)| vesting.amount)
        .unwrap_or(0);

    require!(amount > 0, PresaleError::NoTokensToClaim);

    // Transferir tokens del token_vault a la cuenta del usuario
    let presale_key = presale.key(); // Guardar el valor en una variable
    let authority_seeds = &[presale_key.as_ref(), &[presale.bump]];
    let signer = &[&authority_seeds[..]];
    
    let cpi_accounts = TokenTransfer {
        from: ctx.accounts.token_vault.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: presale.to_account_info(), // Usa la variable mutable
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    token::transfer(cpi_ctx, amount)?;

    // Actualizar el estado de reclamación del usuario
    if let Some((_pubkey, ref mut claimed)) = presale.claims.iter_mut().find(|(pubkey, _)| pubkey == &user_key) {
        *claimed = true;
    } else {
        // Verificar si hay espacio suficiente antes de añadir una nueva entrada
        require!(
            {
                let current_len = presale.claims.len() as u64;
                current_len < presale.max_entries
            },
            PresaleError::InsufficientSpace
        );
        
        presale.claims.push((user_key, true));
    }

    Ok(())
}
