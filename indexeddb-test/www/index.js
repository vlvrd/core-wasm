import $ from "jquery";

import * as wasm from "core-wasm";
wasm.init();

$('#indexeddb_test').click(async () => {
    const result = await wasm.indexeddb_test();
    console.log(result);
});