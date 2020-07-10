import $ from "jquery";

import * as wasm from "core-wasm";
wasm.init();

$("#generate-address").click(() => {
    var gen = wasm.GeneratedAddress.generate();
    $("#private_key").val(gen.private_key());
    $("#public_key").val(gen.public_key());
    $("#address").val(gen.address());
});
