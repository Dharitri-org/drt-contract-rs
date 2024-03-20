#![no_std]

dharitri_sc::imports!();

#[dharitri_sc::contract]
pub trait MoaxDctSwap: dharitri_sc_modules::pause::PauseModule {
    #[init]
    fn init(&self, wrapped_moax_token_id: TokenIdentifier) {
        self.wrapped_moax_token_id().set(&wrapped_moax_token_id);
    }

    // endpoints

    #[payable("MOAX")]
    #[endpoint(wrapMoax)]
    fn wrap_moax(&self) -> DctTokenPayment<Self::Api> {
        self.require_not_paused();

        let payment_amount = self.call_value().moax_value();
        require!(*payment_amount > 0u32, "Payment must be more than 0");

        let wrapped_moax_token_id = self.wrapped_moax_token_id().get();
        self.send()
            .dct_local_mint(&wrapped_moax_token_id, 0, &payment_amount);

        let caller = self.blockchain().get_caller();
        self.send()
            .direct_dct(&caller, &wrapped_moax_token_id, 0, &payment_amount);

        DctTokenPayment::new(wrapped_moax_token_id, 0, payment_amount.clone_value())
    }

    #[payable("*")]
    #[endpoint(unwrapMoax)]
    fn unwrap_moax(&self) {
        self.require_not_paused();

        let (payment_token, payment_amount) = self.call_value().single_fungible_dct();
        let wrapped_moax_token_id = self.wrapped_moax_token_id().get();

        require!(payment_token == wrapped_moax_token_id, "Wrong dct token");
        require!(payment_amount > 0u32, "Must pay more than 0 tokens!");
        require!(
            payment_amount <= self.get_locked_moax_balance(),
            "Contract does not have enough funds"
        );

        self.send()
            .dct_local_burn(&wrapped_moax_token_id, 0, &payment_amount);

        // 1 wrapped MOAX = 1 MOAX, so we pay back the same amount
        let caller = self.blockchain().get_caller();
        self.send().direct_moax(&caller, &payment_amount);
    }

    #[view(getLockedMoaxBalance)]
    fn get_locked_moax_balance(&self) -> BigUint {
        self.blockchain()
            .get_sc_balance(&MoaxOrDctTokenIdentifier::moax(), 0)
    }

    #[view(getWrappedMoaxTokenId)]
    #[storage_mapper("wrappedMoaxTokenId")]
    fn wrapped_moax_token_id(&self) -> SingleValueMapper<TokenIdentifier>;
}
