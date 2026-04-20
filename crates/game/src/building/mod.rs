//! Building systems for Titan survival.
//!
//! Provides foundations and placement systems for structures on the Titan.

mod foundations;
mod placement;

pub use foundations::{FlexibleJoint, Foundation, FoundationType};
pub use placement::BlockInteraction;
