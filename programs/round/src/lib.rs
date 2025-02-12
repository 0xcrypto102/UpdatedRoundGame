pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
use instructions::*;
pub use state::*;

declare_id!("HmWmNgFLYnaRBPvoqK6YrLpKasYmnS7UkiqjFYgGLuUP");

#[program]
pub mod round {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, slot_token_price: u64, fee: u32,) -> Result<()> {
        instructions::initialize(ctx,slot_token_price, fee)
    }

    pub fn update_fee(ctx: Context<Update>, new_fee: u32) -> Result<()> {
        instructions::update_fee(ctx,new_fee)
    }

    pub fn create_round(ctx: Context<CreateRound>, round_index: u16) -> Result<()> {
        instructions::create_round(ctx,round_index)
    }

    pub fn deactive_chad_mod(ctx: Context<ManageUserInfo>) -> Result<()> {
        instructions::deactive_chad_mod(ctx)
    }

    pub fn buy_slot(ctx: Context<BuySlot>, round_index: u16, amount: u32, method: bool) -> Result<()> {
        instructions::buy_slot(ctx,round_index, amount, method)
    }

    pub fn claim_slot(ctx: Context<ClaimSlot>) -> Result<()> {
        instructions::claim_slot(ctx)
    }

    pub fn withdraw_sol(ctx: Context<WithDrawSOL>, amount: u64) -> Result<()> {
        instructions::withdraw_sol(ctx, amount)
    }
}
