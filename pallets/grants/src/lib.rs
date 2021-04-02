#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, ensure,
    traits::{
        Currency, EnsureOrigin, ExistenceRequirement, Get, LockIdentifier, LockableCurrency,
        WithdrawReasons,
    },
};
use frame_system::{ensure_root, ensure_signed};
use parity_scale_codec::{Decode, Encode};
use sp_runtime::{
    traits::{AtLeast32Bit, CheckedAdd, Saturating, StaticLookup, Zero},
    DispatchResult, RuntimeDebug,
};
use sp_std::{
    cmp::{Eq, PartialEq},
    vec::Vec,
};

mod benchmarking;
mod mock;
mod tests;

/// The vesting schedule.
///
/// Benefits would be granted gradually, `per_period` amount every `period` of blocks
/// after `start`.
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct VestingSchedule<BlockNumber, Balance> {
    pub start: BlockNumber,
    pub period: BlockNumber,
    pub period_count: u32,
    pub per_period: Balance,
}

impl<BlockNumber: AtLeast32Bit + Copy, Balance: AtLeast32Bit + Copy>
    VestingSchedule<BlockNumber, Balance>
{
    /// Returns the end of all periods, `None` if calculation overflows.
    pub fn end(&self) -> Option<BlockNumber> {
        self.period
            .checked_mul(&self.period_count.into())?
            .checked_add(&self.start)
    }

    /// Returns all locked amount, `None` if calculation overflows.
    pub fn total_amount(&self) -> Option<Balance> {
        self.per_period.checked_mul(&self.period_count.into())
    }

    /// Returns locked amount for a given `time`.
    ///
    /// Note this func assumes schedule is a valid one(non-zero period and non-overflow total amount),
    /// and it should be guaranteed by callers.
    pub fn locked_amount(&self, time: BlockNumber) -> Balance {
        let full = time
            .saturating_sub(self.start)
            .checked_div(&self.period)
            .expect("ensured non-zero period; qed");
        let unrealized = self
            .period_count
            .saturating_sub(full.unique_saturated_into());
        self.per_period
            .checked_mul(&unrealized.into())
            .expect("ensured non-overflow total amount; qed")
    }
}

pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
pub type VestingScheduleOf<T> =
    VestingSchedule<<T as frame_system::Config>::BlockNumber, BalanceOf<T>>;
pub type ScheduledGrant<T> = (
    <T as frame_system::Config>::BlockNumber,
    <T as frame_system::Config>::BlockNumber,
    u32,
    BalanceOf<T>,
);
pub type ScheduledItem<T> = (
    <T as frame_system::Config>::AccountId,
    Vec<ScheduledGrant<T>>,
);

pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
    type CancelOrigin: EnsureOrigin<Self::Origin>;
}

decl_storage! {
    trait Store for Module<T: Config> as Vesting {
        /// Vesting schedules of an account.
        pub VestingSchedules get(fn vesting_schedules): map hasher(blake2_128_concat) T::AccountId => Vec<VestingScheduleOf<T>>;
    }

    add_extra_genesis {
        config(vesting): Vec<ScheduledItem<T>>;
        build(|config: &GenesisConfig<T>| {
            let grants = config.vesting.iter()
                .map(|(ref who, schedules)|
                    (
                        who.clone(),
                        schedules.iter()
                            .map(|&(start, period, period_count, per_period)| VestingSchedule {
                                start, period, period_count, per_period
                            })
                            .collect::<Vec<_>>()
                    )
                )
                .collect::<Vec<_>>();

            // Create the required coins at genesis and add to storage
            grants.iter()
                .for_each(|(ref who, schedules)| {
                    let total_grants = schedules.iter()
                        .fold(Zero::zero(), |acc: BalanceOf<T>, s| acc.saturating_add(s.locked_amount(Zero::zero())));

                    T::Currency::resolve_creating(who, T::Currency::issue(total_grants));
                    T::Currency::set_lock(VESTING_LOCK_ID, who, total_grants, WithdrawReasons::all());
                    <VestingSchedules<T>>::insert(who, schedules);
                });
        });
    }
}

decl_event!(
    pub enum Event<T> where
        <T as frame_system::Config>::AccountId,
        Balance = BalanceOf<T>,
        VestingSchedule = VestingScheduleOf<T>
    {
        /// Added new vesting schedule (from, to, vesting_schedule)
        VestingScheduleAdded(AccountId, AccountId, VestingSchedule),
        /// Claimed vesting (who, locked_amount)
        Claimed(AccountId, Balance),
        /// Canceled all vesting schedules (who)
        VestingSchedulesCanceled(AccountId),
    }
);

decl_error! {
    /// Error for vesting module.
    pub enum Error for Module<T: Config> {
        ZeroVestingPeriod,
        ZeroVestingPeriodCount,
        NumOverflow,
        InsufficientBalanceToLock,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        /// Claim funds that have been vested so far
        #[weight = 30_000_000 + T::DbWeight::get().reads_writes(2, 3)]
        pub fn claim(origin) {
            let who = ensure_signed(origin)?;
            let locked_amount = Self::do_claim(&who);

            if locked_amount.is_zero() {
                // No more claimable, clear
                VestingSchedules::<T>::remove(who.clone());
            }

            Self::deposit_event(RawEvent::Claimed(who, locked_amount));
        }

        /// Wire funds to be vested by the receiver
        #[weight = 48_000_000 + T::DbWeight::get().reads_writes(4, 4)]
        pub fn add_vesting_schedule(
            origin,
            dest: <T::Lookup as StaticLookup>::Source,
            schedule: VestingScheduleOf<T>,
        ) {
            let from = ensure_signed(origin)?;
            let to = T::Lookup::lookup(dest)?;
            Self::do_add_vesting_schedule(&from, &to, schedule.clone())?;

            Self::deposit_event(RawEvent::VestingScheduleAdded(from, to, schedule));
        }

        /// Cancel all vested schedules for the given user. If there are coins to be
        /// claimed they will be auto claimed for the given user. If `limit_to_free_balance`
        /// is set to true we will not error if the free balance of `who` has less coins
        /// than what was granted and is being revoked (useful if the state was corrupted).
        #[weight = 48_000_000 + T::DbWeight::get().reads_writes(5, 5)]
        pub fn cancel_all_vesting_schedules(
            origin,
            who: <T::Lookup as StaticLookup>::Source,
            funds_collector: <T::Lookup as StaticLookup>::Source,
            limit_to_free_balance: bool,
        ) {
            T::CancelOrigin::try_origin(origin)
                .map(|_| ())
                .or_else(ensure_root)?;

            let account_with_schedule = T::Lookup::lookup(who)?;
            let account_collector = T::Lookup::lookup(funds_collector)?;

            let mut locked_amount_left = Self::do_claim(&account_with_schedule);
            let free_balance = T::Currency::free_balance(&account_with_schedule);
            if limit_to_free_balance && free_balance < locked_amount_left {
                locked_amount_left = free_balance;
            }

            // we need to remove the lock before doing the transfer to avoid
            // liquidity restrictions
            T::Currency::remove_lock(VESTING_LOCK_ID, &account_with_schedule);
            T::Currency::transfer(
                &account_with_schedule,
                &account_collector,
                locked_amount_left,
                ExistenceRequirement::AllowDeath
            )?;
            VestingSchedules::<T>::remove(account_with_schedule.clone());

            Self::deposit_event(RawEvent::VestingSchedulesCanceled(account_with_schedule));
        }
    }
}

pub const VESTING_LOCK_ID: LockIdentifier = *b"nvesting";

impl<T: Config> Module<T> {
    fn do_claim(who: &T::AccountId) -> BalanceOf<T> {
        let locked = Self::locked_balance(who);
        if locked.is_zero() {
            T::Currency::remove_lock(VESTING_LOCK_ID, who);
        } else {
            T::Currency::set_lock(VESTING_LOCK_ID, who, locked, WithdrawReasons::all());
        }
        locked
    }

    /// Returns locked balance based on current block number.
    fn locked_balance(who: &T::AccountId) -> BalanceOf<T> {
        let now = <frame_system::Module<T>>::block_number();
        Self::vesting_schedules(who)
            .iter()
            .fold(Zero::zero(), |acc, s| {
                acc.checked_add(&s.locked_amount(now)).expect(
                    "locked amount is a balance and can't be higher than the total balance stored inside the same integer type; qed",
                )
            })
    }

    fn do_add_vesting_schedule(
        from: &T::AccountId,
        to: &T::AccountId,
        schedule: VestingScheduleOf<T>,
    ) -> DispatchResult {
        let schedule_amount = Self::ensure_valid_vesting_schedule(&schedule)?;
        let total_amount = Self::locked_balance(to)
            .checked_add(&schedule_amount)
            .ok_or(Error::<T>::NumOverflow)?;

        T::Currency::transfer(from, to, schedule_amount, ExistenceRequirement::AllowDeath)?;
        T::Currency::set_lock(VESTING_LOCK_ID, to, total_amount, WithdrawReasons::all());
        <VestingSchedules<T>>::mutate(to, |v| (*v).push(schedule));

        Ok(())
    }

    /// Returns `Ok(amount)` if valid schedule, or error.
    fn ensure_valid_vesting_schedule(
        schedule: &VestingScheduleOf<T>,
    ) -> Result<BalanceOf<T>, Error<T>> {
        ensure!(!schedule.period.is_zero(), Error::<T>::ZeroVestingPeriod);
        ensure!(
            !schedule.period_count.is_zero(),
            Error::<T>::ZeroVestingPeriodCount
        );
        ensure!(schedule.end().is_some(), Error::<T>::NumOverflow);

        schedule.total_amount().ok_or(Error::<T>::NumOverflow)
    }
}
