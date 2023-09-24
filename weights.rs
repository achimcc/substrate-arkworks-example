
//! Autogenerated weights for `pallet_ark_demo`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-24, STEPS: `3`, REPEAT: `3`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `behemoth`, CPU: `AMD Ryzen Threadripper 3970X 32-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// ./target/release/node-ark-demo
// benchmark
// pallet
// --no-min-squares
// --chain
// dev
// --pallet
// pallet_ark_demo
// --extrinsic
// bls12_377_pairing_opt
// --steps
// 3
// --repeat
// 3
// --output
// weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_ark_demo`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_ark_demo::WeightInfo for WeightInfo<T> {
	fn bls12_377_pairing_opt() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_482_267_000 picoseconds.
		Weight::from_parts(3_482_919_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
}
