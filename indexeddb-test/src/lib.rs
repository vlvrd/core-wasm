#[macro_use]
extern crate log;

use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use std::panic;
use wasm_bindgen::*;
use wasm_bindgen::prelude::*;

use indexeddb::{KeyPath, TransactionMode};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GeneratedAddress {
    private_key: String,
    public_key: String,
    address: String,
}

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

//    init_db().await();

    info!("WASM playground initialized");

    Ok(())
}

// TODO: Init database for the whole example
async fn init_db() -> Result<(), ()> {
    let db = indexeddb::open("indexeddb_test_db", 1, |_, upgrader| {
        let obj_store = upgrader
            .create_object_store("addresses", KeyPath::Single("id".into()), true)
            .unwrap();
        obj_store.create_index("private_keys", "private_key", true).unwrap();
    })
    .await
    .unwrap();

    Ok(())
}

#[wasm_bindgen]
pub async fn indexeddb_test() -> String {
    info!("Starting IndexedDB Test");

    let db = indexeddb::open("indexeddb_test_db", 1, |_, upgrader| {
        let obj_store = upgrader
            .create_object_store("addresses", KeyPath::Single("id".into()), true)
            .unwrap();
        obj_store.create_index("private_keys", "private_key", true).unwrap();
    })
    .await
    .unwrap();

    let tx = db.transaction(TransactionMode::ReadWrite);
    let object_store = tx.object_store("adresses").unwrap();

    object_store
    .put(
        GeneratedAddress {
            private_key: "123456789".to_string(),
            public_key: "123456789".to_string(),
            address: "123456789".to_string(),
        },
        None,
    )
    .await
    .unwrap();

    let test: Vec<GeneratedAddress> = object_store.get_all().await.unwrap();
    let address: String = test[0].address.clone();

    address
}