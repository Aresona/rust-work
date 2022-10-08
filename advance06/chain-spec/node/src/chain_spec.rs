use hex_literal::hex;
use node_primitives::*;
use node_template_runtime::{
	constants::currency::*, opaque::SessionKeys, wasm_binary_unwrap, BabeConfig, BalancesConfig,
	CouncilConfig, DemocracyConfig, ElectionsConfig, GenesisConfig, GrandpaConfig, ImOnlineConfig,
	MaxNominations, NominationPoolsConfig, SessionConfig, StakerStatus, StakingConfig, SudoConfig,
	SystemConfig, TechnicalCommitteeConfig, BABE_GENESIS_EPOCH_CONFIG,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

fn session_keys(babe: BabeId, grandpa: GrandpaId, im_online: ImOnlineId) -> SessionKeys {
	SessionKeys { babe, grandpa, im_online }
}

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AccountId, BabeId, GrandpaId, ImOnlineId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", s)),
		get_account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<BabeId>(s),
		get_from_seed::<GrandpaId>(s),
		get_from_seed::<ImOnlineId>(s),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {
	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				vec![],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				vec![],
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
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		None,
		// Extensions
		None,
	))
}

pub fn staging_network_config() -> ChainSpec {
	let boot_nodes = vec![];

	ChainSpec::from_genesis(
		"Substrate Stencil",
		"stencil_network",
		ChainType::Live,
		staging_network_config_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		None,
		None,
		None,
		Default::default(),
	)
}

fn staging_network_config_genesis() -> GenesisConfig {
	// for i in 1 2 3 4; do for j in stash controller; do subkey inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in babe; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in grandpa; do subkey --ed25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in im_online; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	let initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId)> = vec![
		(
			// 5GGsLNUHKEbJnNP4hnbQRwTQf6E8h1AdPqrUtAZ2Bb7wtypZ
			hex!["0xba37c87f60ebc60460db249a498c513f2d3858ed16c17d0a2b6d898f5fa9f059"].into(),
			// 5Fh1SSgybjCKZgXcKEmBkrt8vNdu6nBi2yz45Dm1U73T1x45
			hex!["0xa064b8e905c40c1cb20cb74ecd60648befc58b6ee2c0a9ce9fd5941718702546"].into(),
			// 5Grz8jseiqJZ8vvpZRTzKNPvFd9TxSEoe9i5GCrZpTYakqsB
			hex!["0xd43d0417895bdb274e0fe3130045e04be549cad44a7cd30983039e9ba658725b"]
				.unchecked_into(),
			// 5HRmbC9mrQDaVNkXAbuTstYv3St8GVaGRojWYXRSNwe6UJ5X
			hex!["0xed3de343da8c647ab04a44dba513ab5322621e3b53ac36dd6384e5b0317a6687"]
				.unchecked_into(),
			// 5GUAvxuobq8cfassAKTupBtDVLZzhtXcQjdbLLPV1fQ8p5VZ
			hex!["0xc2d6b7b1abf7669ba2636af6091fa911c5424ea0cb0cd1eec6194d9ba7158e1f"]
				.unchecked_into(),
		),
		(
			// 5Ecoea3BU226SNgB6YYv77wPUgwn1F3K4qWkqZiqdnGRZvJ5
			hex!["0x70f3d0c23873e0ee25cfb8d6c77dcef9f2b0aea860d8197721b5d1909430375d"].into(),
			// 5EsjYEoCaYzLunqrwyPaChcGie7dUe4wF8GKjFHYizKKaCHQ
			hex!["0x7c56a9f13137a5fcb3c0f9acbdff04418540b116eb771ae998bba6c729f09825"].into(),
			// 5HSosirwi8pQzuikiLDB5QQ2Fwxcgqf3gd6QMJWky9RF7ccN
			hex!["0xee08d33b52d26d92cb3158261a5efa59917ad145d586532b46f0c9b371b70443"]
				.unchecked_into(),
			// 5EReqHr85oWyW8iBCsWtapczegai5RPKVwbnFSnzUjqz8mJ6
			hex!["0x6872710961bbd29f79ce129bc596f48163a4bccce28b4c05e9bad267ea3fab15"]
				.unchecked_into(),
			// 5GQsXwvbu4Au4f28vGqB5AgHKBAfasjh3w5fUdVJLNw7mGW3
			hex!["0xc0526b186c8ea39766211f3e22a96513cafb19679e49cc4cfbf2c48a7ae1f916"]
				.unchecked_into(),
		),
		(
			// 5EL6rajXMe6StXFwaRSNtzoi5tzffYrf8m8AEkKDykfFBDfa
			hex!["0x6436915f71d64fa09d94a4af91474e5e3564356be7a0ccb53ccafb6aa60dda64"].into(),
			// 5FjnnXq3BTW71FiQxw3Cnzp7zc21bF4LF9kSP2mvJzUCzv2B
			hex!["0xa283dcd04c7fbe384f615ae40655bb3e505605df96e8b5b0bb64c83241fd313e"].into(),
			// 5E9t5uASkS124ufNaHGrRNCFnmEn2RutZe797VGj8cPC8EFx
			hex!["0x5c6b1ff7dcb2422bb80769f3af90b03036e941320de0673476db63804e1e8a4e"]
				.unchecked_into(),
			// 5G1B5zQcTajNtAKVuC86PsGFEH6hNd86NcpKAnuE95qNW2Gp
			hex!["0xae3f9d90dc81176374cdf5b47f68d4f10cbd5baf25e85bbae6a8bb472715966d"]
				.unchecked_into(),
			// 5ELVmv3cRW15hwSe8sKT5EAjntxTy3eh9mjk7d1EX8XCNmBA
			hex!["0x6483b8cddb4aa9d167abce4523c6a48a96eff096776c71f6e3a6381e045b4160"]
				.unchecked_into(),
		),
		(
			// 5C597eSEXBQx2KxQ27TEDRi7CvGJj3Dd8s6bLtwXWKmSFpxa
			hex!["0x005506dbc09f06f73bc26bb8716e4905b46d3b3993d35171ee4954b238824650"].into(),
			// 5EpkdTwFVYbd5XZVSuG25JvK3tBitfc1LmPFZN9Tua93xKhw
			hex!["0x7a1098e6a92b3d0ec16466d34497a068c35a132ced8d536efb5eb19dc5dd4a1a"].into(),
			// 5DcWDqZ97KoQbVRVMXDpbN2fCxCWFr6KciuW4omwuwKMQVAQ
			hex!["0x447ceb60c03518941520b38886832d2eadacafb56cce426b4affbee0038fa958"]
				.unchecked_into(),
			// 5DonCXHBSuRDCVbRq7P57t6Nox8rYEuZ6onhAdnDD3MfQCtQ
			hex!["0x4d1667430dd50a318dceaf67066cb4e283a60fc847fd668e2f7585e3084d88bb"]
				.unchecked_into(),
			// 5HbTPzriH3vEVyqGu4quUtuH4QMiyMMgvXqD57PhYNKXsUuk
			hex!["0xf4a11941ab186eae556f5752b665b35cca8d00611b4091e245f15cd329f50259"]
				.unchecked_into(),
		),
	];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = hex![
		// 5FemZuvaJ7wVy4S49X7Y9mj7FyTR4caQD5mZo2rL7MXQoXMi
		"9eaf896d76b55e04616ff1e1dce7fc5e4a417967c17264728b3fd8fee3b12f3c"
	]
	.into();

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

	testnet_genesis(initial_authorities, vec![], root_key, endowed_accounts)
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId)>,
	initial_nominators: Vec<AccountId>,
	root_key: AccountId,
	mut endowed_accounts: Vec<AccountId>,
) -> GenesisConfig {
	// endow all authorities and nominators.
	initial_authorities
		.iter()
		.map(|x| &x.0)
		.chain(initial_nominators.iter())
		.for_each(|x| {
			if !endowed_accounts.contains(x) {
				endowed_accounts.push(x.clone())
			}
		});

	// stakers: all validators and nominators.
	const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
	const STASH: Balance = ENDOWMENT / 1000;
	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
		.chain(initial_nominators.iter().map(|x| {
			use rand::{seq::SliceRandom, Rng};
			let limit = (MaxNominations::get() as usize).min(initial_authorities.len());
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

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary_unwrap().to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, ENDOWMENT)).collect(),
		},
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(x.0.clone(), x.0.clone(), session_keys(x.2.clone(), x.3.clone(), x.4.clone()))
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			// TODO: ForceEra::ForceNone
			..Default::default()
		},
		babe: BabeConfig { authorities: vec![], epoch_config: Some(BABE_GENESIS_EPOCH_CONFIG) },
		grandpa: GrandpaConfig { authorities: vec![] },
		im_online: ImOnlineConfig { keys: vec![] },
		democracy: DemocracyConfig::default(),
		elections: ElectionsConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.map(|member| (member, STASH))
				.collect(),
		},
		council: CouncilConfig::default(),
		technical_committee: TechnicalCommitteeConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default(),
		},
		technical_membership: Default::default(),
		treasury: Default::default(),
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
		nomination_pools: NominationPoolsConfig {
			min_create_bond: 10 * DOLLARS,
			min_join_bond: 1 * DOLLARS,
			..Default::default()
		},
	}
}
