//! Movement and physics systems for the Titan survival game.
//!
//! Provides world origin shifting and relative physics calculations
//! for entities living on a moving colossus.

mod origin_shift;
mod relative_physics;
mod terrain_deformation;

pub use origin_shift::WorldOriginManager;
pub use relative_physics::{RelativePhysics, DEFAULT_GRAVITY};
pub use terrain_deformation::{
    FlexibleJoint, FoundationProperties, FoundationType, TerrainStability,
};
