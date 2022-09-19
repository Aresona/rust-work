use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use super::*;

#[test]
fn create_kitty_works() {
    new_test_ext().execute_with(||{
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_eq!(KittiesModule::next_kitty_id(), 1);
    })
}

#[test]
fn breed_kitty_works() {
    new_test_ext().execute_with(||{
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::breed(Origin::signed(2), 0, 1));
    })
}

#[test]
fn transfer_kitty_works() {
    new_test_ext().execute_with(||{
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::transfer(Origin::signed(1), 0, 2));
    })
}

// create

#[test]
fn create_fail_due_to_invalid_id() {
    new_test_ext().execute_with(||{
		NextKittyId::<Test>::set(KittyIndex::max_value());
        assert_noop!(KittiesModule::create(Origin::signed(1)), Error::<Test>::InvalidKittyId);
	})
}


#[test]
fn create_fail_due_to_token_not_enough(){
    new_test_ext().execute_with(||{
        assert_noop!(KittiesModule::create(Origin::signed(3)), Error::<Test>::TokenNotEnough);
    })
}


// breed
#[test]
fn breed_fail_due_to_invalid_id() {
    new_test_ext().execute_with(||{
        assert_noop!(KittiesModule::breed(Origin::signed(1), 0, 1), Error::<Test>::InvalidKittyId);
	})
}

#[test]
fn breed_fail_due_to_token_not_enough(){
    new_test_ext().execute_with(||{
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_ok!(KittiesModule::create(Origin::signed(2)));
        assert_noop!(KittiesModule::breed(Origin::signed(1),0, 1), Error::<Test>::TokenNotEnough);
    })
}

// business error tests
#[test]
fn transfer_fail_due_to_not_owner() {
    new_test_ext().execute_with(||{
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_noop!(KittiesModule::transfer(Origin::signed(2), 0, 1), Error::<Test>::NotOwner);
    })
}

#[test]
fn transfer_fail_when_kitty_id_not_exist(){
    new_test_ext().execute_with(||{
		assert_noop!(KittiesModule::transfer(Origin::signed(2), 0, 1), Error::<Test>::InvalidKittyId);
    })
}
