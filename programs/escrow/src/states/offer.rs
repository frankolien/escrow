use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
//#[account(discriminator = 1)]
pub struct Offer{
    // a uniques number to differentiaite this offer from others by the same user 
    pub id: u64,
    // alcie wallet address
    pub maker: Pubkey,
    // which tokens being traded
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    //how much of token b the maker(token a) wants
    pub token_b_wanted_amount: u64,
    // the bump seed used to derive the pda for this offer account 
    pub bump: u8,
}