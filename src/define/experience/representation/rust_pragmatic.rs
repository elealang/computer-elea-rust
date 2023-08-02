//! 
//! ! Pragmatic Rust REPRESENTATION
//! A fundamental Rust representation of Elea that is isomorphic to its 
//! formally defined structure. That is, no indexes, runtime additions, or 
//! changes to structure for ease-of-use.
//!
//! ----------------------------------------------------------------------------
//! type: computer for Elea
//! ----------------------------------------------------------------------------

use serde::{Deserialize, Serialize};
use std::collections::HashMap;


use crate::define::experience::representation::rust_minimal::{
    MachineId, StateId, ArrowId
};

//------------------------------------------------------------------------------
// SPACE

//------------------------------------------------------------------------------
// SPACE / Theory

// Space
//
// Description
#[derive(Deserialize, Serialize)]
pub struct Space {
    pub machines: Vec<Machine>,
    #[serde(skip)]
    pub machine_by_id: HashMap<MachineId, Machine>,
}

impl Space {

    pub fn new(mut machines: Vec<Machine>) -> Self {
        machines.sort_by(|m1, m2| m1.id.cmp(&m2.id));
        let mut machine_by_id = HashMap::new();
        for machine in &machines {
            machine_by_id.insert(machine.id.clone(), machine.clone());
        }
        return Self {
            machines, 
            machine_by_id
        };
    }
}

// Machine
// 
// Description
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Machine {
    pub id: MachineId,
    pub states: Vec<State>,
    #[serde(skip_serializing)]
    pub state_by_id: HashMap<StateId, State>,
}

impl Machine {

    pub fn new(id: &str, mut states_: Vec<State>) -> Self {
        states_.sort_by(|s1, s2| s1.id.cmp(&s2.id));
        let mut state_by_id_ = HashMap::new();
        for state in &states_ {
            state_by_id_.insert(state.id.clone(), state.clone());
        }
        return Self {
            id: MachineId(id.to_string()),
            states: states_,
            state_by_id: state_by_id_,
        }
    }
    
}
  
impl PartialEq for Machine {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.states == other.states
    }
}
impl Eq for Machine {}

// State
//
// Description
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct State {
    pub id: StateId,
    pub arrows: Vec<Arrow>,
    #[serde(default = "HashMap::new")]
    #[serde(skip_serializing)]
    pub arrow_by_id: HashMap<ArrowId, Arrow>,
    pub description: Option<String>,
}

impl State {

    pub fn new(id: &str, mut arrows_: Vec<Arrow>) -> Self {
        arrows_.sort_by(|a1, a2| a1.id.cmp(&a2.id));
        let mut arrow_by_id_ = HashMap::new();
        for arr in &arrows_ {
            arrow_by_id_.insert(arr.id.clone(), arr.clone());
        }
        return Self {
            id: StateId(id.to_string()),
            arrows: arrows_,
            arrow_by_id: arrow_by_id_,
            description: None,
        }
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.arrows == other.arrows && self.description == other.description
    }
}
impl Eq for State {}

// Arrow
//
// Description
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Arrow {
    pub id: ArrowId,
    pub next: StateId,
    pub description: Option<String>,
}

impl Arrow {

    pub fn new(id: &str, next_state: &str) -> Self {
        return Self {
            id: ArrowId(id.to_string()),
            next: StateId(next_state.to_string()),
            description: None,
        }
    }
}

impl PartialEq for Arrow {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.next == other.next && self.description == other.description
    }
}
impl Eq for Arrow {}
