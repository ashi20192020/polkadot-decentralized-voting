#![cfg_attr(not(feature = "std"), no_std)]

pub mod macros;

use sp_runtime::traits::Saturating;

/// Balance of an account.
pub type Balance = u128;

pub type ProposalId = u32;

pub trait Incrementable {
	fn increment(&self) -> Self;
	fn initial_value() -> Self;
}

impl_incrementable!(u16, u32, u64, u128, i16, i32, i64, i128);
