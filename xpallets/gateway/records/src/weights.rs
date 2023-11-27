// Copyright 2023 BEVM Project Authors. Licensed under GPL-3.0.

//! Weights for xpallet_gateway_records
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-05-13, STEPS: 50, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("benchmarks"), DB CACHE: 1024

// Executed Command:
// ./target/release/chainx
// benchmark
// --chain=benchmarks
// --steps=50
// --repeat=20
// --pallet=xpallet_gateway_records
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./xpallets/gateway/records/src/weights.rs
// --template=./scripts/xpallet-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for xpallet_gateway_records.
pub trait WeightInfo {
	fn root_deposit() -> Weight;
	fn root_withdraw() -> Weight;
	fn set_withdrawal_state() -> Weight;
	fn set_withdrawal_state_list(u: u32) -> Weight;
}

/// Weights for xpallet_gateway_records using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn root_deposit() -> Weight {
		todo!()
	}
	fn root_withdraw() -> Weight {
		todo!()
	}
	fn set_withdrawal_state() -> Weight {
		todo!()
	}
	fn set_withdrawal_state_list(_u: u32) -> Weight {
		todo!()
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn root_deposit() -> Weight {
		todo!()
	}
	fn root_withdraw() -> Weight {
		todo!()
	}
	fn set_withdrawal_state() -> Weight {
		todo!()
	}
	fn set_withdrawal_state_list(_u: u32) -> Weight {
		todo!()
	}
}
