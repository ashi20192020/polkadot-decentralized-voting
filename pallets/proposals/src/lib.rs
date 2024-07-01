//! # Proposals Pallet
//!
//! A custom Substrate pallet that implements a decentralized voting system.
//! This pallet will allow users to create, vote on, and manage proposals on the blockchain.
//!
//!
//! ## Overview
//!
//! A Proposal is a way for the Community to propose a change.

//!
//! ## Functionalities
//!
//! * A founder can create a new proposal.
//! * Only founder can update proposal.
//! * Any token user can vote on an existing proposal
//!
//! ## Interface
//!
//! * `create_proposal`
//! * `cast_vote`
//! * `update_proposal`
//!

// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
use frame_support::traits::Incrementable;
use sp_std::vec::Vec;
use sp_std::vec;
use frame_system::{
	offchain::{
		AppCrypto, CreateSignedTransaction, SendSignedTransaction,
		Signer
	},
	pallet_prelude::BlockNumberFor,
};
use crate::ProposalResultStatus::{Accepted, Rejected};
pub use pallet::*;
use sp_core::crypto::KeyTypeId;

mod types;
use crate::types::{Choice, Proposal, ProposalResultStatus, Vote};
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"btc!");

/// Based on the above `KeyTypeId` we need to generate a pallet-specific crypto type wrappers.
/// We can use from supported crypto kinds (`sr25519`, `ed25519` and `ecdsa`) and augment
/// the types with this pallet-specific identifier.
pub mod crypto {
	use super::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify,
		MultiSignature, MultiSigner,
	};
	app_crypto!(sr25519, KEY_TYPE);

	pub struct TestAuthId;

	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}

	// implemented for mock runtime in test
	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
	for TestAuthId
	{
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_runtime::SaturatedConversion;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {

		fn offchain_worker(block_number: BlockNumberFor<T>) {

			log::info!("Hello World from offchain workers!");

			if (block_number.saturated_into::<u32>() % 40 as u32) != 0 {
				return;
			}

			let signer = Signer::<T, T::AuthorityId>::all_accounts();
			if !signer.can_sign() {
				log::error!("No OCW found.");
				return;
			}

			let results = signer.send_signed_transaction(|_account| {
				log::info!("Checking Proposals Now.....!");
				Call::pass_proposal {  }
			});

			for (acc, res) in &results {
				match res {
					Ok(()) => log::info!("[{:?}] Submitted ", acc.id),
					Err(e) => log::error!("[{:?}] Failed to submit transaction: {:?}", acc.id, e),
				}
			};

		}
	}

	#[pallet::config]
	pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {
		/// The identifier type for an offchain worker.
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
		/// The overarching runtime event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Identifier for the Proposal.
		type ProposalId: Member + Parameter + MaxEncodedLen + Copy + Incrementable;
		/// The maximum length of proposal name/title.
		#[pallet::constant]
		type NameLimit: Get<u32>;
		/// The maximum length of proposal description.
		#[pallet::constant]
		type DescriptionLimit: Get<u32>;
		/// A type representing the weights required by the dispatchables of this pallet.
		type WeightInfo: WeightInfo;
	}

	/// Store new proposal with a unique proposal id
	#[pallet::storage]
	#[pallet::getter(fn proposals)]
	pub type Proposals<T: Config> = StorageMap<
		_,
		Identity,
		T::ProposalId,
		Proposal<T::DescriptionLimit, T::NameLimit, T::AccountId, BlockNumberFor<T>>,
		OptionQuery,
	>;

	/// Stores the `ProposalId` that is going to be used for the next proposal.
	/// This gets incremented whenever a new proposal is created.
	#[pallet::storage]
	pub(super) type NextProposalId<T: Config> = StorageValue<_, T::ProposalId, OptionQuery>;

	/// Store votes submitted for a proposal
	#[pallet::storage]
	#[pallet::getter(fn votes)]
	pub type Votes<T: Config> =
		StorageMap<_, Blake2_128Concat, T::ProposalId, Vec<Vote<T::AccountId>>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Created Proposals [Proposal Id]
		CreatedProposal(T::ProposalId),
		/// Submitted Proposal [Proposal Id]
		VoteCasted(T::ProposalId),
		/// Proposal updated [Proposal Id]
		UpdatedProposal(T::ProposalId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Proposal Does Not Exist.
		ProposalDoesNotExist,
		/// NotAllowed
		NotAllowed,
		/// Invalid description given.
		BadDescription,
		/// Invalid proposal name given.
		BadName,
		/// Expired Proposal duration.
		ProposalExpired,
		/// Already voted.
		AlreadyVoted,
		/// Invalid Duration
		InvalidDuration,
		/// Vote does not exist.
		VoteDoesNotExist
	}


	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new proposal from a origin.
		///
		/// This new proposal has zero votes.
		///
		/// The origin must be Signed.
		///
		/// Parameters:
		/// - `name`: name/title of the proposal.
		/// - `description`: description of the proposal.
		/// - `proposal_duration`: Voting duration of the proposal.
		///
		/// Emits `CreatedProposal` event when successful.
		///
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn create_proposal(
			origin: OriginFor<T>,
			name: BoundedVec<u8, T::NameLimit>,
			description: BoundedVec<u8, T::DescriptionLimit>,
			proposal_duration: BlockNumberFor<T>,
		) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			let proposer_account = ensure_signed(origin)?;

			let current_block = <frame_system::Pallet<T>>::block_number();
			ensure!(proposal_duration > current_block, Error::<T>::InvalidDuration);
			let new_proposal = Proposal {
				proposer: proposer_account,
				name,
				description,
				status: ProposalResultStatus::Started,
				proposal_duration,
			};

			let proposal_id =
				NextProposalId::<T>::get().unwrap_or(T::ProposalId::initial_value().unwrap());

			<Proposals<T>>::insert(proposal_id, &new_proposal);

			let next_proposal_id = proposal_id.increment();
			NextProposalId::<T>::set(next_proposal_id);

			Self::deposit_event(Event::CreatedProposal(proposal_id));

			Ok(().into())
		}

		/// Update an existing proposal only from Proposal creator.
		///
		///
		/// The origin must be Signed and Proposal owner.
		///
		/// Parameters:
		/// - `name`: name/title of the proposal.
		/// - `description`: description of the proposal.
		///
		/// Emits `UpdatedProposal` event when successful.
		///
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn update_proposal(
			origin: OriginFor<T>,
			proposal_id: T::ProposalId,
			name: BoundedVec<u8, T::NameLimit>,
			description: BoundedVec<u8, T::DescriptionLimit>,
		) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			let proposer_account = ensure_signed(origin)?;

			Proposals::<T>::mutate(proposal_id, |optional_proposal| -> DispatchResult {
				let proposal =
					optional_proposal.as_mut().ok_or(Error::<T>::ProposalDoesNotExist)?;
				let current_block = <frame_system::Pallet<T>>::block_number();
				ensure!(proposal.proposer == proposer_account, Error::<T>::NotAllowed);
				ensure!(proposal.proposal_duration > current_block, Error::<T>::ProposalExpired);
				ensure!(
					proposal.status == ProposalResultStatus::Started,
					Error::<T>::ProposalExpired
				);

				proposal.name = name;
				proposal.description = description;

				Ok(())
			})?;

			Self::deposit_event(Event::UpdatedProposal(proposal_id));

			Ok(().into())
		}

		/// cast a vote for a proposal.
		///
		/// The origin must be Signed.
		///
		/// Parameters:
		/// - `proposal_id`: Id of the proposal.
		/// - `choice`: Id of the choice.
		///
		/// Emits `cast_vote` event when successful.
		///
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn cast_vote(
			origin: OriginFor<T>,
			proposal_id: T::ProposalId,
			choice: Choice,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let proposal =
				Proposals::<T>::get(proposal_id).ok_or(Error::<T>::ProposalDoesNotExist)?;
			let current_block = <frame_system::Pallet<T>>::block_number();
			ensure!(proposal.proposal_duration > current_block && proposal.status == ProposalResultStatus::Started, Error::<T>::ProposalExpired);

			Votes::<T>::mutate(proposal_id, |optional_votes| -> DispatchResult {
				if let Some(votes) = optional_votes {

					let is_already_voted = votes
						.into_iter()
						.find(|vote| vote.who == who).is_some();

					ensure!(!is_already_voted, Error::<T>::AlreadyVoted);
					let vote = Vote { who, choice };

					votes.push(vote);
					*optional_votes = Some(votes.to_vec());

				} else {
					let vote = Vote { who, choice };
					*optional_votes = Some(vec![vote]);
				}

				Ok(())
			})?;

			Self::deposit_event(Event::VoteCasted(proposal_id));
			Ok(().into())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn pass_proposal(
			origin: OriginFor<T>,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin)?;

			let current_block = <frame_system::Pallet<T>>::block_number();
			<Proposals<T>>::iter().filter(|(_, proposal)| (proposal.proposal_duration < current_block) && (proposal.status == ProposalResultStatus::Started)).for_each(
				|(proposal_id, _)| {
					let _ = Proposals::<T>::mutate(proposal_id, |optional_proposal| -> DispatchResult {
						let proposal =
							optional_proposal.as_mut().ok_or(Error::<T>::ProposalDoesNotExist)?;

						let votes: Vec<Vote<T::AccountId>> =
							Votes::<T>::get(proposal_id).ok_or(Error::<T>::VoteDoesNotExist)?;

						let voted_yes: Vec<Vote<T::AccountId>> = votes.clone().into_iter().filter(|vote| vote.choice == Choice::Yes).collect::<Vec<Vote<T::AccountId>>>();
						let voted_no: Vec<Vote<T::AccountId>> = votes.into_iter().filter(|vote| vote.choice == Choice::No).collect::<Vec<Vote<T::AccountId>>>();

						if voted_yes.len() > voted_no.len() {
							proposal.status = Accepted;
						} else {
							proposal.status = Rejected;
						}

						Ok(())
					});

				},
			);

			Ok(().into())
		}
	}
}
