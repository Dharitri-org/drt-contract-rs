#![no_std]

dharitri_sc::imports!();

use dharitri_sc_modules::pause;

pub mod config;
pub mod contract_interactions;

#[dharitri_sc::contract]
pub trait ProxyDeployer:
    contract_interactions::ContractInteractionsModule + config::ConfigModule + pause::PauseModule
{
    #[init]
    fn init(&self, default_gas_for_save: u64) {
        self.default_gas_for_save_operation()
            .set(default_gas_for_save);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
