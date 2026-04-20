//! Falling and respawn mechanics.
//!
//! Handles what happens when a player falls off the Titan.

use glam::IVec3;

/// Result of a fall check.
#[derive(Clone, Debug)]
pub struct FallResult {
    /// Damage taken from the fall.
    pub damage: f32,
    /// Position to respawn at.
    pub respawn_position: IVec3,
    /// Distance fallen before respawn.
    pub fall_distance: f32,
}

/// Manages falling mechanics and anchor respawn points.
#[derive(Clone, Debug)]
pub struct Falling {
    /// Damage rate per tick when in the void.
    void_damage_rate: f32,
    /// Valid respawn anchor positions.
    anchor_positions: Vec<IVec3>,
}

impl Falling {
    /// Create a new falling manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            void_damage_rate: 10.0,
            anchor_positions: Vec::new(),
        }
    }

    /// Add an anchor position for respawning.
    pub fn add_anchor(&mut self, pos: IVec3) {
        if !self.anchor_positions.contains(&pos) {
            self.anchor_positions.push(pos);
        }
    }

    /// Check if a position constitutes a fall.
    ///
    /// Returns `None` if the position is above the Titan's height (safe).
    /// Returns `Some(FallResult)` if the player has fallen into the void.
    #[must_use]
    pub fn check_fall(&self, position: IVec3, titan_height: i32) -> Option<FallResult> {
        if position.y >= titan_height {
            return None;
        }

        let fall_distance = (titan_height - position.y) as f32;
        let damage = fall_distance * self.void_damage_rate;

        let respawn_position = self
            .nearest_anchor(position)
            .unwrap_or(IVec3::new(0, titan_height, 0));

        Some(FallResult {
            damage,
            respawn_position,
            fall_distance,
        })
    }

    /// Find the nearest anchor to a position.
    #[must_use]
    pub fn nearest_anchor(&self, pos: IVec3) -> Option<IVec3> {
        self.anchor_positions
            .iter()
            .min_by_key(|anchor| {
                let diff = **anchor - pos;
                diff.x * diff.x + diff.y * diff.y + diff.z * diff.z
            })
            .copied()
    }

    /// Remove an anchor position.
    pub fn remove_anchor(&mut self, pos: IVec3) {
        self.anchor_positions.retain(|p| *p != pos);
    }
}

impl Default for Falling {
    fn default() -> Self {
        Self::new()
    }
}
