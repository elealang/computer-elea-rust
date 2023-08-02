//! 
//! ! Minimal Rust REPRESENTATION
//! A fundamental Rust representation of Elea that is isomorphic to its 
//! formally defined structure. That is, no indexes, runtime additions, or 
//! changes to structure for ease-of-use.
//!
//! ----------------------------------------------------------------------------
//! type: computer for Elea
//! ----------------------------------------------------------------------------

use serde::{Deserialize, Serialize};
use std::hash::Hash;

//------------------------------------------------------------------------------
// SPACE

//------------------------------------------------------------------------------
// SPACE / Theory

/// Space
///
/// Description
#[derive(Serialize, Deserialize)]
pub struct Space {
    pub machines: Vec<MachineId>,
    pub states:   Vec<StateId>,
    pub arrows:   Vec<Arrow>,
}

/// State Id
///
/// Description
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MachineId(pub String);

/// State Id
/// 
/// Description
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct StateId(pub String);

/// Arrow
///
/// Description
/// should be unique in space of (init_state_id, term_state_id)?
#[derive(Serialize, Deserialize)]
pub struct Arrow {
    pub id:          ArrowId,
    pub init_state_id: StateId,
    pub term_state_id: StateId,
}

/// Arrow Id
///
/// if arrows are uniquely determined by (init_state_id, term_state_id) then
/// this is simply a useful alias for each pair of those
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ArrowId(pub String);

// SPACE / Implementation
//--------------------------------------------------------------------------------

/// Addition
/// 
/// Aliases: expnasion
#[derive(Serialize, Deserialize)]
pub struct Addition {
    pub id: AdditionId,
    pub machines: Vec<MachineId>,
    pub states: Vec<StateId>,
    pub arrows: Vec<Arrow>,
}

/// Addition Id
/// 
/// Description
#[derive(Serialize, Deserialize)]
pub struct AdditionId(String);

//--------------------------------------------------------------------------------
// TIME

//--------------------------------------------------------------------------------
// TIME / Theory

/// Time
/// 
/// Description
#[derive(Serialize, Deserialize)]
pub struct Time {
    pub functions: Vec<Function>,
    pub proofs: Vec<Proof>,
}

/// path / journey / goal / aspiration / 
/// data type / constructor
#[derive(Serialize, Deserialize)]
pub struct Function {
    pub function_id: FunctionId,
    pub init_state_id: StateId,
    pub term_state_id: StateId,
}

/// FunctionId
///
/// Description
#[derive(Serialize, Deserialize)]
pub struct FunctionId(String);

/// Proof
///
/// Description
#[derive(Serialize, Deserialize)]
pub struct Proof {
    pub id: ProofId,
    pub arrow_ids: Vec<ArrowId>,
}

/// Proof Id
///
/// Description
#[derive(Serialize, Deserialize)]
pub struct ProofId(String);

// TIME / Implementation
//--------------------------------------------------------------------------------

/// Multiplication
/// 
/// Aliases: extension
#[derive(Serialize, Deserialize)]
pub struct Multiplication {
    pub id: MultiplicationId,
    pub function_id: FunctionId, 
    pub proof_id: ArrowId,
}

/// Multiplication Id
/// 
/// Description
#[derive(Serialize, Deserialize)]
pub struct MultiplicationId(String);

//  AGENCY
//--------------------------------------------------------------------------------

//  AGENCY / Theory
//--------------------------------------------------------------------------------

/// Agency
///
/// Description
#[derive(Serialize, Deserialize)]
pub struct Agency {
    /// aliases
    ///  - persons [human], actors
    pub agents: Vec<Agent>, // actors
    /// aliases
    ///  - desires [human]
    pub types:  Vec<Type>,  // desires
}

// laws / types
// monads??
//   effects: what happens
// control flow
//   actions / reactions
// property

/// Agent
///
/// Aliases | role, user, account
#[derive(Serialize, Deserialize)]
pub struct Agent {
    pub id:    AgentId,
    /// Aliases | capabilities
    pub types: Vec<TypeId>,
}

/// Agent Id
///
/// Description
#[derive(Serialize, Deserialize)]
pub struct AgentId(String);

/// Type
/// 
/// Description | description
/// Aliases | want, desire, need, requirement, consequence, outcome, result
#[derive(Serialize, Deserialize)]
pub struct Type {
    pub id: TypeId,
    /// Aliases | expansion
    pub addition: Vec<Addition>,
    /// Aliases | continuation, extension
    pub multiplication: Vec<Multiplication>,
}

/// Type Id
/// 
/// Description
#[derive(Serialize, Deserialize)]
pub struct TypeId(String);

//  AGENCY / Implementation
//--------------------------------------------------------------------------------

/// Computation
///
/// Bridge between space and time
/// basically a monad??
#[derive(Serialize, Deserialize)]
pub struct Action {
    pub action_id: ActionId,
    pub function_id: FunctionId,
    pub type_: TypeId,
}

/// Computation Id
///
/// Description
#[derive(Serialize, Deserialize)]
pub struct ActionId(String);


//  PROGRAMS
//--------------------------------------------------------------------------------

// programs (space, time, agency)
// properties??
