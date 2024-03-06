use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn claim_moax_go() {
    world().run("scenarios/claim-moax.scen.json");
}

#[test]
fn claim_dct_go() {
    world().run("scenarios/claim-dct.scen.json");
}

#[test]
fn claim_fees_go() {
    world().run("scenarios/claim-fees.scen.json");
}

#[test]
fn claim_multi_dct_go() {
    world().run("scenarios/claim-multi-dct.scen.json");
}

#[test]
fn forward_go() {
    world().run("scenarios/forward.scen.json");
}

#[test]
fn fund_moax_and_dct_go() {
    world().run("scenarios/fund-moax-and-dct.scen.json");
}

#[test]
fn set_accounts_go() {
    world().run("scenarios/set-accounts.scen.json");
}

#[test]
fn withdraw_moax_go() {
    world().run("scenarios/withdraw-moax.scen.json");
}

#[test]
fn withdraw_dct_go() {
    world().run("scenarios/withdraw-dct.scen.json");
}

#[test]
fn withdraw_multi_dct_go() {
    world().run("scenarios/withdraw-multi-dct.scen.json");
}
