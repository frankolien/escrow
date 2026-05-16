use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{close_account, CloseAccount, Mint, TokenAccount, TokenInterface},
};

use crate::states::offer::Offer;
use super::shared::transfer_tokens;

#[derive(Accounts)]
pub struct RefundOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mint::token_program = token_program
    )]
    pub token_mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_token_account_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = token_mint_a,
        seeds = [b"offer", maker.key().as_ref(), offer.id.to_le_bytes().as_ref()],
        bump = offer.bump,
    )]
    pub offer: Account<'info, Offer>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn refund_offer(context: Context<RefundOffer>) -> Result<()> {
    let maker_key = context.accounts.maker.key();
    let id_bytes = context.accounts.offer.id.to_le_bytes();
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"offer",
        maker_key.as_ref(),
        id_bytes.as_ref(),
        &[context.accounts.offer.bump],
    ]];

    // return all token A from vault back to the maker
    transfer_tokens(
        &context.accounts.vault,
        &context.accounts.maker_token_account_a,
        &context.accounts.vault.amount,
        &context.accounts.token_mint_a,
        &context.accounts.offer.to_account_info(),
        &context.accounts.token_program,
        Some(signer_seeds),
    )?;

    // close vault, returning rent to maker
    let cpi_accounts = CloseAccount {
        account: context.accounts.vault.to_account_info(),
        destination: context.accounts.maker.to_account_info(),
        authority: context.accounts.offer.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        context.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );
    close_account(cpi_ctx)
}
