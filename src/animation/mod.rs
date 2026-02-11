//! Animation module - Sweat drop animation
//!
//! Contains the SweatDrop struct for animating sweat on the forehead.

use rand::Rng;

/// State for a single sweat drop animation
///
/// Tracks position, size, and animation progress for
/// one of the three sweat drops.
#[derive(Debug, Clone)]
pub struct SweatDrop {
    /// Initial X position (anchor point)
    x_initial: i32,
    /// Current X position
    x: f32,
    /// Current Y position
    y: f32,
    /// Maximum Y position (animation end point)
    y_max: i32,
    /// Current width
    width: f32,
    /// Current height
    height: f32,
}

/// Position for sweat drops on the forehead
#[derive(Debug, Clone, Copy)]
pub enum SweatPosition {
    Left,
    Center,
    Right,
}

impl SweatDrop {
    /// Create a new sweat drop with random initial position
    pub fn new(screen_width: u32, position: SweatPosition) -> Self {
        let mut rng = rand::thread_rng();
        let (x_initial, y_max) = match position {
            SweatPosition::Left => (rng.gen_range(0..30), rng.gen_range(10..20)),
            SweatPosition::Center => (
                rng.gen_range(30..(screen_width as i32 - 30)),
                rng.gen_range(10..20),
            ),
            SweatPosition::Right => (
                (screen_width as i32 - 30) + rng.gen_range(0..30),
                rng.gen_range(10..20),
            ),
        };

        Self {
            x_initial,
            x: x_initial as f32,
            y: 2.0,
            y_max,
            width: 1.0,
            height: 2.0,
        }
    }

    /// Update sweat drop animation state
    ///
    /// Returns true if animation completed and needs reset.
    pub fn update(&mut self) -> bool {
        let should_reset = if self.y as i32 <= self.y_max {
            self.y += 0.5;
            false
        } else {
            true
        };

        // Grow then shrink
        if self.y as i32 <= self.y_max / 2 {
            self.width += 0.5;
            self.height += 0.5;
        } else if self.y as i32 > self.y_max / 2 {
            self.width -= 0.1;
            self.height -= 0.5;
        }

        // Keep centered on initial X
        self.x = self.x_initial as f32 - (self.width / 2.0);

        should_reset
    }

    /// Get current drawing parameters
    pub fn params(&self) -> (i32, i32, u32, u32) {
        (
            self.x as i32,
            self.y as i32,
            self.width as u32,
            self.height as u32,
        )
    }

    /// Reset the sweat drop with new position
    pub fn reset(&mut self, screen_width: u32, position: SweatPosition) {
        *self = SweatDrop::new(screen_width, position);
    }
}

/// Array of 3 sweat drops (left, center, right)
#[derive(Debug, Clone)]
pub struct SweatDrops(pub [SweatDrop; 3]);

impl SweatDrops {
    /// Create new sweat drops for given screen width
    pub fn new(screen_width: u32) -> Self {
        Self([
            SweatDrop::new(screen_width, SweatPosition::Left),
            SweatDrop::new(screen_width, SweatPosition::Center),
            SweatDrop::new(screen_width, SweatPosition::Right),
        ])
    }

    /// Update all drops and return indices that need reset
    pub fn update(&mut self) -> Vec<usize> {
        let mut reset_indices = Vec::new();
        for (i, drop) in self.0.iter_mut().enumerate() {
            if drop.update() {
                reset_indices.push(i);
            }
        }
        reset_indices
    }
}
