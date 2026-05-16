use anchor_lang::prelude::*;

pub mod states;
pub mod instructions;

use instructions::*;

declare_id!("BfvAstRx4vfgffP7PmoFKZYdRZ66MeexN6sfAtGrBs2w");

#[program]
pub mod escrow {
    use super::*;

    pub fn make_offer(
        context: Context<MakeOffer>,
        id: u64,
        token_a_offered_amount: u64,
        token_b_wanted_amount: u64,
    ) -> Result<()> {
        instructions::make_offer::make_offer(context, id, token_a_offered_amount, token_b_wanted_amount)
    }

    pub fn take_offer(context: Context<TakeOffer>) -> Result<()> {
        instructions::take_offer::take_offer(context)
    }

    pub fn refund_offer(context: Context<RefundOffer>) -> Result<()> {
        instructions::refund_offer::refund_offer(context)
    }
}
