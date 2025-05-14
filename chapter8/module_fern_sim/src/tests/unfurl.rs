use chapter8::module_ferm_sim::fern_sim::{run_simulation, Fern};

#[test]
fn test_fiddlehed_unfurling() {
    let mut world = Terrarium::load("tests/unfurl_files/fiddlehead.tm");
    assert!(world.fernn(0).is_furled());
    let one_hour = Duration::from_secs(3600);
    assert!(world.fern(0).is_fully_unfurled())
}
