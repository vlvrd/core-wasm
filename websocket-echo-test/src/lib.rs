#[macro_use]
extern crate log;

use std::panic;
use wasm_bindgen::prelude::*;
use cfg_if::cfg_if;

use ws_stream_wasm::*;

use futures::prelude::* ;

use js_sys::Uint8Array;

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

const ECHO_URL: &str = "wss://echo.websocket.org";

fn ws_message_to_string(msg: WsMessage) -> Result<String, String> {
    match msg {
        WsMessage::Text(string) => Ok(string),
        _ => Err("Cannot stringify binary data".to_string()),
    }
}

/**
 * https://github.com/najamelan/ws_stream_wasm/blob/master/tests/ws_io.rs#L41
 */
#[wasm_bindgen]
pub async fn ws_text_echo_test() -> String {
    info!("Starting WS Text Echo Test");

    let (_ws, mut wsio) = WsMeta::connect(ECHO_URL, None).await.expect_throw("Could not create websocket");
    let message = "Hello from Nimiq".to_string();

    wsio.send(WsMessage::Text(message.clone())).await.expect_throw("Failed to write to websocket");

    let msg = wsio.next().await;
    let result = msg.expect_throw("Websocket stream closed");

    assert_eq!(WsMessage::Text(message), result);

    ws_message_to_string(result).unwrap()
}


/**
 * https://github.com/najamelan/ws_stream_wasm/blob/master/tests/ws_io.rs#L68
 */
#[wasm_bindgen]
pub async fn ws_binary_echo_test() -> Uint8Array {
    info!("Starting WS Binary Echo Test");

    let (_ws, mut wsio) = WsMeta::connect(ECHO_URL, None).await.expect_throw("Could not create websocket");
    let message = b"Hello from browser".to_vec();

    wsio.send(WsMessage::Binary(message.clone())).await
        .expect_throw("Failed to write to websocket");

    let msg = wsio.next().await;
    let result = msg.expect_throw("Stream closed");

    assert_eq!(WsMessage::Binary(message), result);

    Uint8Array::from(&Vec::from(result)[..])
}
