use codec::{Decode, Encode};
use frame_support::pallet_prelude::RuntimeDebug;
use frame_support::{pallet_prelude::Get, BoundedVec};
use scale_info::TypeInfo;

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(DescriptionLimit, NameLimit, AccountLimit))]
pub struct Proposal<DescriptionLimit: Get<u32>, NameLimit: Get<u32>, AccountId, BlockNumber> {
	pub proposer: AccountId,
	pub name: BoundedVec<u8, NameLimit>,
	pub description: BoundedVec<u8, DescriptionLimit>,
	pub status: ProposalResultStatus,
	pub proposal_duration: BlockNumber,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(AccountLimit))]
pub struct Vote<AccountId> {
	pub who: AccountId,
	pub choice: Choice,
}

/// Result of proposal.
#[derive(Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, Encode, Decode)]
pub enum ProposalResultStatus {
	/// Proposal is passed.
	Accepted,
	/// Proposal is rejected.
	Rejected,
	/// Proposal is started.
	Started,
}

/// Choice of the Vote.
#[derive(Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, Encode, Decode)]
pub enum Choice {
	/// If the choice is Yes.
	Yes,
	/// If the choice is No.
	No,
}
