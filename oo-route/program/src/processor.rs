//! Program state processor

use crate::instruction::OOSwapInstruction;

use spl_token::state::Account;
use spl_token_swap::error::SwapError;
use spl_token_swap::instruction::{swap, Swap};

use crate::state::OOSwapStruct;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_pack::Pack,
    pubkey::Pubkey,
};

/// Program state handler.
pub struct Processor {}
impl Processor {
    /// Unpacks a spl_token `Account`.
    pub fn unpack_token_account(
        account_info: &AccountInfo,
        token_program_id: &Pubkey,
    ) -> Result<spl_token::state::Account, SwapError> {
        if account_info.owner != token_program_id {
            Err(SwapError::IncorrectTokenProgramId)
        } else {
            spl_token::state::Account::unpack(&account_info.data.borrow())
                .map_err(|_| SwapError::ExpectedAccount)
        }
    }
    /// Calculates the authority id by generating a program address.
    pub fn authority_id(
        program_id: &Pubkey,
        my_info: &Pubkey,
        nonce: u8,
    ) -> Result<Pubkey, SwapError> {
        Pubkey::create_program_address(&[&my_info.to_bytes()[..32], &[nonce]], program_id)
            .or(Err(SwapError::InvalidProgramAddress))
    }
    /// process_swap
    pub fn process_swap(
        _program_id: &Pubkey,
        swap_info_len: u8,
        amounts_in: Vec<u64>,
        minimum_amount_out: u64,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        // 用户相关的account info
        let user_transfer_authority_info = next_account_info(account_info_iter)?;
        let source_info = next_account_info(account_info_iter)?;
        let destination_info = next_account_info(account_info_iter)?;
        let swap_program_info = next_account_info(account_info_iter)?;
        let token_program_info = next_account_info(account_info_iter)?;

        let old_destination_account = Account::unpack(&destination_info.data.borrow())?;
        let old_destination_balance = old_destination_account.amount;

        //获取 swap info相关的信息
        for i in (0..swap_info_len).into_iter() {
            let swap_info = next_account_info(account_info_iter)?;
            let authority_info = next_account_info(account_info_iter)?;
            let swap_source_info = next_account_info(account_info_iter)?;
            let swap_destination_info = next_account_info(account_info_iter)?;
            let pool_mint_info = next_account_info(account_info_iter)?;
            let pool_fee_account_info = next_account_info(account_info_iter)?;

            let ix = swap(
                swap_program_info.key, //TODO 这个 是不是应该修改成 调用的合约的地址
                token_program_info.key,
                swap_info.key,
                authority_info.key,
                user_transfer_authority_info.key,
                source_info.key,
                swap_source_info.key,
                swap_destination_info.key,
                destination_info.key,
                pool_mint_info.key,
                pool_fee_account_info.key,
                None,
                Swap {
                    amount_in: amounts_in[i as usize],
                    minimum_amount_out: 0,
                },
            )?;
            let res = invoke(
                &ix,
                &[
                    swap_info.clone(),
                    authority_info.clone(),
                    user_transfer_authority_info.clone(),
                    source_info.clone(),
                    swap_source_info.clone(),
                    swap_destination_info.clone(),
                    destination_info.clone(),
                    pool_mint_info.clone(),
                    pool_fee_account_info.clone(),
                    token_program_info.clone(),
                ],
            );
            if res.is_err() {
                return res;
            }
        }

        let new_destination_account = Account::unpack(&destination_info.data.borrow())?;
        let new_destination_balance = new_destination_account.amount;
        if (new_destination_balance - old_destination_balance) < minimum_amount_out {
            return Err(SwapError::ExceededSlippage.into());
        }

        return Ok(());
    }

    /// Processes an instruction given extra constraint
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        let instruction = OOSwapInstruction::unpack(input)?;
        match instruction {
            OOSwapInstruction::OOSwap(OOSwapStruct {
                swap_info_len,
                amounts_in,
                minimum_amount_out,
            }) => {
                msg!("Instruction: OOSwap");
                Self::process_swap(
                    program_id,
                    swap_info_len,
                    amounts_in,
                    minimum_amount_out,
                    accounts,
                )
            }
        }
    }
}
