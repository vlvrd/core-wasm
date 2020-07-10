import $ from "jquery";

import * as wasm from "core-wasm";
wasm.init();

$("#generate-address").click(() => {
    var gen = wasm.GeneratedAddress.generate();
    $("#private_key").val(gen.private_key());
    $("#public_key").val(gen.public_key());
    $("#address").val(gen.address());
});

$('#create-transaction').click(() => {
    const priv_key = $('#tx_priv_key').val();
    const to = $('#tx_to').val();
    const value = $('#tx_value').val();
    const fee = $('#tx_fee').val();
    const validity_start_height = $('#tx_validity_start_height').val();

    const tx = wasm.create_transaction(priv_key, to, value, fee, validity_start_height);
    console.log(tx);
});
