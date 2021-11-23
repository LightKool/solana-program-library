#![deny(missing_docs)]

//! An Uniswap-like program for the Solana blockchain.

pub mod constraints;
pub mod curve;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
mod utils;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;

// solana_program::declare_id!("3dSBFmJ6zvMmeDCXrP1N9CXBYs65rbHxpCRRyHSRgyZD");
solana_program::declare_id!("9zt2vgp2JMDwiN3cQt36g2JiZXwQcURoLzZQmnuMGNTo");
