use crate::*;
use frame_benchmarking::{benchmarks, whitelisted_caller, account};
use frame_system::RawOrigin;

benchmarks! {
	create_claim{
		let d in 0 .. T::MaxClaimLength::get();
		let claim = vec![0; d as usize];
		// 通过 whitelisted_caller 生成的 caller 账号可以直接读写数据库
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), claim)

	revoke_claim{
		let d in 0 .. T::MaxClaimLength::get();
		let claim = vec![0; d as usize];
        let caller: T::AccountId = whitelisted_caller();
		let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone()).unwrap();
		Proofs::<T>::insert(
				&bounded_claim,
				(caller.clone(), frame_system::Pallet::<T>::block_number()),
			);
	}: _(RawOrigin::Signed(caller.clone()), claim.clone())

	transfer_claim{
		let d in 0 .. T::MaxClaimLength::get();
		let claim = vec![0; d as usize];
        let caller: T::AccountId = whitelisted_caller();
		let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone()).unwrap();
		Proofs::<T>::insert(
				&bounded_claim,
				(caller.clone(), frame_system::Pallet::<T>::block_number()),
			);
		let dest: T::AccountId = account("dest", 0, 0);
	}: _(RawOrigin::Signed(caller), claim, dest)

	impl_benchmark_test_suite!(PoeModule, crate::mock::new_test_ext(), crate::mock::Test);

}
