// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            3
// Async Callback (empty):               1
// Total number of exported functions:   5

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    second_contract
    (
        init => init
        acceptEsdtPayment => accept_esdt_payment
        rejectEsdtPayment => reject_esdt_payment
        getesdtTokenName => get_contract_esdt_token_identifier
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
