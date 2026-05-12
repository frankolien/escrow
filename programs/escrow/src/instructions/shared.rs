use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

pub fn transfer_tokens<'info>(
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    amount: &u64,
    mint: &InterfaceAccount<'info, Mint>,
    authority: &AccountInfo<'info>,
    token_program: &Interface<'info, TokenInterface>,
    signer_seeds: Option<&[&[&[u8]]]>,
) -> Result<()> {
    let cpi_accounts = TransferChecked {
        from: from.to_account_info(),
        mint: mint.to_account_info(),
        to: to.to_account_info(),
        authority: authority.clone(),
    };

    let cpi_ctx = match signer_seeds {
        Some(seeds) => CpiContext::new_with_signer(
            token_program.to_account_info(),
            cpi_accounts,
            seeds,
        ),
        None => CpiContext::new(token_program.to_account_info(), cpi_accounts),
    };

    transfer_checked(cpi_ctx, *amount, mint.decimals)
}
