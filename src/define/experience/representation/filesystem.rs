//! ! Elea Filesystem Representation Parser for Rust
//!
//! Description
//!
//! ----------------------------------------------------------------------------
//! type: computer for EleaFilesystem
//! ----------------------------------------------------------------------------

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::ffi::OsStr;
use std::fmt;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::define::experience::representation::rust_pragmatic as elea;

//------------------------------------------------------------------------------
// PARSERS

pub enum Format {
    NestedMachines,
}

//------------------------------------------------------------------------------
// PARSERS / Types
//

#[derive(Clone, Debug, Deserialize, Serialize)]
struct StateTree {
    id: String,
    #[serde(default = "Vec::new")]
    tree: Vec<StateTree>,
    description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum StateFileFormat {
    Unknown,
    State,
    Tree,
    StateList,
}

impl fmt::Display for StateFileFormat {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match &self {
            StateFileFormat::Unknown => write!(f, "{}", "Unknown"),
            StateFileFormat::State => write!(f, "{}", "State"),
            StateFileFormat::Tree => write!(f, "{}", "Tree"),
            StateFileFormat::StateList => write!(f, "{}", "StateList"),
        }
    }
}


/// Check the top-level keys of the YAML file to determine the format
fn yaml_state_file_format(file_path: &Path) -> Result<StateFileFormat, StateYAMLFileParseError> {

    let file = state_yaml_file(file_path)?;
    let file_value: serde_yaml::Value = serde_yaml::from_reader(file).map_err(|err| {
        StateYAMLFileParseError::YAML(StateYAMLFileParseYAMLError {
            file_path: file_path.display().to_string(),
            format: StateFileFormat::Unknown,
            error: err.to_string(),
        })
    })?;

    let mut format: StateFileFormat = StateFileFormat::Unknown;
    if let serde_yaml::Value::Mapping(mapping) = file_value {
        for (key, _) in mapping {
            if let serde_yaml::Value::String(key_string) = key {
                match key_string.as_str() {
                    "arrows" => format = StateFileFormat::State,
                    "tree" => format = StateFileFormat::Tree,
                    "states" => format = StateFileFormat::StateList,
                    _        => {},
                }
            }
        }
    }

    return Ok(format);
}


// PARSERS / Types / Errors
//------------------------------------------------------------------------------

#[derive(Debug, Deserialize, Serialize)]
pub enum MachinesError {
    NestedMachines(FormatNestedMachinesError)
}


//------------------------------------------------------------------------------
// PARSERS / Types / Errors / Format / NestedMachines

#[derive(Debug, Deserialize, Serialize)]
pub enum FormatNestedMachinesError {
    StateFile(StateFileParseError),
}

//------------------------------------------------------------------------------
// PARSERS / Types / Errors / State File

#[derive(Debug, Deserialize, Serialize)]
pub enum StateFileParseError {
    YAML(StateYAMLFileParseError),
    UnsupportedExtension,
    NoExtensionOrError,
}

//------------------------------------------------------------------------------
// PARSERS / Types / Errors / State File / YAML

#[derive(Debug, Deserialize, Serialize)]
pub enum StateYAMLFileParseError {
    File(String),
    YAML(StateYAMLFileParseYAMLError),
    UnknownFormat,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StateYAMLFileParseYAMLError {
    pub file_path: String,
    pub format: StateFileFormat,
    pub error: String,
}

//------------------------------------------------------------------------------
// PARSERS / Parser Combinators

/// Create Elea Machines using the Lulo Filesystem representation.
///
/// works with yaml files
pub fn to_machines(machines_path: &Path, format: Format) -> Result<Vec<elea::Machine>, MachinesError> {
    match format {
        Format::NestedMachines => {
            return nested_machines_from_filesystem(machines_path).map_err(|err| {
                MachinesError::NestedMachines(err)
            });
        }
    }
}


fn nested_machines_from_filesystem(machines_path: &Path) -> Result<Vec<elea::Machine>, FormatNestedMachinesError> {
    
    let mut states_default = Vec::new(); 
    let mut states_by_machine_id: HashMap<String,Vec<elea::State>> = HashMap::new();

    for entry in WalkDir::new(machines_path)
        .into_iter()
        .filter_map(|e| e.ok()) 
        .filter(|e| !e.file_type().is_dir())
    {
        let path = entry.path();
        let prefix = path.strip_prefix(machines_path).unwrap();

        let num_of_parts = prefix.iter().count();
        // Default machine
        if num_of_parts == 1 {
            let states = states_from_file(path, "default").map_err(|err| {
                FormatNestedMachinesError::StateFile(err)
            })?;
            states_default.extend(states);
        // Machine is first directory
        } else if num_of_parts >= 2 {
            let base_path = prefix.parent();

            let mut machine_name = String::new();
            let mut state_parts: Vec<&str> = Vec::new();

            // there should be at least 1 because of >=2
            // the first will be machine
            let mut count = 1;
            for c in base_path.unwrap().into_iter() {
                let part_str = c.to_str().unwrap();
                if count == 1 {
                    machine_name.push_str(part_str);
                } else {
                    state_parts.push(part_str);
                }
                count += 1;
            }
            
            let states = states_from_file(path, &machine_name).map_err(|err| {
                FormatNestedMachinesError::StateFile(err)
            })?;
            let states_with_full_id =
                states.into_iter()
                    .map(|state| {
                        let full_id;
                        if state_parts.is_empty() {
                            full_id = state.id.0;
                        } else {
                            full_id = state_parts.join("/") + "/" + &state.id.0;
                        } 
                        elea::State::new(&full_id, state.arrows)
                    }).collect();
            match states_by_machine_id.entry(machine_name) {
                Entry::Vacant(e) => { e.insert(states_with_full_id); },
                Entry::Occupied(mut e) => { e.get_mut().extend(states_with_full_id); }
            }
            //states_by_machine_id.insert(machine_name, states_with_full_id);
        }
    }

    let mut machines: Vec<elea::Machine> = states_by_machine_id.iter().map(|(m_id, m_states)| {
        println!("machine id {}", m_id);
        elea::Machine::new(m_id, m_states.clone())
    }).collect();
    machines.push(elea::Machine::new("default", states_default));
    return Ok(machines);
}

fn states_from_file(file_path: &Path, machine_id: &str)->  Result<Vec<elea::State>, StateFileParseError> {
    let extension = file_path.extension().unwrap_or(OsStr::new("__no_ext_or_err__")).to_str().unwrap_or("__error__");
    return match extension {
        "yaml"             => states_from_yaml_file(file_path, machine_id).map_err(|err| {
            StateFileParseError::YAML(err)
        }),
        "__no_ext_or_err__" => Err(StateFileParseError::NoExtensionOrError),
        _                   => Err(StateFileParseError::UnsupportedExtension),
    }
}


/// Parse the different state file YAML formats
///
/// More details on options.
fn states_from_yaml_file(file_path: &Path, machine_id: &str) -> Result<Vec<elea::State>, StateYAMLFileParseError> {

    let format = yaml_state_file_format(file_path)?;
    println!("Parsing state YAML file [{}] as {}", file_path.display(), format);

    match format {
        // Option 1: Entire file represents one state
        StateFileFormat::State => {
            return state_from_file(file_path).map(|st| { vec![st] });
        },
        // Option 2: File contains a state tree
        StateFileFormat::Tree => {
            return states_from_tree_file(file_path, machine_id);
        },
        StateFileFormat::Unknown => {
            return Err(StateYAMLFileParseError::UnknownFormat);
        },
        _ => {
            println!("Error: StateList not implemented!!!");
            return Ok(Vec::new());
        }
    }
}

fn state_from_file(file_path: &Path) -> Result<elea::State, StateYAMLFileParseError> {
    let file = state_yaml_file(file_path)?;     
    return serde_yaml::from_reader(file)
            .map_err(|err| {
                StateYAMLFileParseError::YAML(StateYAMLFileParseYAMLError {
                    format: StateFileFormat::State, 
                    file_path: file_path.display().to_string(),
                    error: err.to_string(),
                })
            })
            .map(|state: elea::State| {
                // recreate state so that indexes get built and data is sorted
                elea::State::new(&state.id.0, state.arrows.clone())
            });
}

fn states_from_tree_file(file_path: &Path, machine_id: &str) -> Result<Vec<elea::State>, StateYAMLFileParseError> {
    let file = state_yaml_file(file_path)?;     
    let state_tree: StateTree = serde_yaml::from_reader(file).map_err(|err| {
        StateYAMLFileParseError::YAML(StateYAMLFileParseYAMLError {
            format: StateFileFormat::State, 
            file_path: file_path.display().to_string(),
            error: err.to_string(),
        })
    })?;

    let mut states: Vec<elea::State> = Vec::new();

    let mut unproc_nodes: Vec<(StateTree, String)> = vec![(state_tree.clone(), String::new())];
    let root_state_id = machine_id.to_owned() + "/" + &state_tree.id;

    while let Some((node, prefix)) = unproc_nodes.pop() {

        let mut arrows: Vec<elea::Arrow> = Vec::new();

        let state_id: String;
        if prefix.is_empty() {
            state_id = node.id;
        } else {
            state_id = prefix.to_owned() + "/" + &node.id;
        };

        for child_node in node.tree {

            // if no arrows, then goes to root state and not a new state
            let arrow: elea::Arrow;
            if child_node.tree.is_empty() {
                arrow = elea::Arrow::new(&child_node.id, &root_state_id);
            } else {
                let arrow_next = machine_id.to_owned() + "/" + &state_id.clone() + "/" + &child_node.id;
                arrow = elea::Arrow::new(&child_node.id, &arrow_next);
                unproc_nodes.push((child_node, state_id.clone()));
            }
            arrows.push(arrow);
        }
        let new_state = elea::State::new(&state_id, arrows);
        states.push(new_state);
    }
    
    return Ok(states);
}

fn state_yaml_file(file_path: &Path) -> Result<fs::File, StateYAMLFileParseError> {
    return fs::File::open(file_path).map_err(|err| {
        StateYAMLFileParseError::File(err.to_string())
    });
}

//fn dir_has_dirs(dir_path: &str) -> io::Result<bool> {
    //for entry in fs::read_dir(dir_path)? {
        //let path = &entry?.path();
        //if path.is_dir() {
            //return Ok(true);
        //}
    //}
    //return Ok(false);
//}


