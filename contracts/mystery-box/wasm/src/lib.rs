// Code generated by the dharitri-sc build system. DO NOT EDIT.

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

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    mystery_box
    (
        init => init
        setupMysteryBox => setup_mystery_box
        updateMysteryBoxUris => update_mystery_box_uris
        createMysteryBox => create_mystery_box
        openMysteryBox => open_mystery_box
        getMysteryBoxTokenIdentifier => mystery_box_token_id
        getGlobalCooldownEpoch => global_cooldown_epoch
        getWinningRates => winning_rates
        getMysteryBoxUris => mystery_box_uris
        isAdmin => is_admin
        addAdmin => add_admin
        removeAdmin => remove_admin
        getAdmins => admins
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
