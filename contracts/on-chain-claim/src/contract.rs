#![no_std]
#![allow(unused_attributes)]

pub use address_info::AddressInfo;
dharitri_sc::imports!();
dharitri_sc::derive_imports!();

pub mod address_info;
pub mod config;

use crate::config::{MAX_REPAIR_GAP, SFT_AMOUNT};
use dharitri_sc_modules::only_admin;

#[dharitri_sc::contract]
pub trait OnChainClaimContract: config::ConfigModule + only_admin::OnlyAdminModule {
    #[init]
    fn init(&self, repair_streak_token_id: TokenIdentifier) {
        self.internal_set_repair_streak_token_id(repair_streak_token_id);

        let caller = self.blockchain().get_caller();
        self.add_admin(caller);
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint(claim)]
    fn claim(&self) {
        let caller = self.blockchain().get_caller();
        require!(
            !self.blockchain().is_smart_contract(&caller),
            "Only user accounts can perform claim"
        );
        self.require_same_shard(&caller);

        let current_epoch = self.blockchain().get_block_epoch();

        let address_info_mapper = self.address_info(&caller);
        if address_info_mapper.is_empty() {
            let address_info = AddressInfo::new_with_epoch(current_epoch);
            self.address_info(&caller).set(address_info);
            return;
        }

        address_info_mapper.update(|address_info| {
            require!(
                address_info.last_epoch_claimed < current_epoch,
                "epoch already claimed"
            );

            if address_info.last_epoch_claimed + 1 == current_epoch {
                address_info.current_streak += 1;
            } else {
                address_info.current_streak = 1;
            }

            address_info.total_epochs_claimed += 1;
            address_info.last_epoch_claimed = current_epoch;

            if address_info.best_streak < address_info.current_streak {
                address_info.best_streak = address_info.current_streak;
            }
        });
    }

    #[payable("*")]
    #[endpoint(claimAndRepair)]
    fn claim_and_repair(&self) {
        let caller = self.blockchain().get_caller();
        require!(
            !self.blockchain().is_smart_contract(&caller),
            "Only user accounts can perform claim and repair"
        );
        self.require_same_shard(&caller);

        let payment = self.call_value().single_dct();
        let repair_streak_token_identifier = self.repair_streak_token_identifier().get();
        require!(
            payment.token_identifier == repair_streak_token_identifier,
            "Bad payment token"
        );
        require!(payment.amount == SFT_AMOUNT, "Bad payment amount");

        let current_epoch = self.blockchain().get_block_epoch();

        let address_info_mapper = self.address_info(&caller);

        require!(
            !address_info_mapper.is_empty(),
            "can't repair streak for address"
        );

        address_info_mapper.update(|address_info| {
            let missed_epochs =
                self.get_missed_epochs(current_epoch, address_info.last_epoch_claimed);

            require!(
                missed_epochs > 0 && missed_epochs <= MAX_REPAIR_GAP,
                "can't repair streak for current epoch"
            );

            address_info.current_streak += missed_epochs + 1;
            address_info.total_epochs_claimed += missed_epochs + 1;
            address_info.last_epoch_claimed = current_epoch;
            if address_info.best_streak < address_info.current_streak {
                address_info.best_streak = address_info.current_streak;
            }
        });

        self.send().dct_local_burn(
            &payment.token_identifier,
            payment.token_nonce,
            &payment.amount,
        );
    }

    #[endpoint(updateState)]
    fn update_state(
        &self,
        address: &ManagedAddress,
        current_streak: u64,
        last_epoch_claimed: u64,
        total_epochs_claimed: u64,
        best_streak: u64,
    ) {
        self.require_caller_is_admin();
        self.require_same_shard(address);

        let address_info = AddressInfo::new(
            current_streak,
            last_epoch_claimed,
            total_epochs_claimed,
            best_streak,
        );
        self.address_info(address).set(address_info);
    }

    #[endpoint(setRepairStreakTokenId)]
    fn set_repair_streak_token_id(&self, repair_streak_token_id: TokenIdentifier) {
        self.require_caller_is_admin();

        self.internal_set_repair_streak_token_id(repair_streak_token_id);
    }

    fn internal_set_repair_streak_token_id(&self, repair_streak_token_id: TokenIdentifier) {
        require!(
            repair_streak_token_id.is_valid_dct_identifier(),
            "Invalid token ID"
        );
        self.repair_streak_token_identifier()
            .set(repair_streak_token_id);
    }
}
