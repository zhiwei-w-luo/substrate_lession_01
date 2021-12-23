use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

use super::*;

#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((1, frame_system::Pallet::<Test>::block_number())
            ));
    })
}


#[test]
fn create_claim_failed_when_claim_alreay_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
			PoeModule::create_claim(Origin::signed(1),claim.clone()),
			Error::<Test>::ClaimAlreadyExist
		);
    })
}


#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim), None);
    })
}


#[test]
fn revoke_claim_failed_when_claim_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_noop!(
			PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ClaimNotExist
		);
    })
}


#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((1, frame_system::Pallet::<Test>::block_number())
            ));

        let _ = PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2);

        let val = Proofs::<Test>::get(&claim);
        assert_eq!(
            val,
            Some((2,0))
        );
    })
}

#[test]
fn transfer_claim_failed_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let another = 1234;
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_noop!(
			PoeModule::transfer_claim(Origin::signed(2), claim.clone(),another),
			Error::<Test>::NotClaimOwner
		);
    })
}

#[test]
fn transfer_claim_failed_when_claim_is_not_exist(){
    new_test_ext().execute_with(|| {
        let claim = vec![0,1];
        let another_acount = 1234;
        assert_noop!(
			PoeModule::transfer_claim(Origin::signed(1), claim.clone(),another_acount),
			Error::<Test>::ClaimNotExist
		);

    })
}

#[test]
fn vec_size_limit_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1,2,3,4,5,6,7,8,9];
        let _= PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((1, frame_system::Pallet::<Test>::block_number())
            ));

    })
}

#[test]
fn vec_size_failed_limit_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1,2,3,4,5,6,7,8,9,10];

        assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ClaimOverSize
		);

    })
}

