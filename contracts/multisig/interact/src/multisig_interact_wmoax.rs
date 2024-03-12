use std::time::Duration;

use multisig::action::GasLimit;
use dharitri_sc_scenario::dharitri_sc::types::FunctionCall;
#[allow(unused_imports)]
use dharitri_sc_snippets::dharitri_sc::types::{
    DctTokenPayment, MultiValueEncoded, TokenIdentifier,
};
use dharitri_sc_snippets::{
    dharitri_sc::types::{ContractCall, ContractCallNoPayment},
    dharitri_sc_scenario::{
        denali_system::ScenarioRunner, scenario_format::interpret_trait::InterpretableFrom,
        standalone::retrieve_account_as_scenario_set_state,
    },
};

use super::*;

const WMOAX_SWAP_SC_BECH32: &str = "moa1qqqqqqqqqqqqqpgqcy2wua5cq59y6sxqj2ka3scayh5e5ms7cthq6npfh3";
const WMOAX_TOKEN_IDENTIFIER: &str = "WMOAX-6cf38e";
const WRAP_AMOUNT: u64 = 50000000000000000; // 0.05 MOAX
const UNWRAP_AMOUNT: u64 = 25000000000000000; // 0.025 WMOAX

impl MultisigInteract {
    pub async fn wmoax_swap_full(&mut self) {
        self.deploy().await;
        self.feed_contract_moax().await;
        self.wrap_moax().await;
        self.interactor.sleep(Duration::from_secs(15)).await;
        self.unwrap_moax().await;
    }

    pub async fn wrap_moax(&mut self) {
        println!("proposing wrap moax...");
        let action_id = self.propose_wrap_moax().await;

        println!("perfoming wrap moax action `{action_id}`...");
        self.perform_action(action_id, "15,000,000").await;
    }

    pub async fn unwrap_moax(&mut self) {
        println!("proposing unwrap moax...");
        let action_id = self.propose_unwrap_moax().await;

        println!("perfoming unwrap moax action `{action_id}`...");
        self.perform_action(action_id, "15,000,000").await;
    }

    pub async fn wmoax_swap_set_state(&mut self) {
        let scenario_raw = retrieve_account_as_scenario_set_state(
            Config::load_config().gateway().to_string(),
            WMOAX_SWAP_SC_BECH32.to_string(),
            true,
        )
        .await;

        let scenario = Scenario::interpret_from(scenario_raw, &InterpreterContext::default());

        self.interactor.pre_runners.run_scenario(&scenario);
        self.interactor.post_runners.run_scenario(&scenario);
    }

    async fn propose_wrap_moax(&mut self) -> usize {
        let action_id = self
            .interactor
            .sc_call_get_result(
                ScCallStep::new()
                    .call(self.state.multisig().propose_async_call(
                        bech32::decode(WMOAX_SWAP_SC_BECH32),
                        WRAP_AMOUNT,
                        Option::<GasLimit>::None,
                        FunctionCall::new("wrapMoax"),
                    ))
                    .from(&self.wallet_address)
                    .gas_limit("10,000,000"),
            )
            .await
            .result
            .unwrap();

        println!("successfully proposed wrap moax action `{action_id}`");
        action_id
    }

    async fn propose_unwrap_moax(&mut self) -> usize {
        let contract_call = ContractCallNoPayment::<StaticApi, ()>::new(
            bech32::decode(WMOAX_SWAP_SC_BECH32).into(),
            "unwrapMoax",
        )
        .with_dct_transfer(DctTokenPayment::new(
            TokenIdentifier::from(WMOAX_TOKEN_IDENTIFIER),
            0u64,
            UNWRAP_AMOUNT.into(),
        ))
        .into_normalized();

        let action_id = self
            .interactor
            .sc_call_get_result(
                ScCallStep::new()
                    .call(self.state.multisig().propose_async_call(
                        contract_call.basic.to,
                        0u64,
                        Option::<GasLimit>::None,
                        contract_call.basic.function_call,
                    ))
                    .from(&self.wallet_address)
                    .gas_limit("10,000,000"),
            )
            .await
            .result
            .unwrap();

        println!("successfully proposed unwrap moax action `{action_id}`");
        action_id
    }
}
