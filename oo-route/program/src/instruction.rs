//! Instruction types

#![allow(clippy::too_many_arguments)]

use crate::error::SwapError;
use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use crate::state::OOSwapStruct;
#[cfg(feature = "fuzz")]
use arbitrary::Arbitrary;

/// Instructions supported by the token swap program.
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum OOSwapInstruction {
    ///   CalculateSwapReturn the tokens in the pool.
    OOSwap(OOSwapStruct),
}

impl OOSwapInstruction {
    /// Unpacks a byte buffer into a [SwapInstruction](enum.SwapInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input.split_first().ok_or(SwapError::InvalidInstruction)?;
        Ok(match tag {
            0 => {
                let (&swap_info_len, rest) =
                    rest.split_first().ok_or(SwapError::InvalidInstruction)?;
                if rest.len() % 8 != 0 {
                    return Err(SwapError::InvalidInstruction.into());
                }
                let size = rest.len() / 8 - 1;
                if size as u8 != swap_info_len {
                    return Err(SwapError::InvalidInstruction.into());
                }

                let mut amounts_in = vec![];
                let mut outer_rest = rest;
                for _ in (0..size).into_iter() {
                    let (amount_in, rest) = Self::unpack_u64(outer_rest)?;
                    amounts_in.push(amount_in);
                    outer_rest = rest;
                }
                let (minimum_amount_out, _rest) = Self::unpack_u64(outer_rest)?;
                Self::OOSwap(OOSwapStruct {
                    swap_info_len,
                    amounts_in,
                    minimum_amount_out,
                })
            }
            _ => return Err(SwapError::InvalidInstruction.into()),
        })
    }

    fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        if input.len() >= 8 {
            let (amount, rest) = input.split_at(8);
            let amount = amount
                .get(..8)
                .and_then(|slice| slice.try_into().ok())
                .map(u64::from_le_bytes)
                .ok_or(SwapError::InvalidInstruction)?;
            Ok((amount, rest))
        } else {
            Err(SwapError::InvalidInstruction.into())
        }
    }
}
