use crate::{common::Percentage, exchange_actions::EndpointInfo};

dharitri_sc::imports!();
dharitri_sc::derive_imports!();

mod pair_proxy {
    dharitri_sc::imports!();

    #[dharitri_sc::proxy]
    pub trait PairProxy {
        #[payable("*")]
        #[endpoint(swapTokensFixedInput)]
        fn swap_tokens_fixed_input(
            &self,
            token_out: TokenIdentifier,
            amount_out_min: BigUint,
        ) -> DctTokenPayment;
    }
}

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct InitialLaunchBlocks {
    pub start: u64,
    pub end: u64,
}

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct InitialLaunchInfo<M: ManagedTypeApi> {
    pub account_buy_limit: BigUint<M>,
    pub tx_buy_limit: BigUint<M>,
    pub buy_fee_percentage_start: Percentage,
    pub buy_fee_percentage_end: Percentage,
    pub sell_fee_percentage_start: Percentage,
    pub sell_fee_percentage_end: Percentage,
}

#[dharitri_sc::module]
pub trait InitialLaunchModule:
    crate::common::CommonModule
    + crate::token_info::TokenInfoModule
    + dharitri_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + dharitri_sc_modules::pause::PauseModule
{
    #[payable("*")]
    #[endpoint(buyToken)]
    fn buy_token(
        &self,
        pair_adddress: ManagedAddress,
        amount_out_min: BigUint,
    ) -> DctTokenPayment {
        self.require_not_paused();
        self.require_initial_launch();
        require!(
            !self.known_contracts(&pair_adddress).is_empty(),
            "Unknown pair"
        );

        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_dct();
        let launch_info = self.initial_launch_info().get();
        let fee_percentage = self.get_fee_percentage(
            launch_info.buy_fee_percentage_start,
            launch_info.buy_fee_percentage_end,
        );
        let take_fee_result = self.take_fees(
            caller,
            ManagedVec::from_single_item(payment),
            ManagedVec::from_single_item(fee_percentage),
        );
        require!(
            !take_fee_result.transfers.is_empty(),
            "Payment amount too small to cover fees"
        );

        let out_token_id = self.get_token_id();
        let (_, all_transfers): (DctTokenPayment, _) = self
            .pair_proxy(pair_adddress)
            .swap_tokens_fixed_input(out_token_id, amount_out_min)
            .with_dct_transfer(take_fee_result.transfers.get(0))
            .execute_on_dest_context_with_back_transfers();
        let received_tokens = all_transfers.dct_payments.get(0);

        require!(
            received_tokens.amount <= launch_info.tx_buy_limit,
            "Exceeded tx limit"
        );
        self.total_bought(&take_fee_result.original_caller)
            .update(|total_bought| {
                *total_bought += &received_tokens.amount;

                require!(
                    *total_bought <= launch_info.account_buy_limit,
                    "Total buy amount exceeded"
                );
            });

        let fees = take_fee_result.fees.get(0);
        self.burn_tokens(&fees);

        self.send().direct_dct(
            &take_fee_result.original_caller,
            &received_tokens.token_identifier,
            received_tokens.token_nonce,
            &received_tokens.amount,
        );

        received_tokens
    }

    #[payable("*")]
    #[endpoint(sellToken)]
    fn sell_token(
        &self,
        pair_adddress: ManagedAddress,
        out_token_id: TokenIdentifier,
        amount_out_min: BigUint,
    ) -> DctTokenPayment {
        self.require_not_paused();
        self.require_initial_launch();
        require!(
            !self.known_contracts(&pair_adddress).is_empty(),
            "Unknown pair"
        );

        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_dct();
        let launch_info = self.initial_launch_info().get();
        let fee_percentage = self.get_fee_percentage(
            launch_info.sell_fee_percentage_start,
            launch_info.sell_fee_percentage_end,
        );
        let take_fee_result = self.take_fees(
            caller,
            ManagedVec::from_single_item(payment),
            ManagedVec::from_single_item(fee_percentage),
        );
        require!(
            !take_fee_result.transfers.is_empty(),
            "Payment amount too small to cover fees"
        );

        let (_, all_transfers): (DctTokenPayment, _) = self
            .pair_proxy(pair_adddress)
            .swap_tokens_fixed_input(out_token_id, amount_out_min)
            .with_dct_transfer(take_fee_result.transfers.get(0))
            .execute_on_dest_context_with_back_transfers();
        let received_tokens = all_transfers.dct_payments.get(0);

        let fees = take_fee_result.fees.get(0);
        self.burn_tokens(&fees);

        self.send().direct_dct(
            &take_fee_result.original_caller,
            &received_tokens.token_identifier,
            received_tokens.token_nonce,
            &received_tokens.amount,
        );

        received_tokens
    }

    fn get_fee_percentage(
        &self,
        fee_percentage_start: Percentage,
        fee_percentage_end: Percentage,
    ) -> Percentage {
        let initial_launch_blocks = self.initial_launch_blocks().get();
        let current_block = self.blockchain().get_block_nonce();
        require!(
            current_block <= initial_launch_blocks.end,
            "Invalid buy/sell block"
        );

        let blocks_passed_in_penalty_phase = current_block - initial_launch_blocks.start;
        let blocks_diff = initial_launch_blocks.end - initial_launch_blocks.start;
        let percentage_diff = fee_percentage_start - fee_percentage_end;

        let penalty_percentage_decrease =
            percentage_diff as u64 * blocks_passed_in_penalty_phase / (blocks_diff - 1);

        fee_percentage_start - penalty_percentage_decrease as u32
    }

    fn burn_tokens(&self, tokens: &DctTokenPayment) {
        let token_roles = self
            .blockchain()
            .get_dct_local_roles(&tokens.token_identifier);
        if token_roles.has_role(&DctLocalRole::Burn) {
            self.send().dct_local_burn(
                &tokens.token_identifier,
                tokens.token_nonce,
                &tokens.amount,
            );
        }
    }

    fn require_not_initial_launch(&self) {
        let current_block = self.blockchain().get_block_nonce();
        let initial_launch_blocks = self.initial_launch_blocks().get();
        require!(
            current_block > initial_launch_blocks.end,
            "Cannot call this endpoint during initial launch"
        );
    }

    fn require_initial_launch(&self) {
        let current_block = self.blockchain().get_block_nonce();
        let initial_launch_blocks = self.initial_launch_blocks().get();
        require!(
            current_block <= initial_launch_blocks.end,
            "Cannot call this endpoint, initial launch period passed"
        );
    }

    #[storage_mapper("initialLaunchBlocks")]
    fn initial_launch_blocks(&self) -> SingleValueMapper<InitialLaunchBlocks>;

    #[storage_mapper("initialLaunchInfo")]
    fn initial_launch_info(&self) -> SingleValueMapper<InitialLaunchInfo<Self::Api>>;

    #[storage_mapper("totalBought")]
    fn total_bought(&self, user_addr: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[storage_mapper("knownContracts")]
    fn known_contracts(
        &self,
        sc_addr: &ManagedAddress,
    ) -> SingleValueMapper<ManagedVec<EndpointInfo<Self::Api>>>;

    #[proxy]
    fn pair_proxy(&self, to: ManagedAddress) -> pair_proxy::Proxy<Self::Api>;
}
