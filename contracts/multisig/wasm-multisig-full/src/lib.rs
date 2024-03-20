// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           36
// Async Callback:                       1
// Total number of exported functions:  38

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    multisig
    (
        init => init
        upgrade => upgrade
        deposit => deposit
        discardAction => discard_action_endpoint
        discardBatch => discard_batch
        getQuorum => quorum
        getNumBoardMembers => num_board_members
        getNumProposers => num_proposers
        getActionGroup => action_groups
        getActionLastIndex => get_action_last_index
        proposeAddBoardMember => propose_add_board_member
        proposeAddProposer => propose_add_proposer
        proposeRemoveUser => propose_remove_user
        proposeChangeQuorum => propose_change_quorum
        proposeTransferExecute => propose_transfer_execute
        proposeAsyncCall => propose_async_call
        proposeSCDeployFromSource => propose_sc_deploy_from_source
        proposeSCUpgradeFromSource => propose_sc_upgrade_from_source
        sign => sign
        signBatch => sign_batch
        signAndPerform => sign_and_perform
        signBatchAndPerform => sign_batch_and_perform
        unsign => unsign
        unsignBatch => unsign_batch
        signed => signed
        quorumReached => quorum_reached
        performAction => perform_action_endpoint
        performBatch => perform_batch
        dnsRegister => dns_register
        getPendingActionFullInfo => get_pending_action_full_info
        userRole => user_role
        getAllBoardMembers => get_all_board_members
        getAllProposers => get_all_proposers
        getActionData => get_action_data
        getActionSigners => get_action_signers
        getActionSignerCount => get_action_signer_count
        getActionValidSignerCount => get_action_valid_signer_count
    )
}

dharitri_sc_wasm_adapter::async_callback! { multisig }
