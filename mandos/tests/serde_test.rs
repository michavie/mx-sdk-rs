extern crate mandos;

use std::{fs, fs::File, io::Write};

use mandos::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    model::Scenario,
    serde_raw::ScenarioRaw,
};

#[test]
fn test_scenario_low_level_ser_de() {
    let contents = fs::read_to_string("./example.scen.json").unwrap();
    let scenario_raw = ScenarioRaw::from_json_str(contents.as_str());

    let serialized = scenario_raw.to_json_string();

    let mut file = File::create("serialized_raw.scen.json").unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
    assert_eq!(serialized, contents);
}

#[test]
fn test_scenario_high_level_ser_de() {
    let contents = fs::read_to_string("./example.scen.json").unwrap();
    let scenario_raw = ScenarioRaw::from_json_str(contents.as_str());
    let scenario = Scenario::interpret_from(scenario_raw, &InterpreterContext::default());

    let scenario_raw_re = scenario.into_raw();
    let serialized = scenario_raw_re.to_json_string();

    let mut file = File::create("serialized.scen.json").unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
    // assert_eq!(serialized, contents);
}
