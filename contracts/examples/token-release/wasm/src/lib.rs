// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           13
// Async Callback (empty):               1
// Total number of exported functions:  15

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    token_release
    (
        init => init
        addFixedAmountGroup => add_fixed_amount_group
        addPercentageBasedGroup => add_percentage_based_group
        removeGroup => remove_group
        addUserGroup => add_user_group
        removeUser => remove_user
        requestAddressChange => request_address_change
        approveAddressChange => approve_address_change
        endSetupPeriod => end_setup_period
        claimTokens => claim_tokens
        verify_address_change => verify_address_change
        get_claimable_tokens => get_claimable_tokens
        getTokenIdentifier => token_identifier
        getTokenTotalSupply => token_total_supply
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
