// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Test environment for node-authorization pallet.

use super::*;
use crate as pallet_node_authorization;

use frame::testing_prelude::*;

type Block = frame_system::mocking::MockBlock<Test>;

construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		NodeAuthorization: pallet_node_authorization,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
}

ord_parameter_types! {
	pub const One: u64 = 1;
	pub const Two: u64 = 2;
	pub const Three: u64 = 3;
	pub const Four: u64 = 4;
}

impl Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MaxWellKnownNodes = ConstU32<4>;
	type MaxPeerIdLength = ConstU32<2>;
	type AddOrigin = EnsureSignedBy<One, u64>;
	type RemoveOrigin = EnsureSignedBy<Two, u64>;
	type SwapOrigin = EnsureSignedBy<Three, u64>;
	type ResetOrigin = EnsureSignedBy<Four, u64>;
	type WeightInfo = ();
}

pub fn test_node(id: u8) -> PeerId {
	PeerId(vec![id])
}

pub fn new_test_ext() -> TestState {
	let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	pallet_node_authorization::GenesisConfig::<Test> {
		nodes: vec![(test_node(10), 10), (test_node(20), 20), (test_node(30), 30)],
	}
	.assimilate_storage(&mut t)
	.unwrap();
	t.into()
}
