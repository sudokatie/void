//! Event systems for space station survival.
//!
//! Provides random events and cascade event chains.

mod cascade_events;
mod random_events;

pub use cascade_events::{CascadeEffect, CascadeEvent, CascadeEventType};
pub use random_events::{RandomEvent, RandomEventType};
