#[macro_use]
extern crate log;

use std::panic;
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;
use cfg_if::cfg_if;
use hex;

use beserial::{Serialize};
use nimiq_keys::{Address, PrivateKey, PublicKey, KeyPair};
use nimiq_utils::key_rng::SecureGenerate;
use nimiq_hash::{Blake2bHash, Hash};
use nimiq_primitives::coin::Coin;
use nimiq_primitives::networks::NetworkId;
use nimiq_transaction::{TransactionFormat};
use nimiq_transaction_builder::{Recipient, TransactionBuilder};

// mod plain_transaction;
// use plain_transaction::{PlainData, PlainTransaction};

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn init() -> Result<(), JsValue> {
    let config = wasm_logger::Config::new(log::Level::Debug);
    wasm_logger::init(config);

    panic::set_hook(Box::new(console_error_panic_hook::hook));

    info!("WASM playground initialized");

    Ok(())
}

#[wasm_bindgen]
pub struct GeneratedAddress {
    private_key: String,
    public_key: String,
    address: String,
}

#[wasm_bindgen]
impl GeneratedAddress {
    pub fn generate() -> GeneratedAddress {
        let private_key = PrivateKey::generate_default_csprng();
        let public_key = PublicKey::from(&private_key);
        let address = Address::from(&public_key);
        GeneratedAddress {
            private_key: private_key.to_hex(),
            public_key: public_key.to_hex(),
            address: address.to_user_friendly_address()
        }
    }

    pub fn private_key(&self) -> String {
        self.private_key.clone()
    }

    pub fn public_key(&self) -> String {
        self.public_key.clone()
    }

    pub fn address(&self) -> String {
        self.address.clone()
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct PlainData {
    raw: String,
}

#[wasm_bindgen]
impl PlainData {
    pub fn raw(&self) -> String {
        self.raw.clone()
    }
}

#[wasm_bindgen]
pub struct PlainTransaction {
    transactionHash: String,
    format: String,
    sender: String,
    senderType: String,
    recipient: String,
    recipientType: String,
    pub value: u64,
    pub fee: u64,
    pub feePerByte: f64,
    pub validityStartHeight: u32,
    network: String,
    pub flags: u8,
    data: PlainData,
    proof: PlainData,
    pub size: u32,
    pub valid: bool,
}

#[wasm_bindgen]
impl PlainTransaction {
    #[wasm_bindgen(method, getter)]
    pub fn transactionHash(&self) -> String {
        self.transactionHash.clone()
    }

    #[wasm_bindgen(method, getter)]
    pub fn format(&self) -> String {
        self.format.clone()
    }

    #[wasm_bindgen(method, getter)]
    pub fn sender(&self) -> String {
        self.sender.clone()
    }

    #[wasm_bindgen(method, getter)]
    pub fn senderType(&self) -> String {
        self.senderType.clone()
    }

    #[wasm_bindgen(method, getter)]
    pub fn recipient(&self) -> String {
        self.recipient.clone()
    }

    #[wasm_bindgen(method, getter)]
    pub fn recipientType(&self) -> String {
        self.recipientType.clone()
    }

    #[wasm_bindgen(method, getter)]
    pub fn network(&self) -> String {
        self.network.clone()
    }

    #[wasm_bindgen(method, getter)]
    pub fn data(&self) -> PlainData {
        self.data.clone()
    }

    #[wasm_bindgen(method, getter)]
    pub fn proof(&self) -> PlainData {
        self.proof.clone()
    }
}

#[wasm_bindgen]
pub fn create_transaction(
    priv_key: String,
    to: String,
    value: u64,
    fee: u64,
    validity_start_height: u32,
    // network_id: u8,
) -> PlainTransaction {
    let private_key = priv_key.parse::<PrivateKey>().unwrap();
    let public_key = PublicKey::from(&private_key);
    let sender = Address::from(&public_key);

    let recipient = Address::from_user_friendly_address(&to).unwrap();

    let mut tx_builder = TransactionBuilder::new();
    tx_builder
        .with_sender(sender)
        .with_recipient(Recipient::new_basic(recipient))
        .with_value(Coin::try_from(value).unwrap())
        .with_fee(Coin::try_from(fee).unwrap())
        .with_validity_start_height(validity_start_height)
        .with_network_id(NetworkId::DevAlbatross);

    let mut proof_builder = tx_builder.generate().unwrap().unwrap_basic();
    proof_builder.sign_with_key_pair(&KeyPair::from(private_key));
    let tx = proof_builder.generate().unwrap();

    PlainTransaction {
        transactionHash: tx.hash::<Blake2bHash>().to_string(),
        format: match tx.format() {
            TransactionFormat::Basic => "basic".to_string(),
            TransactionFormat::Extended => "extended".to_string()
        },
        sender: tx.sender.to_user_friendly_address(),
        senderType: "basic".to_string(),
        recipient: tx.recipient.to_user_friendly_address(),
        recipientType: "basic".to_string(),
        value: u64::from(tx.value),
        fee: u64::from(tx.fee),
        feePerByte: tx.fee_per_byte(),
        validityStartHeight: tx.validity_start_height,
        network: tx.network_id.to_string(),
        flags: tx.flags.bits(),
        data: PlainData {
            raw: hex::encode(&tx.data),
        },
        proof: PlainData {
            raw: hex::encode(&tx.proof),
        },
        size: tx.serialized_size() as u32,
        valid: tx.verify(tx.network_id).is_ok(),
    }
}
