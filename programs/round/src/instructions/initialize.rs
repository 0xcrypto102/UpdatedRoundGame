use crate::{constants::*, error::*, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use solana_program::{program::{invoke, invoke_signed}, system_instruction};

use std::mem::size_of;

pub fn initialize(ctx: Context<Initialize>, slot_token_price: u64, fee: u32) -> Result<()> {
    let accts = ctx.accounts;
    require!(fee < 100, RoundError::MaxFeeError);

    // init the global state account
    accts.global_state.owner = accts.owner.key();
    accts.global_state.total_round = 0;
    accts.global_state.slot_token_price = slot_token_price;
    accts.global_state.vault = accts.vault.key();
    accts.global_state.fee = fee;

    Ok(())
}

pub fn update_fee(ctx: Context<Update>, new_fee: u32) -> Result<()> {
    let accts = ctx.accounts;
    require!(accts.global_state.owner == accts.owner.key(), RoundError::NotAllowedOwner);

    accts.global_state.fee = new_fee;
    Ok(())
}

pub fn create_round(ctx: Context<CreateRound>, round_index: u16) -> Result<()> {
    let accts = ctx.accounts;

    require!(accts.global_state.owner == accts.owner.key(), RoundError::NotAllowedOwner);
    require!(accts.global_state.total_round + 1 == round_index, RoundError::InvalidRoundIndex);
    require!(accts.round.total_slot_number == accts.round.current_slot_number, RoundError::RoundNotFinished);

    let current_index = accts.global_state.total_round;
    // create the new round
    accts.round.round_index = current_index + 1;
    accts.round.total_slot_number = 2_u32.pow((round_index - 1).into());
    accts.round.current_slot_number = 0;
    // update the global state
    accts.global_state.total_round += 1;
    let mut temp_slot_amount = 0;
   
    // Check the chad mod users and auto claim & buy slots
    // Iterate through chad users and update their information
    for chad_user in accts.round.chad_users.iter_mut() {
        let available_amount = (chad_user.chad_wait_slot_number * (2000 - accts.global_state.fee * 2) + chad_user.chad_remain_slot_number *  (1000 - accts.global_state.fee) / 1000) / 1000;
        let remain_slot_number = chad_user.chad_wait_slot_number * (2000 - accts.global_state.fee * 2) + chad_user.chad_remain_slot_number - 1000 * available_amount.clone();

        temp_slot_amount += available_amount;

        chad_user.fee_amount += chad_user.chad_wait_slot_number * accts.global_state.fee * 2;
        chad_user.chad_wait_slot_number = chad_user.chad_last_slot_number;
        chad_user.chad_last_slot_number = available_amount;
        chad_user.chad_remain_slot_number = remain_slot_number;
        chad_user.chad_last_round_index = round_index;
    }

    accts.round.current_slot_number = temp_slot_amount;

    Ok(())
}

pub fn deactive_chad_mod(ctx: Context<ManageUserInfo>) -> Result<()> {
    let accts = ctx.accounts;

    // If no longer a chad, update user_info with the corresponding info from `chad_users`
    if let Some(index) = accts.round.chad_users.iter().position(|user| user.address == accts.user_info.address) {
        let removed_user = accts.round.chad_users[index].clone();

        if accts.user_info.last_round_index == removed_user.chad_last_round_index {
            // Update `user_info` with specific fields from the removed user
            accts.user_info.wait_slot_number += removed_user.chad_wait_slot_number;
            accts.user_info.last_slot_number += removed_user.chad_last_slot_number;
            accts.user_info.remain_slot_number += removed_user.chad_remain_slot_number;
            accts.user_info.fee_amount += removed_user.fee_amount;
        } else if accts.user_info.last_round_index == removed_user.chad_last_round_index - 1 {
            // Update `user_info` with specific fields from the removed user
            accts.user_info.claimable_slot_number += accts.user_info.wait_slot_number;
            accts.user_info.wait_slot_number = accts.user_info.last_slot_number + removed_user.chad_wait_slot_number;
            accts.user_info.last_slot_number = removed_user.chad_last_slot_number;
            accts.user_info.last_round_index = removed_user.chad_last_round_index;
            accts.user_info.remain_slot_number += removed_user.chad_remain_slot_number;
            accts.user_info.fee_amount += removed_user.fee_amount;
        } else {
            accts.user_info.claimable_slot_number += accts.user_info.last_slot_number + accts.user_info.wait_slot_number;
            accts.user_info.wait_slot_number = removed_user.chad_wait_slot_number;
            accts.user_info.last_slot_number = removed_user.chad_last_slot_number;
            accts.user_info.remain_slot_number += removed_user.chad_remain_slot_number;
            accts.user_info.fee_amount += removed_user.fee_amount;
            accts.user_info.last_round_index = removed_user.chad_last_round_index;
        }
        // Remove the user from the `chad_users` list
        accts.round.chad_users.remove(index);
        msg!("removed user index {:?}", index);
    }
    Ok(())
}

pub fn buy_slot(ctx: Context<BuySlot>, round_index: u16, amount: u32, method: bool) -> Result<()> {
    let accts = ctx.accounts;

    require!(amount > 0, RoundError::ZeroAmount);
    require!(round_index == accts.round.round_index, RoundError::InvalidRoundIndex);
    require!(accts.global_state.total_round == round_index, RoundError::InvalidRoundIndex);
    require!(accts.round.current_slot_number + amount <= accts.round.total_slot_number, RoundError::OverMaxSlot);
    // require!(accts.user_info.last_round_index < round_index, RoundError::AlreadyBuySlot);
    
    msg!("Current round's solt number is {:?} and total slot is {:?}",accts.round.current_slot_number, accts.round.total_slot_number);

    if method {
        msg!("Buying slots with chad mod...");
        require!(amount >= 4, RoundError::SmallAmount);
        if let Some(user) = accts
            .round
            .chad_users
            .iter_mut()
            .find(|user| user.address == accts.user_info.address) {
            // Modify the existing user directly in `accts.round.chad_users`
            user.chad_last_slot_number += amount;
        } else {
            // If user doesn't exist, create a new one and push it to the vector
            let user_info_data = UserInfoData {
                address: accts.user.key(),
                chad_wait_slot_number: 0,
                chad_last_slot_number: amount,
                chad_remain_slot_number: 0,
                chad_last_round_index: round_index,
                fee_amount: 0,
            };

            if accts.user_info.address != accts.user.key() {
                accts.user_info.address = accts.user.key();
                accts.user_info.reference = accts.reference.key();
                accts.user_info.claimable_slot_number = 0;
                accts.user_info.wait_slot_number = 0;
                accts.user_info.claimed_slot_number = 0;
                accts.user_info.last_slot_number = 0;
                accts.user_info.last_round_index = 0;
                accts.user_info.remain_slot_number = 0;
                accts.user_info.fee_amount = 0;
            }

            accts.round.chad_users.push(user_info_data);
        }
    } else {
        // update the user info data
        if accts.user_info.last_round_index == 0 {
            accts.user_info.address = accts.user.key();
            accts.user_info.last_slot_number = amount;
            accts.user_info.claimable_slot_number = 0;
            accts.user_info.wait_slot_number = 0;
            accts.user_info.claimed_slot_number = 0;
            accts.user_info.last_round_index = round_index;
            accts.user_info.fee_amount = 0;
            accts.user_info.reference = accts.reference.key();
        } else {
            if accts.user_info.last_round_index == round_index {
                accts.user_info.last_slot_number += amount;
            } else if accts.user_info.last_round_index == round_index - 1 {
                accts.user_info.claimable_slot_number += accts.user_info.wait_slot_number;
                accts.user_info.wait_slot_number = accts.user_info.last_slot_number;
                accts.user_info.last_slot_number = amount;
                accts.user_info.last_round_index = round_index;
            } else {
                accts.user_info.claimable_slot_number += accts.user_info.last_slot_number + accts.user_info.wait_slot_number;
                accts.user_info.last_slot_number = amount;
                accts.user_info.last_round_index = round_index;
            }
        }
    }
    // update the round data
    accts.round.current_slot_number += amount;

    // send sol to vault
    let transfer_amount = accts.global_state.slot_token_price * amount as u64;

    invoke(
        &system_instruction::transfer(
            &accts.user.key(),
            &accts.vault.key(),
            transfer_amount
        ),
        &[
            accts.user.to_account_info().clone(),
            accts.vault.clone(),
            accts.system_program.to_account_info().clone(),
        ],
    )?;

    Ok(())
}

pub fn claim_slot(ctx: Context<ClaimSlot>) -> Result<()> {
    let accts = ctx.accounts;
    let mut amount = 0;
    let mut fee_amount = 0;

    require!(accts.user_info.address == accts.user.key(), RoundError::NotAllowedOwner);
    require!(accts.global_state.owner == accts.owner.key(), RoundError::NotAllowedOwner);
    require!(accts.user_info.reference == accts.reference.key(), RoundError::InvalidReference);
   
    if accts.global_state.total_round == accts.user_info.last_round_index {
        let mut claim_amount = accts.user_info.claimable_slot_number * 2000;
        claim_amount += accts.user_info.remain_slot_number;

        fee_amount = accts.user_info.fee_amount as u64 * accts.global_state.slot_token_price / 1000 + claim_amount.clone() as u64 * accts.global_state.fee as u64 * accts.global_state.slot_token_price / 1000000;

        amount = claim_amount.clone() as u64 * (1000 - accts.global_state.fee as u64) * accts.global_state.slot_token_price / 1000000;

        msg!("the claim amount is {:?}", amount);
        msg!("the claim fee amount is {:?}", fee_amount);

        accts.user_info.claimable_slot_number = 0;
        accts.user_info.fee_amount = 0;
        accts.user_info.remain_slot_number = 0;
        accts.user_info.claimed_slot_number += amount as u32;
    } else if accts.global_state.total_round == accts.user_info.last_round_index - 1 {
        let mut claim_amount = (accts.user_info.claimable_slot_number + accts.user_info.wait_slot_number) * 2000;
        claim_amount += accts.user_info.remain_slot_number;

        fee_amount = accts.user_info.fee_amount as u64 * accts.global_state.slot_token_price / 1000 + claim_amount.clone() as u64 * accts.global_state.fee as u64 * accts.global_state.slot_token_price / 1000000;

        amount = claim_amount.clone() as u64 * (1000 - accts.global_state.fee as u64) * accts.global_state.slot_token_price / 1000000;

        msg!("the claim amount is {:?}", amount);
        msg!("the claim fee amount is {:?}", fee_amount);

        accts.user_info.claimable_slot_number = 0;
        accts.user_info.fee_amount = 0;
        accts.user_info.remain_slot_number = 0;
        accts.user_info.claimed_slot_number += amount as u32;
    } else {
        let mut claim_amount = (accts.user_info.claimable_slot_number + accts.user_info.wait_slot_number + accts.user_info.last_slot_number) * 2000;
        claim_amount += accts.user_info.remain_slot_number;

        fee_amount =  accts.user_info.fee_amount as u64 * accts.global_state.slot_token_price / 1000  + claim_amount.clone() as u64 * accts.global_state.fee as u64 * accts.global_state.slot_token_price / 1000000;

        amount = claim_amount.clone() as u64 * (1000 - accts.global_state.fee as u64) * accts.global_state.slot_token_price / 1000000;

        msg!("The claim amount is {:?}", amount);
        msg!("The claim fee amount is {:?}", fee_amount);

        accts.user_info.claimable_slot_number = 0;
        accts.user_info.wait_slot_number = 0;
        accts.user_info.fee_amount = 0;
        accts.user_info.remain_slot_number = 0;
        accts.user_info.last_slot_number = 0;
        accts.user_info.claimed_slot_number += amount as u32;
    }

    let (_, bump) = Pubkey::find_program_address(&[VAULT_SEED], &crate::ID);

    invoke_signed(
        &system_instruction::transfer(&accts.vault.key(), &accts.user.key(), amount),
        &[
            accts.vault.to_account_info().clone(),
            accts.user.to_account_info().clone(),
            accts.system_program.to_account_info().clone(),
        ],
        &[&[VAULT_SEED, &[bump]]],
    )?;

    invoke_signed(
        &system_instruction::transfer(&accts.vault.key(), &accts.owner.key(), fee_amount  / 5),
        &[
            accts.vault.to_account_info().clone(),
            accts.owner.to_account_info().clone(),
            accts.system_program.to_account_info().clone(),
        ],
        &[&[VAULT_SEED, &[bump]]],
    )?;

    invoke_signed(
        &system_instruction::transfer(&accts.vault.key(), &accts.reference.key(), fee_amount * 4 / 5),
        &[
            accts.vault.to_account_info().clone(),
            accts.reference.to_account_info().clone(),
            accts.system_program.to_account_info().clone(),
        ],
        &[&[VAULT_SEED, &[bump]]],
    )?;

    Ok(())
}

pub fn withdraw_sol(ctx: Context<WithDrawSOL>, amount:u64) -> Result<()> {
    let accts = ctx.accounts;

    require!(accts.global_state.owner == accts.owner.key(), RoundError::NotAllowedOwner);

    let (_, bump) = Pubkey::find_program_address(&[VAULT_SEED], &crate::ID);

    invoke_signed(
        &system_instruction::transfer(&accts.vault.key(), &accts.owner.key(), amount),
        &[
            accts.vault.to_account_info().clone(),
            accts.owner.to_account_info().clone(),
            accts.system_program.to_account_info().clone(),
        ],
        &[&[VAULT_SEED, &[bump]]],
    )?;

    Ok(())
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        seeds = [GLOBAL_STATE_SEED], 
        bump,
        space = 8 + size_of::<GlobalState>()
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds = [VAULT_SEED],
        bump
    )]
    /// CHECK: this should be set by admin
    pub vault: AccountInfo<'info>,  // to receive SOL

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED], 
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct CreateRound<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED], 
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        init_if_needed,
        payer = owner,
        seeds = [ROUND_SEED],
        bump, 
        space = 9600
    )]
    pub round: Account<'info, RoundState>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct BuySlot<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub owner: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED], 
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds = [ROUND_SEED],
        bump, 
    )]
    pub round: Account<'info, RoundState>,

    #[account(
        mut,
        address = global_state.vault
    )]
    /// CHECK: this should be set by admin
    pub vault: AccountInfo<'info>,  // to receive SOL

    /// CHECK
    pub reference: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = user,
        seeds = [ROUN_USER_INFO_SEED, user.key().as_ref()],
        bump,
        space = 8 + size_of::<UserInfo>()
    )]
    pub user_info: Account<'info, UserInfo>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct ClaimSlot<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED], 
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        mut,
    )]
    /// CHECK: this should be set by admin
    pub owner: AccountInfo<'info>,  // To send fee to owner

    #[account(
        mut,
        address = global_state.vault
    )]
    /// CHECK: this should be set by admin
    pub vault: AccountInfo<'info>,  // to receive SOL

    #[account(
        mut,
    )]
    /// CHECK
    pub reference: AccountInfo<'info>, 

    #[account(
        mut,
        seeds = [ROUN_USER_INFO_SEED, user.key().as_ref()],
        bump,
    )]
    pub user_info: Account<'info, UserInfo>,

    #[account(
        mut,
        seeds = [ROUND_SEED],
        bump, 
    )]
    pub round: Account<'info, RoundState>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct WithDrawSOL<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED], 
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        mut,
        address = global_state.vault
    )]
    /// CHECK: this should be set by admin
    pub vault: AccountInfo<'info>,  // to receive SOL

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct ManageUserInfo<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [ROUN_USER_INFO_SEED, user.key().as_ref()],
        bump,
    )]
    pub user_info: Account<'info, UserInfo>,

    #[account(
        mut,
        seeds = [ROUND_SEED],
        bump, 
    )]
    pub round: Account<'info, RoundState>,
}
