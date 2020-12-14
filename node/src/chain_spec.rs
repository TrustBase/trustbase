// Copyright 2019 TrustBase Network
// This file is part of TrustBase library.
//
// The TrustBase library is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// The TrustBase library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with the TrustBase library. If not, see <http://www.gnu.org/licenses/>.
use sp_core::{Pair, Public, sr25519};
use trust_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, SystemConfig, WASM_BINARY, Signature,
	ContractsConfig,
};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{Verify, IdentifyAccount};
use sc_service::ChainType;
use hex_literal::hex;

// Note this is the URL for the telemetry server
//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Trust network config
pub fn trust_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../../res/trust_cc1.json")[..])
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate an authority key for Aura
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(
		get_from_seed::<AuraId>(s),
		get_from_seed::<GrandpaId>(s),
	)
}


pub fn testnet_authorities() -> Vec<(AuraId, GrandpaId)> {
	use sp_core::crypto::UncheckedInto;
	vec![
		(
			hex!("1c83e0e631105f17c8985f0fa6f8d544a4f58d446574f2bc9873f3ddca0b3c51").unchecked_into(),
			hex!("fb67b3659cd68bde1aaa13b3c993474985a7892c040cf6d2f73208760a9a51db").unchecked_into(),
		),
		(
			hex!("fcba156e9d068b60570c37604f2f49bc01d985a03a6b64d64ae06db3071f4474").unchecked_into(),
			hex!("3b9682cdbda6233a9fc38ed3ca3783300a5e225003f728b29283ffc0bdf4bc29").unchecked_into(),
		),
		(
			hex!("7ae6fceddf6f670ec1f42113d4f0d6210010ca26cd7571db64c7857098e1cf7c").unchecked_into(),
			hex!("f6640d0021de921083054c0a9536345de5501ebfb23327732b74d37539c20f7f").unchecked_into(),
		),
		(
			hex!("e60d952e690e72aea36117eeca6f346bdaef59b627cc5c26491cb72194bd5a1a").unchecked_into(),
			hex!("6b54329f016ac966ac99c83cce6e48a4d088a6c3e9951f243c3e65596c8dfab8").unchecked_into(),
		),
	]
}

pub fn testnet_root() -> AccountId {
	// 5DFMx3uqVu1gaQ4JFb3c9ArS28EggtxUugxLKpYkHFbN8sEc
	hex!("345ceb562c0dec7a2fa2231aa21bdab6bd3712a587a203550a9bbdb495fa5969").into()
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		move || testnet_genesis(
			wasm_binary,
			vec![
				authority_keys_from_seed("Alice"),
			],
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			],
			true,
		),
		vec![],
		None,
		None,
		None,
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
				authority_keys_from_seed("Bob"),
			],
			// Sudo account
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			// Pre-funded accounts
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_account_id_from_seed::<sr25519::Public>("Eve"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
				get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
				get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		"trust Testnet 1",
		"trust_testnet1",
		ChainType::Live,
		move || testnet_genesis(
			wasm_binary,
			testnet_authorities(),
			testnet_root(),
			vec![
				// 5HYbaX5ZAYjNXpLhmH8LMok4Nf8And1kd1maKvj1fQgbrnBK
				hex!("f272e74ac1a79a1f375788bd8b7449023eb8709b6fa458a7f4006327b62a951a").into(),
				// 5DFMx3uqVu1gaQ4JFb3c9ArS28EggtxUugxLKpYkHFbN8sEc
				hex!("345ceb562c0dec7a2fa2231aa21bdab6bd3712a587a203550a9bbdb495fa5969").into(),
				// 5CcRiAqEXHLykeibjLLye4asBirJydANSLAShHHjyNj2ig4Z
				hex!("18317fb76f0d670623c715e67e5fac24863dd49e73789d162fcab890420f391b").into(),
				// 5CLVZtuj3dieNHDSVmD9rpPE1rfotgpknw1C4E6p6DXve1Lm
				hex!("0c0a8eaff79cf40af6181da4710cf85daadbb6458493b66a4a32d71261b55a4e").into(),
			],
			true,
		),
		vec![
			"/ip4/34.210.80.113/tcp/30333/p2p/12D3KooWNNgBEeRgNhMeZ6iv1pMawU26fs77SMLZZay6DtBBncEm".parse()
				.expect("MultiaddrWithPeerId"),
		],
		None,
		Some("prc"),
		None,
		None
	))
}

fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	enable_println: bool
) -> GenesisConfig {

	GenesisConfig {
		frame_system: Some(SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
		}),
		pallet_aura: Some(AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		}),
		pallet_grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		}),
		pallet_sudo: Some(SudoConfig {
			// Assign network admin rights.
			key: root_key,
		}),
		pallet_contracts: Some(ContractsConfig {
			current_schedule: pallet_contracts::Schedule {
					enable_println,
					..Default::default()
			},
		}),
	}
}
