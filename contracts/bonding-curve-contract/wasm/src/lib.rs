// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           10
// Async Callback (empty):               1
// Total number of exported functions:  12

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    bonding_curve_contract
    (
        init => init
        sellToken => sell_token_endpoint
        buyToken => buy_token_endpoint
        deposit => deposit_endpoint
        setBondingCurve => set_bonding_curve_endpoint
        claim => claim_endpoint
        view_buy_price => view_buy_price
        view_sell_price => view_sell_price
        getTokenAvailability => get_token_availability
        setLocalRoles => set_local_roles
        unsetLocalRoles => unset_local_roles
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
