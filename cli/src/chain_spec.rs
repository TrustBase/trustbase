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


// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! TrustBase chain configurations.

use sc_chain_spec::ChainSpecExtension;
use sp_core::{Pair, Public, crypto::UncheckedInto, sr25519};
use serde::{Serialize, Deserialize};
use node_runtime::{
	AuthorityDiscoveryConfig, BabeConfig, BalancesConfig, CouncilConfig,
	DemocracyConfig, GrandpaConfig, ImOnlineConfig, SessionConfig, SessionKeys, StakerStatus,
	StakingConfig, ElectionsConfig, IndicesConfig, SocietyConfig, SudoConfig, SystemConfig,
	TechnicalCommitteeConfig, wasm_binary_unwrap, MAX_NOMINATIONS,
};
use node_runtime::Block;
use node_runtime::constants::currency::*;
use sc_service::ChainType;
use hex_literal::hex;
use sc_telemetry::TelemetryEndpoints;
use grandpa_primitives::{AuthorityId as GrandpaId};
use sp_consensus_babe::{AuthorityId as BabeId};
use pallet_im_online::sr25519::{AuthorityId as ImOnlineId};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_runtime::{Perbill, traits::{Verify, IdentifyAccount}};

pub use node_primitives::{AccountId, Balance, Signature};
pub use node_runtime::GenesisConfig;

type AccountPublic = <Signature as Verify>::Signer;

const TRUSTCC2_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some TrustBase core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<
	GenesisConfig,
	Extensions,
>;
/// Flaming Fir testnet generator
pub fn trust_cc2_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../res/trust_cc2.json")[..])
}

fn session_keys(
	grandpa: GrandpaId,
	babe: BabeId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { grandpa, babe, im_online, authority_discovery }
}

fn testnet_config_genesis() -> GenesisConfig {
	// stash, controller, session-key
	// generated with secret:
	// for i in 1 2 3 4 ; do for j in stash controller; do subkey inspect "$secret"/fir/$j/$i; done; done
	// and
	// for i in 1 2 3 4 ; do for j in session; do subkey --ed25519 inspect "$secret"//fir//$j//$i; done; done

	let initial_authorities: Vec<(AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId)> = vec![(
		// 5FHAoSx5RpeaxAfAasqoLaFJShf4nT7Nzg8wArCAnkUSc35G
		hex!["8e3659306967d22db66360dae35a250a816a49a0560fd340bd23cb67d5c24f69"].into(),
		// 5EhRVJobxCD3L8mNzazaRApzhwqXjUrRFYVFwNpxmrnymLpp
		hex!["7479723f8e880cd7c54671c3c012405ec3229974b3c3e1f455472e7c60245b41"].into(),
		// 5HWcZhWqCQm9w2oKct1UXkymVCkQf9W1DsjSNbWb9C4dzbue
		hex!["f0efbb3ba8b3b507ef8ab80f203832554a55913b3092e89b9797b066d6bde209"].unchecked_into(),
		// 5EPQnq4rbr8HhjXV8AEXFVWqydPKTcm9ww3FdkwBsKmcDdZE
		hex!["66bcadfe4c77ebf6f270b1c983327fe799e75c43fbd0ff245d0964a4e8e7240b"].unchecked_into(),
		// 5EPQnq4rbr8HhjXV8AEXFVWqydPKTcm9ww3FdkwBsKmcDdZE
		hex!["66bcadfe4c77ebf6f270b1c983327fe799e75c43fbd0ff245d0964a4e8e7240b"].unchecked_into(),
		// 5EPQnq4rbr8HhjXV8AEXFVWqydPKTcm9ww3FdkwBsKmcDdZE
		hex!["66bcadfe4c77ebf6f270b1c983327fe799e75c43fbd0ff245d0964a4e8e7240b"].unchecked_into(),
	),(
		// 5EZZ2v4mX3cbMPn7nh2XztDXmPhQqEjqCY91p58BgNoYBUWk
		hex!["6e78e1b4a82e03c957fd6e2bb393de72372180a4d5c3c74011cad0f75554d649"].into(),
		// 5GH8r7cTKQYfZF5KPQC9T5X9CeUx7N5NCQuC1se8kEfzTcNR
		hex!["ba6c0107240c63cbb2c9f3ee12e0beeaf2c15e193016b6378261c09b3a97ab4c"].into(),
		// 5FYfZinXLfRx5z6AsYAmRXV99RYqobCjnboFTZWDi8Jr1nJS
		hex!["9a07dafeffeafc8a9c7212afd9a2de8f3681fcaa7e0d86413889f1304d7cbaf6"].unchecked_into(),
		// 5EjzSDzW4bZKWU2Q5Jz8HbiijNqabpW1wV6wDYVCNyWPM8rF
		hex!["766ed89755181326d9a0a5921e4dec15efea32f5b3162b076f9661cf9ae8ed7c"].unchecked_into(),
		// 5EjzSDzW4bZKWU2Q5Jz8HbiijNqabpW1wV6wDYVCNyWPM8rF
		hex!["766ed89755181326d9a0a5921e4dec15efea32f5b3162b076f9661cf9ae8ed7c"].unchecked_into(),
		// 5EjzSDzW4bZKWU2Q5Jz8HbiijNqabpW1wV6wDYVCNyWPM8rF
		hex!["766ed89755181326d9a0a5921e4dec15efea32f5b3162b076f9661cf9ae8ed7c"].unchecked_into(),
	),(
		// 5EhFQxYQ5R69snnTaaFf1HdAepWFEudHHkgEqQPjsRwWcwLn
		hex!["745787e5956204ec868069ad79cd5606a170760967b9b23b66c9fd6a1406d830"].into(),
		// 5FWAFEQGjVzo4MVxnWCh5wSgtakMcP1KvPZ8yAMMjYxqj9rK
		hex!["981eab3829581e9274d32379d51d21a3b21879695217bf99fb41c22c77d49a3e"].into(),
		// 5Es66HgxzDabC9vS9LjjPxyixY9kEsNi8VJgXoXRoyyBD15d
		hex!["7bd89ac6e1a6ad194c244f0c642e0b6b40a0e477c0e6df46a5eb60b0705e41c3"].unchecked_into(),
		// 5GWTionxm9rN7PsVyJwKa3Q9ij62yoTX9TMKeTBF8YWz9MLB
		hex!["c495bad86a3642811ebb54a6e8f08cfb2c6ed281c184920c6eacac634e48ee74"].unchecked_into(),
		// 5GWTionxm9rN7PsVyJwKa3Q9ij62yoTX9TMKeTBF8YWz9MLB
		hex!["c495bad86a3642811ebb54a6e8f08cfb2c6ed281c184920c6eacac634e48ee74"].unchecked_into(),
		// 5GWTionxm9rN7PsVyJwKa3Q9ij62yoTX9TMKeTBF8YWz9MLB
		hex!["c495bad86a3642811ebb54a6e8f08cfb2c6ed281c184920c6eacac634e48ee74"].unchecked_into(),
	),(
		// 5CnkJUAxV4MoULHyYut4UkQYLzKR5p71usm5SyHJfV2hcfRG
		hex!["2010897b516c6cd0c43e727690a8dc3fcb19dccd126b9625e08a9fcd65944f13"].into(),
		// 5C5HKrq3uCT3YnwTjHACJHXtzmHcK1SiovxyEczoeYJL81uQ
		hex!["0070aa837f8b46ab5bef7d137cdeb3af2a77152357886c364c0e126451bc9422"].into(),
		// 5DXSupP8CVJqn1XvswbxR3FrywT7B7oCBCcMuexYPTPqfVG2
		hex!["40a18c31daa63f713fbc3a089e0eb696381d0af23f015342ad98a995a3a9a125"].unchecked_into(),
		// 5CQJ8SW3WXPfMQbbriNScUFF3sxTJa7Hfk2Y1a7axkstMJa3
		hex!["0ef10842f02694ca5e4a7e51838aed4f0b02a2e94e92d16154972c2f1b575246"].unchecked_into(),
		// 5CQJ8SW3WXPfMQbbriNScUFF3sxTJa7Hfk2Y1a7axkstMJa3
		hex!["0ef10842f02694ca5e4a7e51838aed4f0b02a2e94e92d16154972c2f1b575246"].unchecked_into(),
		// 5CQJ8SW3WXPfMQbbriNScUFF3sxTJa7Hfk2Y1a7axkstMJa3
		hex!["0ef10842f02694ca5e4a7e51838aed4f0b02a2e94e92d16154972c2f1b575246"].unchecked_into(),
	)];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = hex![
		// 5DLNQLALs9aG927EPyrTq8xjCki1LP35a5VDFQX3rBWo1mAP
		"382eabedd42655978ddc2e4203677500c21df8685c93c1cb4c6d5f50a9b4f045"
	].into();

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

	testnet_genesis(
		initial_authorities,
		vec![],
		root_key,
		Some(endowed_accounts)
	)
}

/// Staging testnet config.
pub fn testnet_config() -> ChainSpec {
	let boot_nodes = vec![];
	ChainSpec::from_genesis(
		"trust_cc2 Testnet",
		"trust_cc2_testnet",
		ChainType::Live,
		testnet_config_genesis,
		boot_nodes,
		Some(TelemetryEndpoints::new(vec![(TRUSTCC2_TELEMETRY_URL.to_string(), 0)])
			.expect("trust_cc2 telemetry url is valid; qed")),
		None,
		None,
		Default::default(),
	)
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(seed: &str) -> (
	AccountId,
	AccountId,
	GrandpaId,
	BabeId,
	ImOnlineId,
	AuthorityDiscoveryId,
) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	initial_nominators: Vec<AccountId>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> GenesisConfig {
	let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
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
		]
	});
	// endow all authorities and nominators.
	initial_authorities.iter().map(|x| &x.0).chain(initial_nominators.iter()).for_each(|x| {
		if !endowed_accounts.contains(&x) {
			endowed_accounts.push(x.clone())
		}
	});

	// stakers: all validators and nominators.
	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
		.chain(initial_nominators.iter().map(|x| {
			use rand::{seq::SliceRandom, Rng};
			let limit = (MAX_NOMINATIONS as usize).min(initial_authorities.len());
			let count = rng.gen::<usize>() % limit;
			let nominations = initial_authorities
				.as_slice()
				.choose_multiple(&mut rng, count)
				.into_iter()
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), STASH, StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();

	let num_endowed_accounts = endowed_accounts.len();

	const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
	const STASH: Balance = ENDOWMENT / 2;

	GenesisConfig {
		system: SystemConfig {
			code: wasm_binary_unwrap().to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			balances: endowed_accounts.iter().cloned()
				.map(|x| (x, ENDOWMENT))
				.collect()
		},
		indices: IndicesConfig {
			indices: vec![],
		},
		session: SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.0.clone(), session_keys(
					x.2.clone(),
					x.3.clone(),
					x.4.clone(),
					x.5.clone(),
				))
			}).collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			.. Default::default()
		},
		democracy: DemocracyConfig::default(),
		elections: ElectionsConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.map(|member| (member, STASH))
						.collect(),
		},
		council: CouncilConfig::default(),
		technical_committee: TechnicalCommitteeConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.collect(),
			phantom: Default::default(),
		},
		sudo: SudoConfig {
			key: root_key,
		},
		babe: BabeConfig {
			authorities: vec![],
			epoch_config: Some(node_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		im_online: ImOnlineConfig {
			keys: vec![],
		},
		authority_discovery: AuthorityDiscoveryConfig {
			keys: vec![],
		},
		grandpa: GrandpaConfig {
			authorities: vec![],
		},
		technical_membership: Default::default(),
		treasury: Default::default(),
		society: SocietyConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.collect(),
			pot: 0,
			max_members: 999,
		},
		transaction_storage: Default::default(),
	}
}

fn development_config_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![
			authority_keys_from_seed("Alice"),
		],
		vec![],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		development_config_genesis,
		vec![],
		None,
		None,
		None,
		Default::default(),
	)
}

fn local_testnet_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![
			authority_keys_from_seed("Alice"),
			authority_keys_from_seed("Bob"),
		],
		vec![],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		local_testnet_genesis,
		vec![],
		None,
		None,
		None,
		Default::default(),
	)
}

#[cfg(test)]
pub(crate) mod tests {
	use super::*;
	use crate::service::{new_full_base, new_light_base, NewFullBase};
	use sc_service_test;
	use sp_runtime::BuildStorage;

	fn local_testnet_genesis_instant_single() -> GenesisConfig {
		testnet_genesis(
			vec![
				authority_keys_from_seed("Alice"),
			],
			vec![],
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			None,
		)
	}

	/// Local testnet config (single validator - Alice)
	pub fn integration_test_config_with_single_authority() -> ChainSpec {
		ChainSpec::from_genesis(
			"Integration Test",
			"test",
			ChainType::Development,
			local_testnet_genesis_instant_single,
			vec![],
			None,
			None,
			None,
			Default::default(),
		)
	}

	/// Local testnet config (multivalidator Alice + Bob)
	pub fn integration_test_config_with_two_authorities() -> ChainSpec {
		ChainSpec::from_genesis(
			"Integration Test",
			"test",
			ChainType::Development,
			local_testnet_genesis,
			vec![],
			None,
			None,
			None,
			Default::default(),
		)
	}

	#[test]
	#[ignore]
	fn test_connectivity() {
		sc_service_test::connectivity(
			integration_test_config_with_two_authorities(),
			|config| {
				let NewFullBase { task_manager, client, network, transaction_pool, .. }
					= new_full_base(config,|_, _| ())?;
				Ok(sc_service_test::TestNetComponents::new(task_manager, client, network, transaction_pool))
			},
			|config| {
				let (keep_alive, _, client, network, transaction_pool) = new_light_base(config)?;
				Ok(sc_service_test::TestNetComponents::new(keep_alive, client, network, transaction_pool))
			}
		);
	}

	#[test]
	fn test_create_development_chain_spec() {
		development_config().build_storage().unwrap();
	}

	#[test]
	fn test_create_local_testnet_chain_spec() {
		local_testnet_config().build_storage().unwrap();
	}

	#[test]
	fn test_staging_test_net_chain_spec() {
		testnet_config().build_storage().unwrap();
	}
}
