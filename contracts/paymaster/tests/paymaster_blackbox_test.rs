use dharitri_sc::{
    codec::{multi_types::MultiValueVec, top_encode_to_vec_u8_or_panic},
    storage::mappers::SingleValue,
    types::{Address, BigUint, MultiValueEncoded},
};
use dharitri_sc_scenario::{
    api::StaticApi,
    scenario_model::{
        Account, AddressValue, CheckAccount, CheckStateStep, ScCallStep, ScDeployStep, ScQueryStep,
        SetStateStep, TxExpect,
    },
    *,
};

use adder::ProxyTrait as _;
use dharitri_wmoax_swap_sc::ProxyTrait as _;
use paymaster::ProxyTrait as _;

const PAYMASTER_ADDRESS_EXPR: &str = "sc:paymaster";
const RELAYER_ADDRESS_EXPR: &str = "address:relayer";
const CALLEE_SC_ADDER_ADDRESS_EXPR: &str = "sc:adder";
const CALLEE_SC_WMOAX_ADDRESS_EXPR: &str = "sc:wmoax";
const PAYMASTER_PATH_EXPR: &str = "file:output/paymaster.wasm";
const ADDER_PATH_EXPR: &str = "file:tests/test-contracts/adder.wasm";
const WMOAX_PATH_EXPR: &str = "file:tests/test-contracts/dharitri-wmoax-swap-sc.wasm.wasm";
const CALLER_ADDRESS_EXPR: &str = "address:caller";
const CALLEE_USER_ADDRESS_EXPR: &str = "address:callee_user";
const OWNER_ADDRESS_EXPR: &str = "address:owner";
const BALANCE: &str = "100,000,000";
const PAYMASTER_TOKEN_ID_EXPR: &str = "str:PAYMSTR-123456";
const WMOAX_TOKEN_ID_EXPR: &str = "str:WMOAX-123456";
const WMOAX_TOKEN_ID: &[u8] = b"WMOAX-123456";
const FEE_TOKEN_ID_EXPR: &str = "str:FEE-123456";
const ADDITIONAL_TOKEN_ID_EXPR: &str = "str:ADDIT-123456";
const FEE_AMOUNT: &str = "20,000";
const INITIAL_ADD_VALUE: u64 = 5;
const ADDITIONAL_ADD_VALUE: u64 = 5;
const UNWRAP_ENDPOINT_NAME: &[u8] = b"unwrap";

type PaymasterContract = ContractInfo<paymaster::Proxy<StaticApi>>;
type AdderContract = ContractInfo<adder::Proxy<StaticApi>>;
type WmoaxContract = ContractInfo<dharitri_wmoax_swap_sc::Proxy<StaticApi>>;


fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/paymaster");

    blockchain.register_contract(PAYMASTER_PATH_EXPR, paymaster::ContractBuilder);
    blockchain.register_contract(ADDER_PATH_EXPR, adder::ContractBuilder);
    blockchain.register_contract(WMOAX_PATH_EXPR, dharitri_wmoax_swap_sc::ContractBuilder);

    blockchain
}

struct PaymasterTestState {
    world: ScenarioWorld,
    callee_user_address: Address,
    paymaster_contract: PaymasterContract,
    relayer_address: Address,
    callee_sc_adder_contract: AdderContract,
    callee_sc_wmoax_address: WmoaxContract,
}

impl PaymasterTestState {
    fn new() -> Self {
        let mut world = world();
        world.start_trace().set_state_step(
            SetStateStep::new()
                .put_account(OWNER_ADDRESS_EXPR, Account::new().nonce(1))
                .put_account(
                    CALLER_ADDRESS_EXPR,
                    Account::new()
                        .nonce(1)
                        .balance(BALANCE)
                        .dct_balance(PAYMASTER_TOKEN_ID_EXPR, BALANCE)
                        .dct_balance(WMOAX_TOKEN_ID_EXPR, BALANCE)
                        .dct_balance(FEE_TOKEN_ID_EXPR, BALANCE)
                        .dct_balance(ADDITIONAL_TOKEN_ID_EXPR, BALANCE),
                )
                .put_account(
                    CALLEE_USER_ADDRESS_EXPR,
                    Account::new().nonce(1).balance(BALANCE),
                )
                .put_account(RELAYER_ADDRESS_EXPR, Account::new().nonce(1).balance(0u32)),
        );

        let callee_user_address = AddressValue::from(CALLEE_USER_ADDRESS_EXPR).to_address();

        let relayer_address = AddressValue::from(RELAYER_ADDRESS_EXPR).to_address();
        let paymaster_contract = PaymasterContract::new(PAYMASTER_ADDRESS_EXPR);
        let callee_sc_adder_contract = AdderContract::new(CALLEE_SC_ADDER_ADDRESS_EXPR);
        let callee_sc_wmoax_address = WmoaxContract::new(CALLEE_SC_WMOAX_ADDRESS_EXPR);

        Self {
            world,
            callee_user_address,
            paymaster_contract,
            relayer_address,
            callee_sc_adder_contract,
            callee_sc_wmoax_address,
        }
    }

    fn deploy_paymaster_contract(&mut self) -> &mut Self {
        let paymaster_code = self.world.code_expression(PAYMASTER_PATH_EXPR);

        self.world
            .set_state_step(SetStateStep::new().new_address(
                OWNER_ADDRESS_EXPR,
                1,
                PAYMASTER_ADDRESS_EXPR,
            ))
            .sc_deploy(
                ScDeployStep::new()
                    .from(OWNER_ADDRESS_EXPR)
                    .code(paymaster_code)
                    .call(self.paymaster_contract.init()),
            );

        self
    }

    fn deploy_adder_contract(&mut self) -> &mut Self {
        let adder_code = self.world.code_expression(ADDER_PATH_EXPR);

        self.world
            .set_state_step(SetStateStep::new().new_address(
                OWNER_ADDRESS_EXPR,
                2,
                CALLEE_SC_ADDER_ADDRESS_EXPR,
            ))
            .sc_deploy(
                ScDeployStep::new()
                    .from(OWNER_ADDRESS_EXPR)
                    .code(adder_code)
                    .call(self.callee_sc_adder_contract.init(INITIAL_ADD_VALUE)),
            );

        self
    }

    fn deploy_wmoax_contract(&mut self) -> &mut Self {
        let wmoax_code = self.world.code_expression(WMOAX_PATH_EXPR);

        self.world
            .set_state_step(SetStateStep::new().new_address(
                OWNER_ADDRESS_EXPR,
                3,
                CALLEE_SC_WMOAX_ADDRESS_EXPR,
            ))
            .sc_deploy(
                ScDeployStep::new()
                    .from(OWNER_ADDRESS_EXPR)
                    .code(wmoax_code)
                    .call(self.callee_sc_wmoax_address.init(WMOAX_TOKEN_ID)),
            );

        self
    }

    fn check_dct_balance(
        &mut self,
        address_expr: &str,
        token_id_expr: &str,
        balance_expr: &str,
    ) -> &mut Self {
        self.world
            .check_state_step(CheckStateStep::new().put_account(
                address_expr,
                CheckAccount::new().dct_balance(token_id_expr, balance_expr),
            ));

        self
    }
    fn check_moax_balance(
        &mut self,
        address_expr: &str,
        balance_expr: &str,
    ) -> &mut Self {
        self.world
            .check_state_step(CheckStateStep::new().put_account(
                address_expr,
                CheckAccount::new().balance(balance_expr),
            ));

        self
    }
}

#[test]
fn test_deploy_paymasters() {
    let mut state = PaymasterTestState::new();
    state.deploy_paymaster_contract();
    state.deploy_adder_contract();
    state.deploy_wmoax_contract();
}

#[test]
fn test_forward_call_no_fee_payment() {
    let mut state = PaymasterTestState::new();
    state.deploy_paymaster_contract();

    state.world.sc_call(
        ScCallStep::new()
            .from(CALLER_ADDRESS_EXPR)
            .call(state.paymaster_contract.forward_execution(
                state.relayer_address.clone(),
                state.callee_user_address.clone(),
                b"add",
                MultiValueVec::<Vec<u8>>::new(),
            ))
            .expect(TxExpect::user_error("str:There is no fee for payment!")),
    );
}

#[test]
fn test_forward_call_user() {
    let mut state = PaymasterTestState::new();
    state.deploy_paymaster_contract();

    state
        .world
        .sc_call(
            ScCallStep::new()
                .from(CALLER_ADDRESS_EXPR)
                .call(state.paymaster_contract.forward_execution(
                    state.relayer_address.clone(),
                    state.callee_user_address.clone(),
                    b"add",
                    MultiValueVec::<Vec<u8>>::new(),
                ))
                .dct_transfer(FEE_TOKEN_ID_EXPR, 0, FEE_AMOUNT),
        )
        .check_state_step(CheckStateStep::new().put_account(
            RELAYER_ADDRESS_EXPR,
            CheckAccount::new().dct_balance(FEE_TOKEN_ID_EXPR, FEE_AMOUNT),
        ));
}

#[test]
fn test_forward_call_sc_adder() {
    let mut state = PaymasterTestState::new();
    state.deploy_paymaster_contract();
    state.deploy_adder_contract();

    state.check_dct_balance(CALLER_ADDRESS_EXPR, FEE_TOKEN_ID_EXPR, BALANCE);
    state.check_dct_balance(CALLER_ADDRESS_EXPR, PAYMASTER_TOKEN_ID_EXPR, BALANCE);

    state.world.sc_call(
        ScCallStep::new()
            .from(CALLER_ADDRESS_EXPR)
            .dct_transfer(FEE_TOKEN_ID_EXPR, 0, FEE_AMOUNT)
            .dct_transfer(PAYMASTER_TOKEN_ID_EXPR, 0, FEE_AMOUNT)
            .call(state.paymaster_contract.forward_execution(
                state.relayer_address.clone(),
                state.callee_sc_adder_contract.to_address(),
                b"add",
                MultiValueVec::from([top_encode_to_vec_u8_or_panic(&ADDITIONAL_ADD_VALUE)]),
            )),
    );

    let expected_adder_sum = INITIAL_ADD_VALUE + ADDITIONAL_ADD_VALUE;
    state.world.sc_query(
        ScQueryStep::new()
            .call(state.callee_sc_adder_contract.sum())
            .expect_value(SingleValue::from(BigUint::from(expected_adder_sum))),
    );
    state.check_dct_balance(RELAYER_ADDRESS_EXPR, FEE_TOKEN_ID_EXPR, FEE_AMOUNT);
    state.check_dct_balance(
        CALLEE_SC_ADDER_ADDRESS_EXPR,
        PAYMASTER_TOKEN_ID_EXPR,
        FEE_AMOUNT,
    );
}

#[test]
fn test_forward_call_sc_adder_multiple_payments() {
    let mut state = PaymasterTestState::new();
    state.deploy_paymaster_contract();
    state.deploy_adder_contract();

    state.check_dct_balance(CALLER_ADDRESS_EXPR, FEE_TOKEN_ID_EXPR, BALANCE);
    state.check_dct_balance(CALLER_ADDRESS_EXPR, PAYMASTER_TOKEN_ID_EXPR, BALANCE);

    state.world.sc_call(
        ScCallStep::new()
            .from(CALLER_ADDRESS_EXPR)
            .dct_transfer(FEE_TOKEN_ID_EXPR, 0, FEE_AMOUNT)
            .dct_transfer(PAYMASTER_TOKEN_ID_EXPR, 0, FEE_AMOUNT)
            .dct_transfer(ADDITIONAL_TOKEN_ID_EXPR, 0, FEE_AMOUNT)
            .call(state.paymaster_contract.forward_execution(
                state.relayer_address.clone(),
                state.callee_sc_adder_contract.to_address(),
                b"add",
                MultiValueVec::from([top_encode_to_vec_u8_or_panic(&ADDITIONAL_ADD_VALUE)]),
            )),
    );

    let expected_adder_sum = INITIAL_ADD_VALUE + ADDITIONAL_ADD_VALUE;
    state.world.sc_query(
        ScQueryStep::new()
            .call(state.callee_sc_adder_contract.sum())
            .expect_value(SingleValue::from(BigUint::from(expected_adder_sum))),
    );
    state.check_dct_balance(RELAYER_ADDRESS_EXPR, FEE_TOKEN_ID_EXPR, FEE_AMOUNT);
    state.check_dct_balance(
        CALLEE_SC_ADDER_ADDRESS_EXPR,
        PAYMASTER_TOKEN_ID_EXPR,
        FEE_AMOUNT,
    );
    state.check_dct_balance(
        CALLEE_SC_ADDER_ADDRESS_EXPR,
        ADDITIONAL_TOKEN_ID_EXPR,
        FEE_AMOUNT,
    );
}

#[test]
fn test_forward_call_wmoax() {
    let mut state = PaymasterTestState::new();
    state.deploy_paymaster_contract();
    state.deploy_adder_contract();

    state.check_dct_balance(CALLER_ADDRESS_EXPR, FEE_TOKEN_ID_EXPR, BALANCE);
    state.check_dct_balance(CALLER_ADDRESS_EXPR, WMOAX_TOKEN_ID_EXPR, BALANCE);

    // Call fails because unwrap amount is 0
    state.world.sc_call(
        ScCallStep::new()
            .from(CALLER_ADDRESS_EXPR)
            .dct_transfer(FEE_TOKEN_ID_EXPR, 0, FEE_AMOUNT)
            .dct_transfer(WMOAX_TOKEN_ID_EXPR, 0, BALANCE)
            .call(state.paymaster_contract.forward_execution(
                state.relayer_address.clone(),
                state.callee_sc_wmoax_address.to_address(),
                UNWRAP_ENDPOINT_NAME,
                MultiValueEncoded::new(),
            ))
    );

    // Fee is kept by the relayer
    let new_fee_amount: &str =  "99980000";
    state.check_dct_balance(RELAYER_ADDRESS_EXPR, FEE_TOKEN_ID_EXPR, FEE_AMOUNT);
    state.check_dct_balance(CALLER_ADDRESS_EXPR, FEE_TOKEN_ID_EXPR, new_fee_amount);

    // Caller has the original balance
    state.check_moax_balance(CALLER_ADDRESS_EXPR, BALANCE);
}


#[test]
fn test_forward_call_fails_wmoax_0_amount() {
    let mut state = PaymasterTestState::new();
    state.deploy_paymaster_contract();
    state.deploy_adder_contract();

    state.check_dct_balance(CALLER_ADDRESS_EXPR, FEE_TOKEN_ID_EXPR, BALANCE);
    state.check_dct_balance(CALLER_ADDRESS_EXPR, WMOAX_TOKEN_ID_EXPR, BALANCE);

    let failling_amount = 0u64;

    // Call fails because unwrap amount is 0
    state.world.sc_call(
        ScCallStep::new()
            .from(CALLER_ADDRESS_EXPR)
            .dct_transfer(FEE_TOKEN_ID_EXPR, 0, FEE_AMOUNT)
            .dct_transfer(WMOAX_TOKEN_ID_EXPR, 0, failling_amount)
            .call(state.paymaster_contract.forward_execution(
                state.relayer_address.clone(),
                state.callee_sc_wmoax_address.to_address(),
                UNWRAP_ENDPOINT_NAME,
                MultiValueEncoded::new(),
            ))
    );

    // Fee is kept by the relayer
    let new_fee_amount: &str =  "99980000";
    state.check_dct_balance(RELAYER_ADDRESS_EXPR, FEE_TOKEN_ID_EXPR, FEE_AMOUNT);
    state.check_dct_balance(CALLER_ADDRESS_EXPR, FEE_TOKEN_ID_EXPR, new_fee_amount);

    // Caller has the original balance
    state.check_dct_balance(CALLER_ADDRESS_EXPR, WMOAX_TOKEN_ID_EXPR, BALANCE);
}
