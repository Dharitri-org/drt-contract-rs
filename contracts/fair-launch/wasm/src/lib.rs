// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           16
// Async Callback:                       1
// Total number of exported functions:  18

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    fair_launch
    (
        init => init
        upgrade => upgrade
        getTokenFees => token_fees
        addExchangeEndpoint => add_exchange_endpoint
        removeExchangeEndpoint => remove_exchange_endpoint
        forwardExecuteOnDest => forward_execute_on_dest
        buyToken => buy_token
        sellToken => sell_token
        issueToken => issue_token
        setTransferRole => set_transfer_role
        setTokenFees => set_token_fees
        addUsersToWhitelist => add_users_to_whitelist
        removeUsersFromWhitelist => remove_users_from_whitelist
        forwardTransfer => forward_transfer
        pause => pause_endpoint
        unpause => unpause_endpoint
        isPaused => paused_status
    )
}

dharitri_sc_wasm_adapter::async_callback! { fair_launch }