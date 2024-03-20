// Code generated by the dharitri-sc build system. DO NOT EDIT.

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

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    liquid_locking
    (
        init => init
        upgrade => upgrade
        set_unbond_period => set_unbond_period
        whitelist_token => whitelist_token
        blacklist_token => blacklist_token
        lock => lock
        unlock => unlock
        unbond => unbond
        lockedTokenAmounts => locked_token_amounts_by_address
        unlockedTokenAmounts => unlocked_token_by_address
        lockedTokens => locked_tokens
        unlockedTokens => unlocked_tokens
        whitelistedTokens => token_whitelist
        unbondPeriod => unbond_period
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
