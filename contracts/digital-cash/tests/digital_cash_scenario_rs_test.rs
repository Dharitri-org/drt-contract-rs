use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/digital-cash");

    blockchain.register_contract(
        "file:output/digital-cash.wasm",
        digital_cash::ContractBuilder,
    );
    blockchain
}

#[test]
fn claim_moax_rs() {
    world().run("scenarios/claim-moax.scen.json");
}

#[test]
fn claim_dct_rs() {
    world().run("scenarios/claim-dct.scen.json");
}

#[test]
fn claim_fees_rs() {
    world().run("scenarios/claim-fees.scen.json");
}

#[test]
fn claim_multi_dct_rs() {
    world().run("scenarios/claim-multi-dct.scen.json");
}

#[test]
fn forward_rs() {
    world().run("scenarios/forward.scen.json");
}

#[test]
fn fund_moax_and_dct_rs() {
    world().run("scenarios/fund-moax-and-dct.scen.json");
}

#[test]
fn set_accounts_rs() {
    world().run("scenarios/set-accounts.scen.json");
}

#[test]
fn withdraw_moax_rs() {
    world().run("scenarios/withdraw-moax.scen.json");
}

#[test]
fn withdraw_dct_rs() {
    world().run("scenarios/withdraw-dct.scen.json");
}

#[test]
fn withdraw_multi_dct_rs() {
    world().run("scenarios/withdraw-multi-dct.scen.json");
}
