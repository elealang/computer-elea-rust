//!
//! Test | Filesystem Representations
//!

use std::path::PathBuf;

use elea::define::experience::representation::rust_pragmatic::{
    Arrow, Machine, State, Space
};
use elea::define::experience::representation::filesystem;

use crate::common;


#[test]
fn test_format_nested_machines() {

    let machine_default = Machine::new("default", vec![
       State::new("on", vec![
          Arrow::new("self", "default/on"),
          Arrow::new("off", "default/off"),
          Arrow::new("pattern", "default/pattern"),
       ]),
       State::new("off", vec![
          Arrow::new("on", "default/on"),
          Arrow::new("self", "default/off"),
       ]),
       State::new("pattern", vec![
          Arrow::new("fade", "pattern/fade/levels"),
          Arrow::new("strobe", "pattern/strobe/speeds"),
       ]),
    ]);
    let machine_pattern =  Machine::new("pattern", vec![
       State::new("fade/levels", vec![
          Arrow::new("20_percent", "default/on"),
          Arrow::new("40_percent", "default/on"),
          Arrow::new("60_percent", "default/on"),
          Arrow::new("80_percent", "default/on"),
          Arrow::new("100_percent", "default/on"),
       ]),
       State::new("strobe/speeds", vec![
          Arrow::new("slow", "default/on"),
          Arrow::new("medium", "default/on"),
          Arrow::new("fast", "default/on"),
       ]),
    ]);
    let machine_broken = Machine::new("broken", vec![
       State::new("fix", vec![
          Arrow::new("power", "broken/fix/power"),
          Arrow::new("light", "broken/fix/light"),
          Arrow::new("structure", "broken/fix/structure"),
       ]),
       State::new("fix/power", vec![
          Arrow::new("battery", "broken/fix/power/battery"),
          Arrow::new("miscellaneous", "broken/fix/power/miscellaneous"),
       ]),
       State::new("fix/power/battery", vec![
          Arrow::new("replace", "broken/fix"),
          Arrow::new("charge", "broken/fix"),
          Arrow::new("plug_in", "broken/fix"),
       ]),
       State::new("fix/power/miscellaneous", vec![
          Arrow::new("check_outlet_power", "broken/fix"),
          Arrow::new("check_cord", "broken/fix"),
       ]),
       State::new("fix/light", vec![
          Arrow::new("change_bulb", "broken/fix"),
       ]),
       State::new("fix/structure", vec![
          Arrow::new("cord", "broken/fix"),
          Arrow::new("casing", "broken/fix"),
       ]),
    ]);
    let mut target_machines = vec![machine_default, machine_broken, machine_pattern]; 
    target_machines.sort_by(|m1,m2| m1.id.cmp(&m2.id));

    // todo verify state paths

    let machines_path: PathBuf = common::asset_path("representation/filesystem/machines/nested-machines-lightswitch");
    let actual_machines_res = filesystem::to_machines(machines_path.as_path(), filesystem::Format::NestedMachines);

    if actual_machines_res.is_err() {
        println!("{:?}", actual_machines_res);
        assert!(false);
    }
    let mut actual_machines = actual_machines_res.unwrap();
    actual_machines.sort_by(|m1,m2| m1.id.cmp(&m2.id));

    // Uncomment to help debugging
    //write_machine_files(&Space::new(target_machines), &Space::new(actual_machines.unwrap()));
    
    assert_eq!(target_machines, actual_machines);
}

/// Useful for debugging failed tests
fn write_machine_files(space1: &Space, space2: &Space) {
    let f_space1 = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("space1.yml")
        .expect("Couldn't open file");
    let f_space2 = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("space2.yml")
        .expect("Couldn't open file");
    serde_yaml::to_writer(f_space1, &space1).unwrap();
    serde_yaml::to_writer(f_space2, &space2).unwrap();
}


