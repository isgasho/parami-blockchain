#![cfg(test)]

use super::*;
use crate as parami_did;

use frame_support::{assert_ok, ord_parameter_types, parameter_types};
use sp_core::{sr25519, H256};
use sp_runtime::{testing::Header, traits::BlakeTwo256};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Did: parami_did::{Pallet, Call, Storage, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(1024);
}
impl frame_system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Call = Call;
    type Hashing = BlakeTwo256;
    type AccountId = sr25519::Public;
    type Lookup = Did;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
}
parameter_types! {
    pub const ExistentialDeposit: u64 = 1;
}
impl pallet_balances::Config for Test {
    type MaxLocks = ();
    type Balance = u64;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
}
ord_parameter_types! {
    pub const One: u64 = 1;
    pub const DidDeposit: u64 = 1;

}

parameter_types! {
    pub const MinimumPeriod: u64 = 1;
}
impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

impl Config for Test {
    type Event = Event;
    type Currency = Balances;
    type Deposit = DidDeposit;
    type Signature = sr25519::Signature;
    type Public = <sr25519::Signature as Verify>::Signer;
    type WeightInfo = ();
    type Call = Call;
    type Time = Timestamp;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (sr25519::Public([1; 32]), 10),
            (sr25519::Public([2; 32]), 10),
        ],
    }
    .assimilate_storage(&mut t)
    .unwrap();
    t.into()
}

#[test]
fn register_did_should_work() {
    new_test_ext().execute_with(|| {
        let acct1 = sr25519::Public([1; 32]);
        assert_ok!(Did::register(
            Origin::signed(sr25519::Public([1; 32])),
            sr25519::Public([1; 32]),
            None
        ));
        assert_eq!(<TotalDids<Test>>::get(), Some(1));
        assert_eq!(Balances::total_balance(&acct1), 10);
        //assert_eq!(Balances::total_balance(&2), 8);
        // should have a did
        let maybe_did = <DidOf<Test>>::get(acct1);
        assert!(maybe_did.is_some());
        // should have metadata
        let maybe_metadata = <Metadata<Test>>::get(maybe_did.unwrap());
        assert!(maybe_metadata.is_some());
        // not revoked
        assert!(!maybe_metadata.unwrap().3);

        // referrer should work
        let did1 = maybe_did.unwrap();
        assert_ok!(Did::register(
            Origin::signed(sr25519::Public([2; 32])),
            sr25519::Public([2; 32]),
            Some(did1)
        ));
        assert_eq!(<TotalDids<Test>>::get(), Some(2));

        // register for on-ex account on chain
        // 0.you cannot register before deposit
        assert!(Did::register_for(
            Origin::signed(sr25519::Public([1; 32])),
            sr25519::Public([3; 32]),
        )
        .is_err());
        // 1.first, lock amount
        assert_ok!(Did::lock(Origin::signed(sr25519::Public([1; 32])), 5));
        // 2.then, register
        assert_ok!(Did::register_for(
            Origin::signed(sr25519::Public([1; 32])),
            sr25519::Public([3; 32]),
        ));
    });
}

#[test]
fn refuse_wrong_public() {
    new_test_ext().execute_with(|| {
        assert!(Did::register(
            Origin::signed(sr25519::Public([2; 32])),
            sr25519::Public([1; 32]),
            None
        )
        .is_err());
    });
}

#[test]
fn refuse_nonex_referrer() {
    new_test_ext().execute_with(|| {
        assert!(Did::register(
            Origin::signed(sr25519::Public([1; 32])),
            sr25519::Public([1; 32]),
            Some([0xee; 20])
        )
        .is_err());
    });
}

#[test]
fn refuse_multiple_registrations() {
    new_test_ext().execute_with(|| {
        assert_ok!(Did::register(
            Origin::signed(sr25519::Public([1; 32])),
            sr25519::Public([1; 32]),
            None
        ));

        assert!(Did::register(
            Origin::signed(sr25519::Public([1; 32])),
            sr25519::Public([1; 32]),
            None
        )
        .is_err());
    });
}
