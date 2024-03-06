use dharitri_sc_modules::transfer_role_proxy::PaymentsVec;

use crate::{
    action::{Action, CallActionData, GasLimit},
    multisig_state::{ActionId, GroupId},
};

dharitri_sc::imports!();

/// Contains all events that can be emitted by the contract.
#[dharitri_sc::module]
pub trait MultisigProposeModule: crate::multisig_state::MultisigStateModule {
    fn propose_action(&self, action: Action<Self::Api>) -> ActionId {
        let (caller_id, caller_role) = self.get_caller_id_and_role();
        require!(
            caller_role.can_propose(),
            "only board members and proposers can propose"
        );

        let action_id = self.action_mapper().push(&action);
        if caller_role.can_sign() {
            // also sign
            // since the action is newly created, the caller can be the only signer
            let _ = self.action_signer_ids(action_id).insert(caller_id);
        }

        action_id
    }

    /// Initiates board member addition process.
    /// Can also be used to promote a proposer to board member.
    #[endpoint(proposeAddBoardMember)]
    fn propose_add_board_member(&self, board_member_address: ManagedAddress) -> ActionId {
        self.propose_action(Action::AddBoardMember(board_member_address))
    }

    /// Initiates proposer addition process..
    /// Can also be used to demote a board member to proposer.
    #[endpoint(proposeAddProposer)]
    fn propose_add_proposer(&self, proposer_address: ManagedAddress) -> ActionId {
        self.propose_action(Action::AddProposer(proposer_address))
    }

    /// Removes user regardless of whether it is a board member or proposer.
    #[endpoint(proposeRemoveUser)]
    fn propose_remove_user(&self, user_address: ManagedAddress) -> ActionId {
        self.propose_action(Action::RemoveUser(user_address))
    }

    #[endpoint(proposeChangeQuorum)]
    fn propose_change_quorum(&self, new_quorum: usize) -> ActionId {
        self.propose_action(Action::ChangeQuorum(new_quorum))
    }

    /// Propose a transaction in which the contract will perform a transfer-execute call.
    /// Can send MOAX without calling anything.
    /// Can call smart contract endpoints directly.
    /// Doesn't really work with builtin functions.
    #[endpoint(proposeTransferExecute)]
    fn propose_transfer_execute(
        &self,
        to: ManagedAddress,
        moax_amount: BigUint,
        opt_gas_limit: Option<GasLimit>,
        function_call: FunctionCall,
    ) -> ActionId {
        require!(
            moax_amount > 0 || !function_call.is_empty(),
            "proposed action has no effect"
        );

        let call_data = CallActionData {
            to,
            moax_amount,
            opt_gas_limit,
            endpoint_name: function_call.function_name,
            arguments: function_call.arg_buffer.into_vec_of_buffers(),
        };

        self.propose_action(Action::SendTransferExecuteMoax(call_data))
    }

    #[endpoint(proposeTransferExecuteDct)]
    fn propose_transfer_execute_dct(
        &self,
        to: ManagedAddress,
        tokens: PaymentsVec<Self::Api>,
        opt_gas_limit: Option<GasLimit>,
        function_call: FunctionCall,
    ) -> ActionId {
        require!(!tokens.is_empty(), "No tokens to transfer");

        self.propose_action(Action::SendTransferExecuteDct {
            to,
            tokens,
            opt_gas_limit,
            endpoint_name: function_call.function_name,
            arguments: function_call.arg_buffer.into_vec_of_buffers(),
        })
    }

    /// Propose a transaction in which the contract will perform an async call call.
    /// Can call smart contract endpoints directly.
    /// Can use DCTTransfer/DCTNFTTransfer/MultiDCTTransfer to send tokens, while also optionally calling endpoints.
    /// Works well with builtin functions.
    /// Cannot simply send MOAX directly without calling anything.
    #[endpoint(proposeAsyncCall)]
    fn propose_async_call(
        &self,
        to: ManagedAddress,
        moax_amount: BigUint,
        opt_gas_limit: Option<GasLimit>,
        function_call: FunctionCall,
    ) -> ActionId {
        require!(
            moax_amount > 0 || !function_call.is_empty(),
            "proposed action has no effect"
        );

        let call_data = CallActionData {
            to,
            moax_amount,
            opt_gas_limit,
            endpoint_name: function_call.function_name,
            arguments: function_call.arg_buffer.into_vec_of_buffers(),
        };

        self.propose_action(Action::SendAsyncCall(call_data))
    }

    #[endpoint(proposeSCDeployFromSource)]
    fn propose_sc_deploy_from_source(
        &self,
        amount: BigUint,
        source: ManagedAddress,
        code_metadata: CodeMetadata,
        arguments: MultiValueEncoded<ManagedBuffer>,
    ) -> ActionId {
        self.propose_action(Action::SCDeployFromSource {
            amount,
            source,
            code_metadata,
            arguments: arguments.into_vec_of_buffers(),
        })
    }

    #[endpoint(proposeSCUpgradeFromSource)]
    fn propose_sc_upgrade_from_source(
        &self,
        sc_address: ManagedAddress,
        amount: BigUint,
        source: ManagedAddress,
        code_metadata: CodeMetadata,
        arguments: MultiValueEncoded<ManagedBuffer>,
    ) -> ActionId {
        self.propose_action(Action::SCUpgradeFromSource {
            sc_address,
            amount,
            source,
            code_metadata,
            arguments: arguments.into_vec_of_buffers(),
        })
    }

    #[endpoint(proposeBatch)]
    fn propose_batch(&self, group_id: GroupId, actions: MultiValueEncoded<Action<Self::Api>>) {
        require!(group_id != 0, "May not use group ID 0");
        require!(!actions.is_empty(), "No actions");

        let (caller_id, caller_role) = self.get_caller_id_and_role();
        require!(
            caller_role.can_propose(),
            "only board members and proposers can propose"
        );

        let own_sc_address = self.blockchain().get_sc_address();
        let own_shard = self.blockchain().get_shard_of_address(&own_sc_address);

        let mut action_mapper = self.action_mapper();
        let mut action_groups_mapper = self.action_groups(group_id);
        for action in actions {
            require!(
                !action.is_nothing() && !action.is_async_call(),
                "Invalid action"
            );

            if let Action::SendTransferExecuteMoax(call_data) = &action {
                let other_sc_shard = self.blockchain().get_shard_of_address(&call_data.to);
                require!(
                    own_shard == other_sc_shard,
                    "All transfer exec must be to the same shard"
                );
            }

            let action_id = action_mapper.push(&action);
            if caller_role.can_sign() {
                let _ = self.action_signer_ids(action_id).insert(caller_id);
            }

            let _ = action_groups_mapper.insert(action_id);
            self.group_for_action(action_id).set(group_id);
        }
    }
}
