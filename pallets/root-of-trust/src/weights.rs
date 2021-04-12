/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2020  Nodle International
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

//! Autogenerated weights for pallet_root_of_trust
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-04-04, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/nodle-chain
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_root_of_trust
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/root-of-trust/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_root_of_trust.
pub trait WeightInfo {
    fn book_slot() -> Weight;
    fn renew_slot() -> Weight;
    fn revoke_slot() -> Weight;
    fn revoke_child() -> Weight;
}

/// Weights for pallet_root_of_trust using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn book_slot() -> Weight {
        (77_054_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(9 as Weight))
            .saturating_add(T::DbWeight::get().writes(6 as Weight))
    }
    fn renew_slot() -> Weight {
        (82_574_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(9 as Weight))
            .saturating_add(T::DbWeight::get().writes(6 as Weight))
    }
    fn revoke_slot() -> Weight {
        (33_723_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(6 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    fn revoke_child() -> Weight {
        (34_745_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(6 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn book_slot() -> Weight {
        (77_054_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(9 as Weight))
            .saturating_add(RocksDbWeight::get().writes(6 as Weight))
    }
    fn renew_slot() -> Weight {
        (82_574_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(9 as Weight))
            .saturating_add(RocksDbWeight::get().writes(6 as Weight))
    }
    fn revoke_slot() -> Weight {
        (33_723_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(6 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
    fn revoke_child() -> Weight {
        (34_745_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(6 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
}