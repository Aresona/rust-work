use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_err, assert_noop, assert_ok, BoundedVec};

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
        // 保证创建凭证不出错
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

		let bounded_claim =
			BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
  
        // 验证存储中的值与想象中一样
		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
	})
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim: Vec<u8> = vec![0, 1];
        // 创建凭证
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        // 再次创建时报错提示存证已经存在
		assert_noop!(
			PoeModule::create_claim(Origin::signed(2), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	})
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        // 确保取消存储正常执行
		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
        let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
        // 确保存储中清除干净
        assert!(!Proofs::<Test>::contains_key(&bounded_claim));
	})
}

#[test]
fn revoke_claim_failed_when_claim_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
        // 确保正常报错
		assert_err!(
			PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ClaimNotExist
		);
	})
}
#[test]
fn revoke_claim_failed_when_not_claim_owner() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
        // 为用户 1 创建存证
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        // 为用户 2 取消存证
		assert_err!(
			PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	})
}

#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        // 确保转移存证正常执行
        assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2));
        let bounded_claim =
			BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
        let (owner, _) = Proofs::<Test>::get(&bounded_claim).unwrap();
        // 确保转移存证后存证的所有者确实为转移目标
        assert_eq!(owner, 2);
    })
}

#[test]
fn transfer_claim_failed_when_claim_not_exist() {
	new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        // 当存储不存在时报错提示
		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2),
			Error::<Test>::ClaimNotExist
		);
    })
}

#[test]
fn transfer_claim_failed_when_not_claim_owner() {
	new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        // 当存储不属于 owner 时报错提示
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 3),
            Error::<Test>::NotClaimOwner
        );
    })
}
