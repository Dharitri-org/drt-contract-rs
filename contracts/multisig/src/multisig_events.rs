use crate::{
    action::{ActionFullInfo, GasLimit},
    multisig_state::ActionId,
    user_role::UserRole,
};

dharitri_sc::imports!();

/// Contains all events that can be emitted by the contract.
#[dharitri_sc::module]
pub trait MultisigEventsModule {
    #[event("startPerformAction")]
    fn start_perform_action_event(&self, data: &ActionFullInfo<Self::Api>);

    #[event("performChangeUser")]
    fn perform_change_user_event(
        &self,
        #[indexed] action_id: ActionId,
        #[indexed] changed_user: &ManagedAddress,
        #[indexed] old_role: UserRole,
        #[indexed] new_role: UserRole,
    );

    #[event("performChangeQuorum")]
    fn perform_change_quorum_event(
        &self,
        #[indexed] action_id: ActionId,
        #[indexed] new_quorum: usize,
    );

    #[event("performAsyncCall")]
    fn perform_async_call_event(
        &self,
        #[indexed] action_id: ActionId,
        #[indexed] to: &ManagedAddress,
        #[indexed] moax_value: &BigUint,
        #[indexed] gas: GasLimit,
        #[indexed] endpoint: &ManagedBuffer,
        #[indexed] arguments: &MultiValueManagedVec<ManagedBuffer>,
    );

    #[event("performTransferExecute")]
    fn perform_transfer_execute_event(
        &self,
        #[indexed] action_id: ActionId,
        #[indexed] to: &ManagedAddress,
        #[indexed] moax_value: &BigUint,
        #[indexed] gas: GasLimit,
        #[indexed] endpoint: &ManagedBuffer,
        #[indexed] arguments: &MultiValueManagedVec<ManagedBuffer>,
    );

    #[event("performDeployFromSource")]
    fn perform_deploy_from_source_event(
        &self,
        #[indexed] action_id: ActionId,
        #[indexed] moax_value: &BigUint,
        #[indexed] source_address: &ManagedAddress,
        #[indexed] code_metadata: CodeMetadata,
        #[indexed] gas: GasLimit,
        #[indexed] arguments: &MultiValueManagedVec<ManagedBuffer>,
    );

    #[event("performUpgradeFromSource")]
    fn perform_upgrade_from_source_event(
        &self,
        #[indexed] action_id: ActionId,
        #[indexed] target_address: &ManagedAddress,
        #[indexed] moax_value: &BigUint,
        #[indexed] source_address: &ManagedAddress,
        #[indexed] code_metadata: CodeMetadata,
        #[indexed] gas: GasLimit,
        #[indexed] arguments: &MultiValueManagedVec<ManagedBuffer>,
    );
}
