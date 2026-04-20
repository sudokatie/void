//! Power generation and distribution systems.
//!
//! Provides reactor, power grid, and consumer management.

mod consumers;
mod grid;
mod reactor;

pub use consumers::{PowerConsumer, PowerManager};
pub use grid::PowerGrid;
pub use reactor::Reactor;
