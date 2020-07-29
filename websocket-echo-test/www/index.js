import $ from "jquery";

import * as wasm from "core-wasm";
wasm.init();

$('#ws_text_echo_test').click(async () => {
    const result = await wasm.ws_text_echo_test();
    console.log(result);
});

$('#ws_binary_echo_test').click(async () => {
    const result = await wasm.ws_binary_echo_test();
    console.log(result);
});
