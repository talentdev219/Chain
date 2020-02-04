//! A module that is called by the `collective` and is in charge of holding
//! the company funds.

use frame_support::{
    decl_event, decl_module,
    dispatch::DispatchResult,
    traits::{Currency, ExistenceRequirement, Get},
};
use sp_runtime::traits::EnsureOrigin;

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

/// The module's configuration trait.
pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    type ExternalOrigin: EnsureOrigin<Self::Origin>;
    type Currency: Currency<Self::AccountId>;

    /// AccountId holding the reserve's funds
    type FundAccount: Get<Self::AccountId>;
}

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        pub fn spend(origin, to: T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
            T::ExternalOrigin::ensure_origin(origin)?;

            // TODO: we currently `AllowDeath` for our source account, shall we use `KeepAlive` instead?
            let _ = T::Currency::transfer(&T::FundAccount::get(), &to, amount, ExistenceRequirement::AllowDeath);

            Self::deposit_event(RawEvent::SpentFunds(to, amount));

            Ok(())
        }
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
        Balance = BalanceOf<T>,
    {
        SpentFunds(AccountId, Balance),
    }
);

/// tests for this module
#[cfg(test)]
mod tests {
    use super::*;

    use frame_support::{
        assert_noop, assert_ok, impl_outer_origin, ord_parameter_types, parameter_types,
        traits::Imbalance, weights::Weight,
    };
    use sp_core::H256;
    use sp_runtime::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup},
        DispatchError::BadOrigin,
        Perbill,
    };
    use system::EnsureSignedBy;

    impl_outer_origin! {
        pub enum Origin for Test {}
    }

    // For testing the module, we construct most of a mock runtime. This means
    // first constructing a configuration type (`Test`) which `impl`s each of the
    // configuration traits of modules we want to use.
    #[derive(Clone, Eq, PartialEq)]
    pub struct Test;
    parameter_types! {
        pub const BlockHashCount: u64 = 250;
        pub const MaximumBlockWeight: Weight = 1024;
        pub const MaximumBlockLength: u32 = 2 * 1024;
        pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
    }
    impl system::Trait for Test {
        type Origin = Origin;
        type Call = ();
        type Index = u64;
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type Event = ();
        type BlockHashCount = BlockHashCount;
        type MaximumBlockWeight = MaximumBlockWeight;
        type MaximumBlockLength = MaximumBlockLength;
        type AvailableBlockRatio = AvailableBlockRatio;
        type Version = ();
        type ModuleToIndex = ();
    }
    impl balances::Trait for Test {
        type Balance = u64;
        type OnNewAccount = ();
        type OnReapAccount = ();
        type Event = ();
        type TransferPayment = ();
        type DustRemoval = ();
        type ExistentialDeposit = ();
        type CreationFee = ();
    }

    ord_parameter_types! {
        pub const Admin: u64 = 1;
        pub const FundAccount: u64 = 2;
    }
    impl Trait for Test {
        type Event = ();
        type Currency = balances::Module<Self>;
        type ExternalOrigin = EnsureSignedBy<Admin, u64>;
        type FundAccount = FundAccount;
    }
    type TestModule = Module<Test>;
    type Balances = balances::Module<Test>;

    type PositiveImbalanceOf<T> =
        <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::PositiveImbalance;

    // This function basically just builds a genesis storage key/value store according to
    // our desired mockup.
    fn new_test_ext() -> sp_io::TestExternalities {
        system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap()
            .into()
    }

    #[test]
    fn spend_error_if_bad_origin() {
        new_test_ext().execute_with(|| {
            assert_noop!(TestModule::spend(Origin::signed(0), 1, 1), BadOrigin);
        })
    }

    #[test]
    fn spend_funds_to_target() {
        new_test_ext().execute_with(|| {
            let mut total_imbalance = <PositiveImbalanceOf<Test>>::zero();
            let r = <Test as Trait>::Currency::deposit_creating(&FundAccount::get(), 100);
            total_imbalance.subsume(r);

            assert_eq!(Balances::free_balance(FundAccount::get()), 100);
            assert_eq!(Balances::free_balance(3), 0);
            assert_ok!(TestModule::spend(Origin::signed(Admin::get()), 3, 100));
            assert_eq!(Balances::free_balance(3), 100);
            assert_eq!(Balances::free_balance(FundAccount::get()), 0);
        })
    }
}
