// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           12
// Async Callback (empty):               1
// Total number of exported functions:  14

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!(static64k);
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    erc1155_marketplace
    (
        init => init
        onERC1155Received => on_erc1155_received
        onERC1155BatchReceived => on_erc1155_batch_received
        claim => claim
        setCutPercentage => set_percentage_cut_endpoint
        setTokenOwnershipContractAddress => set_token_ownership_contract_address_endpoint
        bid => bid
        endAuction => end_auction
        isUpForAuction => is_up_for_auction
        getAuctionStatus => get_auction_status
        getCurrentWinningBid => get_current_winning_bid
        getCurrentWinner => get_current_winner
        getPercentageCut => percentage_cut
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
