//! Did pallet benchmarking.

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::{benchmarks, whitelist_account};
use frame_system::RawOrigin;
use sp_core::sr25519;
use sp_runtime::traits::Bounded;

use crate::Module as Did;

// const SEED: u32 = 0;
// existential deposit multiplier
// const ED_MULTIPLIER: u32 = 10;

benchmarks! {
    register {
        // referrer setting
        let public: T::Public = sr25519::Public([1; 32]).into();
        let caller: T::AccountId = public.clone().into_account();
        T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
        whitelist_account!(caller);
        Did::<T>::register(RawOrigin::Signed(caller.clone()).into(), public, None)?;
        let ref_id = Did::<T>::did_of(caller).unwrap();

        // caller setting
        let public: T::Public = sr25519::Public([2; 32]).into();
        let caller: T::AccountId = public.clone().into_account();
        T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
        whitelist_account!(caller);
    }: register(RawOrigin::Signed(caller.clone()), public, Some(ref_id))
    verify {
        assert_eq!(Did::<T>::total_dids(), Some(2), "should create did");
    }

    register_for {
        use frame_support::traits::Get;

        let min_deposit = <T as pallet::Config>::Deposit::get();
        // referrer setting
        let public: T::Public = sr25519::Public([1; 32]).into();
        let caller: T::AccountId = public.clone().into_account();
        T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
        whitelist_account!(caller);
        Did::<T>::register(RawOrigin::Signed(caller.clone()).into(), public, None)?;
        Did::<T>::lock(RawOrigin::Signed(caller.clone()).into(), min_deposit)?;

        let public: T::Public = sr25519::Public([2; 32]).into();
    }: register_for(RawOrigin::Signed(caller.clone()), public)
    verify {
        assert_eq!(Did::<T>::total_dids(), Some(2), "should create did");
    }

    lock {
        use frame_support::traits::Get;

        let min_deposit = <T as pallet::Config>::Deposit::get();
        // referrer setting
        let public: T::Public = sr25519::Public([1; 32]).into();
        let caller: T::AccountId = public.clone().into_account();
        T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
        whitelist_account!(caller);
        Did::<T>::register(RawOrigin::Signed(caller.clone()).into(), public, None)?;
    }: lock(RawOrigin::Signed(caller.clone()), min_deposit)
    verify {
        assert_eq!(Did::<T>::total_dids(), Some(1), "should create did");
    }

    revoke {
        use frame_support::traits::Get;

        let min_deposit = <T as pallet::Config>::Deposit::get();
        // referrer setting
        let public: T::Public = sr25519::Public([1; 32]).into();
        let caller: T::AccountId = public.clone().into_account();
        T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
        whitelist_account!(caller);
        Did::<T>::register(RawOrigin::Signed(caller.clone()).into(), public, None)?;
        Did::<T>::lock(RawOrigin::Signed(caller.clone()).into(), min_deposit)?;
    }: revoke(RawOrigin::Signed(caller.clone()))
    verify {
        assert_eq!(Did::<T>::total_dids(), Some(0), "should revoke did");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{new_test_ext, Test};
    use frame_support::assert_ok;

    #[test]
    fn test_benchmarks() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_register::<Test>());
            assert_ok!(test_benchmark_register_for::<Test>());
            assert_ok!(test_benchmark_lock::<Test>());
            assert_ok!(test_benchmark_revoke::<Test>());
        });
    }
}
