use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn unwrap_moax_go() {
    world().run("scenarios/unwrap_moax.scen.json");
}

#[test]
fn wrap_moax_go() {
    world().run("scenarios/wrap_moax.scen.json");
}
