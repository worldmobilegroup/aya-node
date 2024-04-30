use std::collections::BTreeMap;

use fp_evm::GenesisAccount;
use hex_literal::hex;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
// Substrate
use sc_chain_spec::{ChainType, Properties};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{H160, Pair, Public};
use sp_core::crypto::Ss58Codec;
#[allow(unused_imports)]
use sp_core::ecdsa;
use sp_runtime::traits::{IdentifyAccount, Verify};

// Frontier
use aya_runtime::{
    AccountId, Balance, opaque::SessionKeys, RuntimeGenesisConfig, Signature, SS58Prefix,
    WASM_BINARY,
};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig>;

const PROTOCOL_ID: &str = "aya";
const TOKEN_SYMBOL: &str = "FERN";

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

#[allow(dead_code)]
type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
/// For use with `AccountId32`, `dead_code` if `AccountId20`.
#[allow(dead_code)]
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

fn session_keys(aura: AuraId, grandpa: GrandpaId, im_online: ImOnlineId) -> SessionKeys {
    SessionKeys {
        aura,
        grandpa,
        im_online,
    }
}

pub fn authority_keys_from_seed(
    s: &str,
    a: AccountId,
) -> (AccountId, AuraId, GrandpaId, ImOnlineId) {
    (
        a,
        get_from_seed::<AuraId>(s),
        get_from_seed::<GrandpaId>(s),
        get_from_seed::<ImOnlineId>(s),
    )
}

fn properties() -> Properties {
    let mut properties = Properties::new();
    properties.insert("isEthereum".into(), true.into());
    properties.insert("tokenDecimals".into(), 18.into());
    properties.insert("ss58Format".into(), SS58Prefix::get().into());
    properties.insert("tokenSymbol".to_string(), TOKEN_SYMBOL.into());
    properties
}

const UNITS: Balance = 1_000_000_000_000_000_000;

pub fn development_config(enable_manual_seal: bool) -> ChainSpec {
    ChainSpec::builder(WASM_BINARY.expect("WASM not available"), Default::default())
        .with_name("Development")
        .with_id("dev")
        .with_chain_type(ChainType::Development)
        .with_properties(properties())
        .with_genesis_config_patch(testnet_genesis(
            // Sudo account (Alith)
            AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")),
            // Pre-funded accounts
            vec![
                AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")), // Alith
                AccountId::from(hex!("3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0")), // Baltathar
                AccountId::from(hex!("798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc")), // Charleth
                AccountId::from(hex!("773539d4Ac0e786233D90A233654ccEE26a613D9")), // Dorothy
                AccountId::from(hex!("Ff64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB")), // Ethan
                AccountId::from(hex!("C0F0f4ab324C46e55D02D0033343B4Be8A55532d")), // Faith
            ],
            // Initial PoA authorities
            vec![authority_keys_from_seed(
                "Alice",
                AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")),
            )],
            // Ethereum chain ID
            SS58Prefix::get() as u64,
            enable_manual_seal,
        ))
        .with_protocol_id(PROTOCOL_ID)
        .build()
}

pub fn local_testnet_config() -> ChainSpec {
    ChainSpec::builder(WASM_BINARY.expect("WASM not available"), Default::default())
        .with_name("Local Testnet")
        .with_id("local_testnet")
        .with_chain_type(ChainType::Local)
        .with_properties(properties())
        .with_genesis_config_patch(testnet_genesis(
            // Sudo account (Alith)
            AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")),
            // Pre-funded accounts
            vec![
                AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")), // Alith
                AccountId::from(hex!("3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0")), // Baltathar
                AccountId::from(hex!("798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc")), // Charleth
                AccountId::from(hex!("773539d4Ac0e786233D90A233654ccEE26a613D9")), // Dorothy
                AccountId::from(hex!("Ff64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB")), // Ethan
                AccountId::from(hex!("C0F0f4ab324C46e55D02D0033343B4Be8A55532d")), // Faith
            ],
            vec![
                authority_keys_from_seed(
                    "Alice",
                    AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")),
                ),
                authority_keys_from_seed(
                    "Bob",
                    AccountId::from(hex!("3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0")),
                ),
            ],
            1357,
            false,
        ))
        .with_protocol_id(PROTOCOL_ID)
        .build()
}

pub fn devnet_config() -> ChainSpec {
    ChainSpec::builder(WASM_BINARY.expect("WASM not available"), Default::default())
        .with_name("Devnet")
        .with_id("aya_devnet")
        .with_chain_type(ChainType::Live)
        .with_properties(properties())
        .with_genesis_config_patch(testnet_genesis(
            // Sudo account (Frigga)
            AccountId::from(hex!("be2b160c405f48C966D500D51825Ca7C3b895115")),
            // Pre-funded accounts
            vec![
                AccountId::from(hex!("be2b160c405f48C966D500D51825Ca7C3b895115")), // Frigga
                AccountId::from(hex!("bDEfFf4E3c33130f712b8ade58a9a2ec6508F87a")), // Ullr
                AccountId::from(hex!("34a2a056E8D76c237f614498A7d4A4b66bb07a05")), // Freyr
                AccountId::from(hex!("e89784D78F3F7d6d9aD117952eBC0946e661E7dF")), // Tyr
                AccountId::from(hex!("9c33Afb3f5Fbf26b0Ee239dD409971c199558E5a")), // Vidarr
                AccountId::from(hex!("90df1a639bfF37D23a240bEc0BBA19585D74956D")), // Baldr
            ],
            vec![
                (
                    AccountId::from(hex!("be2b160c405f48C966D500D51825Ca7C3b895115")),
                    AuraId::from_ss58check("5DLejswkk5ZkYCadBbbeHjYS1pEkHBSmtGnbQ5mF8TVctG6R").unwrap(),
                    GrandpaId::from_ss58check("5E8oFZ6d5JexFBA573hKYsJnspZ8CJFYTKjWE5194eAMMgSQ").unwrap(),
                    ImOnlineId::from_ss58check("5DLejswkk5ZkYCadBbbeHjYS1pEkHBSmtGnbQ5mF8TVctG6R").unwrap(),
                ),
                (
                    AccountId::from(hex!("bDEfFf4E3c33130f712b8ade58a9a2ec6508F87a")),
                    AuraId::from_ss58check("5HWgpxBm7jY8GKFabTnBhdHbBPkucMPDNpHyTRHzCZYPUd2z").unwrap(),
                    GrandpaId::from_ss58check("5DrCBGSzmUexKxc1DPitNF7uChZdHJZdZER6a5nxRrvmzeDx").unwrap(),
                    ImOnlineId::from_ss58check("5HWgpxBm7jY8GKFabTnBhdHbBPkucMPDNpHyTRHzCZYPUd2z").unwrap(),
                ),
            ],
            1357,
            false,
        ))
        .with_protocol_id(PROTOCOL_ID)
        .build()
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    sudo_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    initial_authorities: Vec<(AccountId, AuraId, GrandpaId, ImOnlineId)>,
    chain_id: u64,
    enable_manual_seal: bool,
) -> serde_json::Value {
    let evm_accounts = {
        let mut map: BTreeMap<H160, GenesisAccount> = BTreeMap::new();
        // map.insert(
        //     // H160 address for benchmark usage
        //     H160::from_str("1000000000000000000000000000000000000001")
        //         .expect("internal H160 is valid; qed"),
        //     fp_evm::GenesisAccount {
        //         nonce: U256::from(1),
        //         balance: U256::from(1_000_000_000_000_000_000_000_000u128),
        //         storage: Default::default(),
        //         code: vec![0x00],
        //     },
        // );
        map
    };

    serde_json::json!({
        "sudo": { "key": Some(sudo_key) },
        "balances": {
            "balances": endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1_000_000 * UNITS))
                .collect::<Vec<_>>()
        },
        "validatorSet" : { "initialValidators" : initial_authorities.iter().map(|x| x.0.clone()).collect::<Vec<_>>(), },
        "session" : { "keys": initial_authorities.iter().map(|x| {
            (x.0.clone(), x.0.clone(), session_keys(x.1.clone(), x.2.clone(), x.3.clone()))
        }).collect::<Vec<_>>(),},
        "aura": { "authorities": [] },
        "grandpa": { "authorities": [] },
        "imOnline": { "keys": [] },
        "evmChainId": { "chainId": chain_id },
        "evm": { "accounts": evm_accounts },
        "manualSeal": { "enable": enable_manual_seal }
    })
}
