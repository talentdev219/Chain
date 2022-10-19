/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2020-2022  Nodle International
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

//! Autogenerated weights for pallet_scheduler
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-19, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/nodle-parachain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./.maintain/frame-weight-template.hbs
// --output=weights

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_scheduler.
pub trait WeightInfo {
	fn on_initialize_periodic_named_resolved(s: u32) -> Weight;
	fn on_initialize_named_resolved(s: u32) -> Weight;
	fn on_initialize_periodic_resolved(s: u32) -> Weight;
	fn on_initialize_resolved(s: u32) -> Weight;
	fn on_initialize_named_aborted(s: u32) -> Weight;
	fn on_initialize_aborted(s: u32) -> Weight;
	fn on_initialize_periodic_named(s: u32) -> Weight;
	fn on_initialize_periodic(s: u32) -> Weight;
	fn on_initialize_named(s: u32) -> Weight;
	fn on_initialize(s: u32) -> Weight;
	fn schedule(s: u32) -> Weight;
	fn cancel(s: u32) -> Weight;
	fn schedule_named(s: u32) -> Weight;
	fn cancel_named(s: u32) -> Weight;
}

/// Weights for pallet_scheduler using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn on_initialize_periodic_named_resolved(s: u32) -> Weight {
		Weight::from_ref_time(25_766_000 as u64)
			// Standard Error: 23_000
			.saturating_add(Weight::from_ref_time(62_300_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((4 as u64).saturating_mul(s as u64)))
	}
	fn on_initialize_named_resolved(s: u32) -> Weight {
		Weight::from_ref_time(172_909_000 as u64)
			// Standard Error: 889_000
			.saturating_add(Weight::from_ref_time(40_730_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(s as u64)))
	}
	fn on_initialize_periodic_resolved(s: u32) -> Weight {
		Weight::from_ref_time(17_947_000 as u64)
			// Standard Error: 18_000
			.saturating_add(Weight::from_ref_time(23_753_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(s as u64)))
	}
	fn on_initialize_resolved(s: u32) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 992_000
			.saturating_add(Weight::from_ref_time(28_817_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(s as u64)))
	}
	fn on_initialize_named_aborted(s: u32) -> Weight {
		Weight::from_ref_time(11_837_000 as u64)
			// Standard Error: 6_000
			.saturating_add(Weight::from_ref_time(8_383_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	fn on_initialize_aborted(s: u32) -> Weight {
		Weight::from_ref_time(19_165_000 as u64)
			// Standard Error: 166_000
			.saturating_add(Weight::from_ref_time(7_497_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	fn on_initialize_periodic_named(s: u32) -> Weight {
		Weight::from_ref_time(25_271_000 as u64)
			// Standard Error: 17_000
			.saturating_add(Weight::from_ref_time(31_745_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(s as u64)))
	}
	fn on_initialize_periodic(s: u32) -> Weight {
		Weight::from_ref_time(30_827_000 as u64)
			// Standard Error: 11_000
			.saturating_add(Weight::from_ref_time(20_581_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	fn on_initialize_named(s: u32) -> Weight {
		Weight::from_ref_time(29_921_000 as u64)
			// Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(19_412_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	fn on_initialize(s: u32) -> Weight {
		Weight::from_ref_time(29_418_000 as u64)
			// Standard Error: 6_000
			.saturating_add(Weight::from_ref_time(15_079_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	fn schedule(s: u32) -> Weight {
		Weight::from_ref_time(46_815_000 as u64)
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(127_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	fn cancel(s: u32) -> Weight {
		Weight::from_ref_time(45_517_000 as u64)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(1_225_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	fn schedule_named(s: u32) -> Weight {
		Weight::from_ref_time(58_878_000 as u64)
			// Standard Error: 6_000
			.saturating_add(Weight::from_ref_time(236_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	fn cancel_named(s: u32) -> Weight {
		Weight::from_ref_time(53_088_000 as u64)
			// Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(1_358_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn on_initialize_periodic_named_resolved(s: u32) -> Weight {
		Weight::from_ref_time(25_766_000 as u64)
			// Standard Error: 23_000
			.saturating_add(Weight::from_ref_time(62_300_000).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().reads((3 as u64).saturating_mul(s as u64)))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
			.saturating_add(RocksDbWeight::get().writes((4 as u64).saturating_mul(s as u64)))
	}
	fn on_initialize_named_resolved(s: u32) -> Weight {
		Weight::from_ref_time(172_909_000 as u64)
			// Standard Error: 889_000
			.saturating_add(Weight::from_ref_time(40_730_000).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().reads((2 as u64).saturating_mul(s as u64)))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
			.saturating_add(RocksDbWeight::get().writes((3 as u64).saturating_mul(s as u64)))
	}
	fn on_initialize_periodic_resolved(s: u32) -> Weight {
		Weight::from_ref_time(17_947_000 as u64)
			// Standard Error: 18_000
			.saturating_add(Weight::from_ref_time(23_753_000).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().reads((3 as u64).saturating_mul(s as u64)))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
			.saturating_add(RocksDbWeight::get().writes((3 as u64).saturating_mul(s as u64)))
	}
	fn on_initialize_resolved(s: u32) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 992_000
			.saturating_add(Weight::from_ref_time(28_817_000).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().reads((2 as u64).saturating_mul(s as u64)))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
			.saturating_add(RocksDbWeight::get().writes((2 as u64).saturating_mul(s as u64)))
	}
	fn on_initialize_named_aborted(s: u32) -> Weight {
		Weight::from_ref_time(11_837_000 as u64)
			// Standard Error: 6_000
			.saturating_add(Weight::from_ref_time(8_383_000).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	fn on_initialize_aborted(s: u32) -> Weight {
		Weight::from_ref_time(19_165_000 as u64)
			// Standard Error: 166_000
			.saturating_add(Weight::from_ref_time(7_497_000).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	fn on_initialize_periodic_named(s: u32) -> Weight {
		Weight::from_ref_time(25_271_000 as u64)
			// Standard Error: 17_000
			.saturating_add(Weight::from_ref_time(31_745_000).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
			.saturating_add(RocksDbWeight::get().writes((2 as u64).saturating_mul(s as u64)))
	}
	fn on_initialize_periodic(s: u32) -> Weight {
		Weight::from_ref_time(30_827_000 as u64)
			// Standard Error: 11_000
			.saturating_add(Weight::from_ref_time(20_581_000).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	fn on_initialize_named(s: u32) -> Weight {
		Weight::from_ref_time(29_921_000 as u64)
			// Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(19_412_000).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	fn on_initialize(s: u32) -> Weight {
		Weight::from_ref_time(29_418_000 as u64)
			// Standard Error: 6_000
			.saturating_add(Weight::from_ref_time(15_079_000).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	fn schedule(s: u32) -> Weight {
		Weight::from_ref_time(46_815_000 as u64)
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(127_000).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	fn cancel(s: u32) -> Weight {
		Weight::from_ref_time(45_517_000 as u64)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(1_225_000).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	fn schedule_named(s: u32) -> Weight {
		Weight::from_ref_time(58_878_000 as u64)
			// Standard Error: 6_000
			.saturating_add(Weight::from_ref_time(236_000).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	fn cancel_named(s: u32) -> Weight {
		Weight::from_ref_time(53_088_000 as u64)
			// Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(1_358_000).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
}