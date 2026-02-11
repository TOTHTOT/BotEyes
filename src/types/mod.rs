//! Types module - Enums and configuration structs
//!
//! Contains all type definitions used by the RoboEyes library.

/// Mood types for eye expressions
///
/// Determines how the eyelids are drawn:
/// - `Default`: Normal open eyes
/// - `Tired`: Upper triangular eyelids partially covering eyes
/// - `Angry`: Angled upper eyelids for angry expression
/// - `Happy`: Lower rounded eyelid overlays for happy expression
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mood {
    Default,
    Tired,
    Angry,
    Happy,
}

/// Predefined eye positions (gaze directions)
///
/// Represents 8 compass directions plus center. The eye position
/// is constrained within the screen boundaries.
///
/// ```text
///     NW  N  NE
///      \  |  /
///     W--+--E
///      /  |  \
///     SW  S  SE
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Position {
    North,     // Top center
    NorthEast, // Top right corner
    East,      // Middle right
    SouthEast, // Bottom right corner
    South,     // Bottom center
    SouthWest, // Bottom left corner
    West,      // Middle left
    NorthWest, // Top left corner
    Center,    // Middle center
}

/// Configuration for eye geometry
///
/// Contains the default and current sizes for eyes. All values are in pixels.
#[derive(Debug, Clone)]
pub struct EyeGeometry {
    /// Default eye width in pixels
    pub width: u32,
    /// Default eye height in pixels
    pub height: u32,
    /// Border radius for rounded corners
    pub border_radius: u32,
}

impl EyeGeometry {
    /// Create new geometry with default values
    pub fn new(width: u32, height: u32, border_radius: u32) -> Self {
        Self {
            width,
            height,
            border_radius,
        }
    }
}

/// Configuration for blink behavior
///
/// Controls automatic eye blinking:
/// - `interval`: Base time between blinks in seconds
/// - `variation`: Random variation added to interval (0 = no variation)
#[derive(Debug, Clone)]
pub struct BlinkConfig {
    pub interval: u64,
    pub variation: u64,
}

impl Default for BlinkConfig {
    fn default() -> Self {
        Self {
            interval: 1,
            variation: 4,
        }
    }
}

/// Configuration for idle animation
///
/// Controls automatic eye movement when idle:
/// - `interval`: Base time between eye movements in seconds
/// - `variation`: Random variation added to interval
#[derive(Debug, Clone)]
pub struct IdleConfig {
    pub interval: u64,
    pub variation: u64,
}

impl Default for IdleConfig {
    fn default() -> Self {
        Self {
            interval: 1,
            variation: 3,
        }
    }
}

/// Screen constraint calculation helper
pub struct ScreenConstraints {
    pub width: u32,
    pub height: u32,
}

impl ScreenConstraints {
    /// Create new constraints for given screen dimensions
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Maximum X position for left eye
    pub fn max_x(&self, eye_width: u32, space_between: u32, right_eye_width: u32) -> i32 {
        (self.width as i32) - eye_width as i32 - space_between as i32 - right_eye_width as i32
    }

    /// Maximum Y position for left eye
    pub fn max_y(&self, eye_height: u32) -> i32 {
        (self.height as i32) - eye_height as i32
    }
}
