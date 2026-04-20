//! Space station architecture and systems.
//!
//! Provides room types, bulkheads, hull integrity, and station systems.

mod bulkheads;
mod hull;
mod rooms;
mod systems;

pub use bulkheads::{Bulkhead, BulkheadState};
pub use hull::HullSegment;
pub use rooms::{RoomProperties, RoomType, StationRoom};
pub use systems::{RoomSystem, SystemState};
