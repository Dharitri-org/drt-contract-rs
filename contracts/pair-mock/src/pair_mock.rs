#![no_std]

dharitri_sc::imports!();

#[dharitri_sc::contract]
pub trait PairMock {
    #[init]
    fn init(&self, first_token_id: TokenIdentifier, second_token_id: TokenIdentifier) {
        self.first_token_id().set(first_token_id);
        self.second_token_id().set(second_token_id);
    }

    #[payable("*")]
    #[endpoint(swapTokensFixedInput)]
    fn swap_tokens_fixed_input(
        &self,
        _token_out: TokenIdentifier,
        _amount_out_min: BigUint,
    ) -> DctTokenPayment {
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_dct();
        let first_token_id = self.first_token_id().get();
        let second_token_id = self.second_token_id().get();
        let output = if payment.token_identifier == first_token_id {
            DctTokenPayment::new(second_token_id, 0, payment.amount * 2u32)
        } else {
            DctTokenPayment::new(first_token_id, 0, payment.amount / 2u32)
        };

        self.send()
            .direct_dct(&caller, &output.token_identifier, 0, &output.amount);

        output
    }

    #[storage_mapper("firstTokenId")]
    fn first_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("secondTokenId")]
    fn second_token_id(&self) -> SingleValueMapper<TokenIdentifier>;
}
