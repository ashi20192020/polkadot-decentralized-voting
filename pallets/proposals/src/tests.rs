use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::ConstU32;
use sp_runtime::BoundedVec;
use crate::{Proposals, Choice, ProposalResultStatus, Votes};

fn test_pub() -> sp_core::sr25519::Public {
	sp_core::sr25519::Public::from_raw([1u8; 32])
}

#[test]
fn create_proposal_works() {
	new_test_ext().execute_with(|| {

		System::set_block_number(2);
		let name: Vec<u8> =
			"name".into();
		let bounded_name: BoundedVec<u8, ConstU32<50>> =
			name.try_into().unwrap();

		let description: Vec<u8> =
			"description".into();
		let bounded_description: BoundedVec<u8, ConstU32<250>> =
			description.try_into().unwrap();

		assert_ok!(ProposalPallet::create_proposal(RuntimeOrigin::signed(test_pub()), bounded_name.clone(), bounded_description.clone(), 20));

		assert_eq!(
			Proposals::<Test>::get(0).unwrap().description,
			bounded_description
		);

		assert_noop!(
			ProposalPallet::create_proposal(RuntimeOrigin::signed(test_pub()), bounded_name, bounded_description, 0),
			Error::<Test>::InvalidDuration
		);

	});
}

#[test]
fn update_proposal_works() {
	new_test_ext().execute_with(|| {

		let name: Vec<u8> =
			"name".into();
		let bounded_name: BoundedVec<u8, ConstU32<50>> =
			name.try_into().unwrap();

		let description: Vec<u8> =
			"description".into();
		let bounded_description: BoundedVec<u8, ConstU32<250>> =
			description.try_into().unwrap();

		assert_ok!(ProposalPallet::create_proposal(RuntimeOrigin::signed(test_pub()), bounded_name, bounded_description.clone(), 20));

		assert_eq!(
			Proposals::<Test>::get(0).unwrap().description,
			bounded_description
		);

		let name: Vec<u8> =
			"update name".into();
		let bounded_name: BoundedVec<u8, ConstU32<50>> =
			name.try_into().unwrap();

		let description: Vec<u8> =
			"update description".into();
		let updated_bounded_description: BoundedVec<u8, ConstU32<250>> =
			description.try_into().unwrap();

		assert_ok!(ProposalPallet::update_proposal(RuntimeOrigin::signed(test_pub()), 0, bounded_name.clone(), updated_bounded_description.clone()));

		assert_eq!(
			Proposals::<Test>::get(0).unwrap().description,
			updated_bounded_description
		);

		assert_noop!(
			ProposalPallet::update_proposal(RuntimeOrigin::signed(test_pub()), 5, bounded_name, updated_bounded_description),
			Error::<Test>::ProposalDoesNotExist
		);
	});
}

#[test]
fn cast_proposal_works() {
	new_test_ext().execute_with(|| {

		let name: Vec<u8> =
			"name".into();
		let bounded_name: BoundedVec<u8, ConstU32<50>> =
			name.try_into().unwrap();

		let description: Vec<u8> =
			"description".into();
		let bounded_description: BoundedVec<u8, ConstU32<250>> =
			description.try_into().unwrap();


		assert_ok!(ProposalPallet::create_proposal(RuntimeOrigin::signed(test_pub()), bounded_name, bounded_description.clone(), 20));

		assert_eq!(
			Proposals::<Test>::get(0).unwrap().description,
			bounded_description
		);

		assert_ok!(ProposalPallet::cast_vote(RuntimeOrigin::signed(test_pub()), 0, Choice::Yes));
		assert!(Votes::<Test>::contains_key(0));

		assert_noop!(
			ProposalPallet::cast_vote(RuntimeOrigin::signed(test_pub()), 0, Choice::No),
			Error::<Test>::AlreadyVoted
		);

		System::set_block_number(21);

		assert_noop!(
			ProposalPallet::cast_vote(RuntimeOrigin::signed(test_pub()), 2, Choice::Yes),
			Error::<Test>::ProposalDoesNotExist
		);

		assert_noop!(
			ProposalPallet::cast_vote(RuntimeOrigin::signed(test_pub()), 0, Choice::Yes),
			Error::<Test>::ProposalExpired
		);

		// Read pallet storage and assert an expected result.


	});
}

#[test]
fn pass_proposal_works() {
	new_test_ext().execute_with(|| {

		let name: Vec<u8> =
			"name".into();
		let bounded_name: BoundedVec<u8, ConstU32<50>> =
			name.try_into().unwrap();

		let description: Vec<u8> =
			"description".into();
		let bounded_description: BoundedVec<u8, ConstU32<250>> =
			description.try_into().unwrap();


		assert_ok!(ProposalPallet::create_proposal(RuntimeOrigin::signed(test_pub()), bounded_name, bounded_description.clone(), 20));

		assert_eq!(
			Proposals::<Test>::get(0).unwrap().description,
			bounded_description
		);

		assert_ok!(ProposalPallet::cast_vote(RuntimeOrigin::signed(test_pub()), 0, Choice::Yes));

		System::set_block_number(21);

		let _ = ProposalPallet::pass_proposal(RuntimeOrigin::signed(test_pub()));
		assert_eq!(
			Proposals::<Test>::get(0).unwrap().status,
			ProposalResultStatus::Accepted
		);

	});
}
