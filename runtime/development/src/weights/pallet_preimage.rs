
//! Autogenerated weights for `pallet_preimage`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner`, CPU: `AMD EPYC 7763 64-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("centrifuge-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=centrifuge-dev
// --steps=50
// --repeat=20
// --pallet=pallet_preimage
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_preimage.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_preimage`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_preimage::WeightInfo for WeightInfo<T> {
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: Preimage PreimageFor (r:0 w:1)
	/// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 4194304]`.
	fn note_preimage(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `209`
		//  Estimated: `2566`
		// Minimum execution time: 36_317 nanoseconds.
		Weight::from_parts(82_180_035, 2566)
			// Standard Error: 18
			.saturating_add(Weight::from_ref_time(2_389).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: Preimage PreimageFor (r:0 w:1)
	/// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 4194304]`.
	fn note_requested_preimage(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `140`
		//  Estimated: `2566`
		// Minimum execution time: 23_804 nanoseconds.
		Weight::from_parts(1_075_216, 2566)
			// Standard Error: 7
			.saturating_add(Weight::from_ref_time(2_539).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: Preimage PreimageFor (r:0 w:1)
	/// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 4194304]`.
	fn note_no_deposit_preimage(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `140`
		//  Estimated: `2566`
		// Minimum execution time: 22_502 nanoseconds.
		Weight::from_parts(22_822_000, 2566)
			// Standard Error: 4
			.saturating_add(Weight::from_ref_time(2_481).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: Preimage PreimageFor (r:0 w:1)
	/// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
	fn unnote_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `387`
		//  Estimated: `2566`
		// Minimum execution time: 53_900 nanoseconds.
		Weight::from_parts(57_466_000, 2566)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: Preimage PreimageFor (r:0 w:1)
	/// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
	fn unnote_no_deposit_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `178`
		//  Estimated: `2566`
		// Minimum execution time: 35_747 nanoseconds.
		Weight::from_parts(41_898_000, 2566)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn request_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `254`
		//  Estimated: `2566`
		// Minimum execution time: 33_192 nanoseconds.
		Weight::from_parts(35_406_000, 2566)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn request_no_deposit_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `178`
		//  Estimated: `2566`
		// Minimum execution time: 22_562 nanoseconds.
		Weight::from_parts(25_358_000, 2566)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn request_unnoted_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `2566`
		// Minimum execution time: 21_160 nanoseconds.
		Weight::from_parts(22_351_000, 2566)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn request_requested_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `140`
		//  Estimated: `2566`
		// Minimum execution time: 12_333 nanoseconds.
		Weight::from_parts(13_284_000, 2566)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: Preimage PreimageFor (r:0 w:1)
	/// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
	fn unrequest_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `178`
		//  Estimated: `2566`
		// Minimum execution time: 39_734 nanoseconds.
		Weight::from_parts(43_110_000, 2566)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn unrequest_unnoted_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `140`
		//  Estimated: `2566`
		// Minimum execution time: 12_663 nanoseconds.
		Weight::from_parts(14_076_000, 2566)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn unrequest_multi_referenced_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `140`
		//  Estimated: `2566`
		// Minimum execution time: 12_323 nanoseconds.
		Weight::from_parts(13_255_000, 2566)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}