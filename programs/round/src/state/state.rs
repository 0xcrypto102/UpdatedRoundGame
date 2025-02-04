use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct GlobalState {
    pub owner: Pubkey, 
    pub total_round: u16,
    pub slot_token_price: u64,
    pub vault: Pubkey,
    pub fee: u32,
}

#[account]
#[derive(Default)]
pub struct RoundState {
    pub round_index: u16,
    pub total_slot_number: u32,
    pub current_slot_number: u32,
    pub chad_users: Vec<UserInfoData>
}


#[account]
pub struct UserInfo {
    pub address: Pubkey,

    pub claimable_slot_number: u32,  // until last - 2 buy slot number
    pub wait_slot_number: u32, // last - 1 buy slot number
    pub last_slot_number: u32, // last buy slot number
    pub remain_slot_number: u32,
    pub last_round_index: u16,
    
    pub claimed_slot_number: u32,
    pub fee_amount: u32,
    pub reference: Pubkey
}

#[derive(Default, AnchorSerialize, AnchorDeserialize, Clone)]
pub struct UserInfoData {
    pub address: Pubkey,

    pub chad_wait_slot_number: u32,
    pub chad_last_slot_number: u32,
    pub chad_remain_slot_number: u32,
    pub chad_last_round_index: u16,

    pub fee_amount: u32,
}