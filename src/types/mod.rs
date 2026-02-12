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
/// - `x_range`: X-axis movement range as percentage of available width (0-100), default 100
/// - `y_range`: Y-axis movement range as percentage of available height (0-100), default 100
#[derive(Debug, Clone)]
pub struct IdleConfig {
    pub interval: u64,
    pub variation: u64,
    pub x_range: u32,
    pub y_range: u32,
}

impl Default for IdleConfig {
    fn default() -> Self {
        Self {
            interval: 1,
            variation: 3,
            x_range: 100,
            y_range: 100,
        }
    }
}

/// Configuration for RoboEyes instance
///
/// Contains all configurable default values for eye appearance.
/// Use [`Default`] to get sensible defaults, or build a custom config.
///
/// # Example
///
/// ```rust
/// use boteyes::RoboEyesConfig;
///
/// let config = RoboEyesConfig::default()
///     .with_eye_width(50)
///     .with_eye_height(50)
///     .with_border_radius(12)
///     .with_space_between(15);
/// ```
#[derive(Debug, Clone)]
pub struct RoboEyesConfig {
    /// Default eye width in pixels
    pub eye_width: u32,
    /// Default eye height in pixels
    pub eye_height: u32,
    /// Border radius for rounded corners
    pub border_radius: u32,
    /// Space between eyes
    pub space_between: u32,
}

impl Default for RoboEyesConfig {
    fn default() -> Self {
        Self {
            eye_width: 36,
            eye_height: 36,
            border_radius: 8,
            space_between: 10,
        }
    }
}

impl RoboEyesConfig {
    /// Create a new config with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set eye width
    pub fn with_eye_width(mut self, width: u32) -> Self {
        self.eye_width = width;
        self
    }

    /// Set eye height
    pub fn with_eye_height(mut self, height: u32) -> Self {
        self.eye_height = height;
        self
    }

    /// Set border radius
    pub fn with_border_radius(mut self, radius: u32) -> Self {
        self.border_radius = radius;
        self
    }

    /// Set space between eyes
    pub fn with_space_between(mut self, space: u32) -> Self {
        self.space_between = space;
        self
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
