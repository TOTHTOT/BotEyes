//! BotEyes - Rust port of FluxGarage RoboEyes
//!
//! A library for drawing smoothly animated robot eyes on OLED displays.
//! Originally written for Arduino/Adafruit GFX, now ported to pure Rust.
//!
//! ## Features
//!
//! - **Eyes Rendering**: Draw left and right eyes with customizable size, position, and border radius
//! - **Mood Expressions**: Default, Tired, Angry, and Happy moods
//! - **Eye Positions**: 8 predefined directions (N, NE, E, SE, S, SW, W, NW) + Center
//! - **Animations**: Blink, Confused (horizontal shake), Laugh (vertical shake), Sweat drops
//! - **Special Modes**: Cyclops (single eye), Curious (eyes grow when looking sideways)
//! - **Frame Tweening**: Smooth animations using interpolation between frames
//!
//! ## Quick Start
//!
//! ```rust
//! use boteyes::{RoboEyes, Mood};
//!
//! // Create eyes instance (128x64 OLED resolution)
//! let mut eyes = RoboEyes::new(128, 64);
//!
//! // Set mood and open eyes
//! eyes.set_mood(Mood::Happy);
//! eyes.open();
//!
//! // Draw frame at current time (in milliseconds)
//! let img = eyes.draw_eyes(1000);
//!
//! // Save to file
//! img.save("happy_eyes.png")?;
//! ```
//!
//! ## Architecture
//!
//! The library is organized into modules:
//!
//! - [`types`]: Enums (Mood, Position) and config structs
//! - [`draw`]: Graphics primitives (rounded rect, triangle)
//! - [`animation`]: Sweat drop animation state
//!
//! The drawing pipeline:
//!
//! 1. **Pre-calculation**: Tween values for smooth animation transitions
//! 2. **Animation processing**: Apply macro animations (blink, flicker, etc.)
//! 3. **Shape drawing**: Render rounded rectangles and triangles
//! 4. **Mood overlays**: Apply eyelid effects (tired, angry, happy)
//!

mod animation;
mod draw;
mod types;

pub use types::{BlinkConfig, EyeGeometry, IdleConfig, Mood, Position, ScreenConstraints};

use animation::{SweatDrops, SweatPosition};
use draw::{draw_rounded_rect, draw_triangle};

use rand::Rng;

// Color constants for display rendering
const BGCOLOR: u8 = 0;
const MAINCOLOR: u8 = 255;

/// Main RoboEyes struct
///
/// Contains all state for rendering robot eyes:
/// - Display dimensions (width, height)
/// - Eye geometry (size, position, border radius)
/// - Animation state (current frame values)
/// - Mood and mode settings
/// - Animation timers and flags
///
/// ## Frame Timing
///
/// The library uses millisecond timestamps for animation timing.
/// Pass `current_time` to [`draw_eyes()`] to advance animations.
pub struct RoboEyes {
    // Display dimensions
    screen_width: u32,
    screen_height: u32,

    // Current animation time (milliseconds)
    current_time: u64,

    // Mood state
    mood: Mood,

    // Eye geometry
    eye_l: EyeGeometry,
    eye_r: EyeGeometry,

    // Eye positions (current and target for tweening)
    eye_l_x: i32,
    eye_l_y: i32,
    eye_l_x_next: i32,
    eye_l_y_next: i32,
    eye_r_x: i32,
    eye_r_y: i32,
    eye_r_x_next: i32,
    eye_r_y_next: i32,

    // Eye height for animation (1 = closed, default = open)
    eye_l_height_default: u32,
    eye_l_height_current: u32,
    eye_l_height_next: u32,
    eye_r_height_default: u32,
    eye_r_height_current: u32,
    eye_r_height_next: u32,

    // Height offset for curious mode
    eye_l_height_offset: u32,
    eye_r_height_offset: u32,

    // Space between eyes
    space_between: u32,
    space_between_next: u32,

    // Eyelid states (for tweening)
    eyelids_tired_height: u32,
    eyelids_tired_height_next: u32,
    eyelids_angry_height: u32,
    eyelids_angry_height_next: u32,
    eyelids_happy_bottom_offset: u32,
    eyelids_happy_bottom_offset_next: u32,

    // Eye open/close state
    eye_l_open: bool,
    eye_r_open: bool,

    // Mode flags
    cyclops: bool,
    curious: bool,
    sweat: bool,

    // Auto-blinker
    autoblinker: bool,
    blink_config: BlinkConfig,
    blink_timer: u64,

    // Idle mode
    idle: bool,
    idle_config: IdleConfig,
    idle_timer: u64,

    // Horizontal flicker (shaking)
    h_flicker: bool,
    h_flicker_amplitude: u32,
    h_flicker_alternate: bool,

    // Vertical flicker
    v_flicker: bool,
    v_flicker_amplitude: u32,
    v_flicker_alternate: bool,

    // Confused animation (horizontal shake)
    confused: bool,
    confused_timer: u64,
    confused_duration: u64,
    confused_toggle: bool,

    // Laugh animation (vertical bounce)
    laugh: bool,
    laugh_timer: u64,
    laugh_duration: u64,
    laugh_toggle: bool,

    // Sweat animation state (3 drops)
    sweat_drops: SweatDrops,
}

impl RoboEyes {
    /// Create a new RoboEyes instance
    ///
    /// # Arguments
    ///
    /// * `screen_width` - Display width in pixels (e.g., 128 for common OLEDs)
    /// * `screen_height` - Display height in pixels (e.g., 64 for common OLEDs)
    ///
    /// # Example
    ///
    /// ```rust
    /// // 128x64 OLED display
    /// let eyes = RoboEyes::new(128, 64);
    /// ```
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        let default_width = 36;
        let default_height = 36;
        let default_border_radius = 8;
        let default_space = 10;

        let mut rng = rand::thread_rng();

        // Calculate initial position (centered)
        let eye_l_x = ((screen_width as i32)
            - default_width as i32
            - default_space as i32
            - default_width as i32)
            / 2;
        let eye_l_y = ((screen_height as i32) - default_height as i32) / 2;

        Self {
            screen_width,
            screen_height,
            current_time: 0,

            mood: Mood::Default,

            eye_l: EyeGeometry::new(default_width, default_height, default_border_radius),
            eye_r: EyeGeometry::new(default_width, default_height, default_border_radius),

            eye_l_x,
            eye_l_y,
            eye_l_x_next: eye_l_x,
            eye_l_y_next: eye_l_y,

            eye_r_x: 0,
            eye_r_y: eye_l_y,
            eye_r_x_next: 0,
            eye_r_y_next: eye_l_y,

            eye_l_height_default: default_height,
            eye_l_height_current: 1,
            eye_l_height_next: default_height,
            eye_r_height_default: default_height,
            eye_r_height_current: 1,
            eye_r_height_next: default_height,

            eye_l_height_offset: 0,
            eye_r_height_offset: 0,

            space_between: default_space,
            space_between_next: default_space,

            eyelids_tired_height: 0,
            eyelids_tired_height_next: 0,
            eyelids_angry_height: 0,
            eyelids_angry_height_next: 0,
            eyelids_happy_bottom_offset: 0,
            eyelids_happy_bottom_offset_next: 0,

            eye_l_open: false,
            eye_r_open: false,

            cyclops: false,
            curious: false,
            sweat: false,

            autoblinker: false,
            blink_config: BlinkConfig::default(),
            blink_timer: 0,

            idle: false,
            idle_config: IdleConfig::default(),
            idle_timer: rng.gen_range(0..u64::MAX),

            h_flicker: false,
            h_flicker_amplitude: 2,
            h_flicker_alternate: false,

            v_flicker: false,
            v_flicker_amplitude: 10,
            v_flicker_alternate: false,

            confused: false,
            confused_timer: 0,
            confused_duration: 500,
            confused_toggle: true,

            laugh: false,
            laugh_timer: 0,
            laugh_duration: 500,
            laugh_toggle: true,

            sweat_drops: SweatDrops::new(screen_width),
        }
    }

    // =====================================================================
    // Configuration Setters
    // =====================================================================

    /// Set the eye mood expression
    pub fn set_mood(&mut self, mood: Mood) {
        self.mood = mood;
    }

    /// Set eye size for both eyes
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.eye_l.width = width;
        self.eye_r.width = width;
        self.eye_l.height = height;
        self.eye_r.height = height;

        self.eye_l_height_default = height;
        self.eye_r_height_default = height;
        self.eye_l_height_next = height;
        self.eye_r_height_next = height;
    }

    /// Set border radius for rounded corners
    pub fn set_border_radius(&mut self, left: u32, right: u32) {
        self.eye_l.border_radius = left;
        self.eye_r.border_radius = right;
    }

    /// Set space between the eyes
    pub fn set_space_between(&mut self, space: u32) {
        self.space_between = space;
        self.space_between_next = space;
    }

    /// Set eye gaze direction
    pub fn set_position(&mut self, position: Position) {
        let constraints = ScreenConstraints::new(self.screen_width, self.screen_height);
        let max_x = constraints.max_x(self.eye_l.width, self.space_between, self.eye_r.width);
        let max_y = constraints.max_y(self.eye_l.height);

        match position {
            Position::North => {
                self.eye_l_x_next = max_x / 2;
                self.eye_l_y_next = 0;
            }
            Position::NorthEast => {
                self.eye_l_x_next = max_x;
                self.eye_l_y_next = 0;
            }
            Position::East => {
                self.eye_l_x_next = max_x;
                self.eye_l_y_next = max_y / 2;
            }
            Position::SouthEast => {
                self.eye_l_x_next = max_x;
                self.eye_l_y_next = max_y;
            }
            Position::South => {
                self.eye_l_x_next = max_x / 2;
                self.eye_l_y_next = max_y;
            }
            Position::SouthWest => {
                self.eye_l_x_next = 0;
                self.eye_l_y_next = max_y;
            }
            Position::West => {
                self.eye_l_x_next = 0;
                self.eye_l_y_next = max_y / 2;
            }
            Position::NorthWest => {
                self.eye_l_x_next = 0;
                self.eye_l_y_next = 0;
            }
            Position::Center => {
                self.eye_l_x_next = max_x / 2;
                self.eye_l_y_next = max_y / 2;
            }
        }
    }

    // =====================================================================
    // Mode Setters
    // =====================================================================

    /// Enable or disable cyclops mode (single eye)
    pub fn set_cyclops(&mut self, enabled: bool) {
        self.cyclops = enabled;
    }

    /// Enable or disable curious mode
    pub fn set_curiosity(&mut self, enabled: bool) {
        self.curious = enabled;
    }

    /// Enable or disable sweat animation
    pub fn set_sweat(&mut self, enabled: bool) {
        self.sweat = enabled;
    }

    // =====================================================================
    // Animation Control
    // =====================================================================

    /// Open both eyes
    pub fn open(&mut self) {
        self.eye_l_open = true;
        self.eye_r_open = true;
    }

    /// Close both eyes
    pub fn close(&mut self) {
        self.eye_l_height_next = 1;
        self.eye_r_height_next = 1;
        self.eye_l_open = false;
        self.eye_r_open = false;
    }

    /// Blink both eyes
    pub fn blink(&mut self) {
        self.close();
        self.open();
    }

    /// Open or close specific eyes
    pub fn blink_eyes(&mut self, left: bool, right: bool) {
        if left {
            self.eye_l_height_next = 1;
            self.eye_l_open = false;
        }
        if right {
            self.eye_r_height_next = 1;
            self.eye_r_open = false;
        }
        self.open_eyes(left, right);
    }

    /// Open specific eyes
    pub fn open_eyes(&mut self, left: bool, right: bool) {
        if left {
            self.eye_l_open = true;
        }
        if right {
            self.eye_r_open = true;
        }
    }

    /// Start confused animation
    pub fn anim_confused(&mut self) {
        self.confused = true;
        self.confused_toggle = true;
    }

    /// Start laugh animation
    pub fn anim_laugh(&mut self) {
        self.laugh = true;
        self.laugh_toggle = true;
    }

    // =====================================================================
    // Auto Animation Setters
    // =====================================================================

    /// Enable or disable automatic blinking
    pub fn set_autoblinker(&mut self, enabled: bool, interval: u64, variation: u64) {
        self.autoblinker = enabled;
        self.blink_config.interval = interval;
        self.blink_config.variation = variation;
    }

    /// Enable or disable idle mode
    pub fn set_idle_mode(&mut self, enabled: bool, interval: u64, variation: u64) {
        self.idle = enabled;
        self.idle_config.interval = interval;
        self.idle_config.variation = variation;
    }

    /// Enable or disable horizontal flicker (shaking)
    pub fn set_h_flicker(&mut self, enabled: bool, amplitude: u32) {
        self.h_flicker = enabled;
        self.h_flicker_amplitude = amplitude;
    }

    /// Enable or disable vertical flicker (shaking)
    pub fn set_v_flicker(&mut self, enabled: bool, amplitude: u32) {
        self.v_flicker = enabled;
        self.v_flicker_amplitude = amplitude;
    }

    // =====================================================================
    // Drawing
    // =====================================================================

    /// Draw a frame of the robot eyes animation
    ///
    /// This is the main rendering function. Call this each frame
    /// with the current timestamp to update animations.
    /// Draw eyes to an existing image buffer
    ///
    /// Modifies the given buffer in place instead of creating a new one.
    /// This is more efficient for animation loops.
    ///
    /// # Arguments
    ///
    /// * `img` - Mutable reference to the image buffer (must match screen dimensions)
    /// * `current_time` - Current timestamp in milliseconds
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut eyes = RoboEyes::new(128, 64);
    /// let mut buffer = image::GrayImage::new(128, 64);
    ///
    /// loop {
    ///     eyes.draw_into(&mut buffer, current_time);
    ///     // use buffer...
    /// }
    /// ```
    pub fn draw_into(&mut self, img: &mut GrayImage, current_time: u64) {
        self.current_time = current_time;

        // Clear buffer
        img.pixels_mut().for_each(|p| *p = image::Luma([BGCOLOR]));

        // 1. Pre-calculation: Tween values
        self.update_curious_mode();
        self.update_eye_heights();

        // Tween heights
        self.eye_l_height_current = (self.eye_l_height_current as f32
            + self.eye_l_height_next as f32
            + self.eye_l_height_offset as f32) as u32
            / 2;
        self.eye_r_height_current = (self.eye_r_height_current as f32
            + self.eye_r_height_next as f32
            + self.eye_r_height_offset as f32) as u32
            / 2;

        if self.eye_l_open && self.eye_l_height_current <= 1 + self.eye_l_height_offset {
            self.eye_l_height_next = self.eye_l_height_default;
        }
        if self.eye_r_open && self.eye_r_height_current <= 1 + self.eye_r_height_offset {
            self.eye_r_height_next = self.eye_r_height_default;
        }

        self.space_between =
            (self.space_between as f32 + self.space_between_next as f32) as u32 / 2;
        self.tween_positions();

        self.eye_l.border_radius =
            (self.eye_l.border_radius as f32 + self.eye_l.border_radius as f32) as u32 / 2;
        self.eye_r.border_radius =
            (self.eye_r.border_radius as f32 + self.eye_r.border_radius as f32) as u32 / 2;

        // 2. Animation processing
        self.process_autoblinker();
        self.process_laugh();
        self.process_confused();
        self.process_idle();
        self.apply_flicker();

        if self.cyclops {
            self.eye_r_height_current = 0;
            self.eye_r.width = 0;
            self.space_between = 0;
        }

        // 3. Shape drawing
        draw_rounded_rect(
            img,
            self.screen_width,
            self.screen_height,
            self.eye_l_x,
            self.eye_l_y,
            self.eye_l.width,
            self.eye_l_height_current,
            self.eye_l.border_radius,
            MAINCOLOR,
        );

        if !self.cyclops {
            draw_rounded_rect(
                img,
                self.screen_width,
                self.screen_height,
                self.eye_r_x,
                self.eye_r_y,
                self.eye_r.width,
                self.eye_r_height_current,
                self.eye_r.border_radius,
                MAINCOLOR,
            );
        }

        // 4. Mood overlays
        self.update_mood_transitions();
        self.draw_eyelids(img);

        // 5. Sweat animation
        if self.sweat {
            self.draw_sweat(img);
        }
    }

    /// Draw a frame of the robot eyes animation
    ///
    /// Creates a new image buffer each call. For animation loops, use [`draw_into()`]
    /// instead to avoid allocating a new buffer every frame.
    ///
    /// # Arguments
    ///
    /// * `current_time` - Current timestamp in milliseconds
    ///
    /// # Returns
    ///
    /// A new grayscale image buffer.
    pub fn draw_eyes(&mut self, current_time: u64) -> GrayImage {
        let mut img = GrayImage::new(self.screen_width, self.screen_height);
        self.draw_into(&mut img, current_time);
        img
    }

    // =====================================================================
    // Private Helper Methods
    // =====================================================================

    fn update_curious_mode(&mut self) {
        if self.curious {
            let left_offset = self.eye_l_x_next <= 10
                || (self.eye_l_x_next >= self.get_constraint_x() - 10 && self.cyclops);
            self.eye_l_height_offset = if left_offset { 8 } else { 0 };

            let right_offset =
                self.eye_r_x_next >= self.screen_width as i32 - self.eye_r.width as i32 - 10;
            self.eye_r_height_offset = if right_offset { 8 } else { 0 };
        } else {
            self.eye_l_height_offset = 0;
            self.eye_r_height_offset = 0;
        }
    }

    fn update_eye_heights(&mut self) {
        // Update Y position for vertical centering based on height change
        // When height changes, adjust Y to keep the eye centered vertically
        self.eye_l_y -= self.eye_l_height_offset as i32 / 2;
        self.eye_r_y -= self.eye_r_height_offset as i32 / 2;
    }

    fn tween_positions(&mut self) {
        self.eye_l_x = (self.eye_l_x as f32 + self.eye_l_x_next as f32) as i32 / 2;
        self.eye_l_y = (self.eye_l_y as f32 + self.eye_l_y_next as f32) as i32 / 2;

        self.eye_r_x_next = self.eye_l_x_next + self.eye_l.width as i32 + self.space_between as i32;
        self.eye_r_y_next = self.eye_l_y_next;

        self.eye_r_x = (self.eye_r_x as f32 + self.eye_r_x_next as f32) as i32 / 2;
        self.eye_r_y = (self.eye_r_y as f32 + self.eye_r_y_next as f32) as i32 / 2;
    }

    fn get_constraint_x(&self) -> i32 {
        (self.screen_width as i32)
            - self.eye_l.width as i32
            - self.space_between as i32
            - self.eye_r.width as i32
    }

    fn process_autoblinker(&mut self) {
        if self.autoblinker && self.current_time >= self.blink_timer {
            self.blink();
            let mut rng = rand::thread_rng();
            self.blink_timer = self.current_time
                + self.blink_config.interval * 1000
                + rng.gen_range(0..self.blink_config.variation) * 1000;
        }
    }

    fn process_laugh(&mut self) {
        if self.laugh {
            if self.laugh_toggle {
                self.v_flicker = true;
                self.v_flicker_amplitude = 5;
                self.laugh_timer = self.current_time;
                self.laugh_toggle = false;
            } else if self.current_time >= self.laugh_timer + self.laugh_duration {
                self.v_flicker = false;
                self.v_flicker_amplitude = 0;
                self.laugh_toggle = true;
                self.laugh = false;
            }
        }
    }

    fn process_confused(&mut self) {
        if self.confused {
            if self.confused_toggle {
                self.h_flicker = true;
                self.h_flicker_amplitude = 20;
                self.confused_timer = self.current_time;
                self.confused_toggle = false;
            } else if self.current_time >= self.confused_timer + self.confused_duration {
                self.h_flicker = false;
                self.h_flicker_amplitude = 0;
                self.confused_toggle = true;
                self.confused = false;
            }
        }
    }

    fn process_idle(&mut self) {
        if self.idle && self.current_time >= self.idle_timer {
            let mut rng = rand::thread_rng();
            self.eye_l_x_next = rng.gen_range(0..=self.get_constraint_x());
            self.eye_l_y_next = rng.gen_range(0..=self.get_constraint_x());
            self.idle_timer = self.current_time
                + self.idle_config.interval * 1000
                + rng.gen_range(0..self.idle_config.variation) * 1000;
        }
    }

    fn apply_flicker(&mut self) {
        if self.h_flicker {
            if self.h_flicker_alternate {
                self.eye_l_x += self.h_flicker_amplitude as i32;
                self.eye_r_x += self.h_flicker_amplitude as i32;
            } else {
                self.eye_l_x -= self.h_flicker_amplitude as i32;
                self.eye_r_x -= self.h_flicker_amplitude as i32;
            }
            self.h_flicker_alternate = !self.h_flicker_alternate;
        }

        if self.v_flicker {
            if self.v_flicker_alternate {
                self.eye_l_y += self.v_flicker_amplitude as i32;
                self.eye_r_y += self.v_flicker_amplitude as i32;
            } else {
                self.eye_l_y -= self.v_flicker_amplitude as i32;
                self.eye_r_y -= self.v_flicker_amplitude as i32;
            }
            self.v_flicker_alternate = !self.v_flicker_alternate;
        }
    }

    fn update_mood_transitions(&mut self) {
        match self.mood {
            Mood::Tired => {
                self.eyelids_tired_height_next = self.eye_l_height_default / 2;
                self.eyelids_angry_height_next = 0;
            }
            Mood::Angry => {
                self.eyelids_angry_height_next = self.eye_l_height_default / 2;
                self.eyelids_tired_height_next = 0;
            }
            Mood::Happy => {
                self.eyelids_happy_bottom_offset_next = self.eye_l_height_default / 2;
            }
            Mood::Default => {
                self.eyelids_tired_height_next = 0;
                self.eyelids_angry_height_next = 0;
                self.eyelids_happy_bottom_offset_next = 0;
            }
        }

        self.eyelids_tired_height =
            (self.eyelids_tired_height as f32 + self.eyelids_tired_height_next as f32) as u32 / 2;
        self.eyelids_angry_height =
            (self.eyelids_angry_height as f32 + self.eyelids_angry_height_next as f32) as u32 / 2;
        self.eyelids_happy_bottom_offset = (self.eyelids_happy_bottom_offset as f32
            + self.eyelids_happy_bottom_offset_next as f32)
            as u32
            / 2;
    }

    fn draw_eyelids(&mut self, img: &mut GrayImage) {
        // Tired eyelids
        if self.mood == Mood::Tired {
            if !self.cyclops {
                draw_triangle(
                    img,
                    self.screen_width,
                    self.screen_height,
                    self.eye_l_x,
                    self.eye_l_y - 1,
                    self.eye_l_x + self.eye_l.width as i32,
                    self.eye_l_y - 1,
                    self.eye_l_x,
                    self.eye_l_y + self.eyelids_tired_height as i32 - 1,
                    BGCOLOR,
                );
                draw_triangle(
                    img,
                    self.screen_width,
                    self.screen_height,
                    self.eye_r_x,
                    self.eye_r_y - 1,
                    self.eye_r_x + self.eye_r.width as i32,
                    self.eye_r_y - 1,
                    self.eye_r_x + self.eye_r.width as i32,
                    self.eye_r_y + self.eyelids_tired_height as i32 - 1,
                    BGCOLOR,
                );
            } else {
                draw_triangle(
                    img,
                    self.screen_width,
                    self.screen_height,
                    self.eye_l_x,
                    self.eye_l_y - 1,
                    self.eye_l_x + self.eye_l.width as i32 / 2,
                    self.eye_l_y - 1,
                    self.eye_l_x,
                    self.eye_l_y + self.eyelids_tired_height as i32 - 1,
                    BGCOLOR,
                );
                draw_triangle(
                    img,
                    self.screen_width,
                    self.screen_height,
                    self.eye_l_x + self.eye_l.width as i32 / 2,
                    self.eye_l_y - 1,
                    self.eye_l_x + self.eye_l.width as i32,
                    self.eye_l_y - 1,
                    self.eye_l_x + self.eye_l.width as i32,
                    self.eye_l_y + self.eyelids_tired_height as i32 - 1,
                    BGCOLOR,
                );
            }
        }

        // Angry eyelids
        if self.mood == Mood::Angry {
            if !self.cyclops {
                draw_triangle(
                    img,
                    self.screen_width,
                    self.screen_height,
                    self.eye_l_x,
                    self.eye_l_y - 1,
                    self.eye_l_x + self.eye_l.width as i32,
                    self.eye_l_y - 1,
                    self.eye_l_x + self.eye_l.width as i32,
                    self.eye_l_y + self.eyelids_angry_height as i32 - 1,
                    BGCOLOR,
                );
                draw_triangle(
                    img,
                    self.screen_width,
                    self.screen_height,
                    self.eye_r_x,
                    self.eye_r_y - 1,
                    self.eye_r_x + self.eye_r.width as i32,
                    self.eye_r_y - 1,
                    self.eye_r_x,
                    self.eye_r_y + self.eyelids_angry_height as i32 - 1,
                    BGCOLOR,
                );
            } else {
                draw_triangle(
                    img,
                    self.screen_width,
                    self.screen_height,
                    self.eye_l_x,
                    self.eye_l_y - 1,
                    self.eye_l_x + self.eye_l.width as i32 / 2,
                    self.eye_l_y - 1,
                    self.eye_l_x + self.eye_l.width as i32 / 2,
                    self.eye_l_y + self.eyelids_angry_height as i32 - 1,
                    BGCOLOR,
                );
                draw_triangle(
                    img,
                    self.screen_width,
                    self.screen_height,
                    self.eye_l_x + self.eye_l.width as i32 / 2,
                    self.eye_l_y - 1,
                    self.eye_l_x + self.eye_l.width as i32,
                    self.eye_l_y - 1,
                    self.eye_l_x + self.eye_l.width as i32 / 2,
                    self.eye_l_y + self.eyelids_angry_height as i32 - 1,
                    BGCOLOR,
                );
            }
        }

        // Happy eyelids
        if self.mood == Mood::Happy {
            let left_happy_y = self.eye_l_y + self.eye_l_height_current as i32
                - self.eyelids_happy_bottom_offset as i32
                + 1;
            draw_rounded_rect(
                img,
                self.screen_width,
                self.screen_height,
                self.eye_l_x - 1,
                left_happy_y,
                self.eye_l.width + 2,
                self.eye_l_height_current,
                self.eye_l.border_radius,
                BGCOLOR,
            );

            if !self.cyclops {
                let right_happy_y = self.eye_r_y + self.eye_r_height_current as i32
                    - self.eyelids_happy_bottom_offset as i32
                    + 1;
                draw_rounded_rect(
                    img,
                    self.screen_width,
                    self.screen_height,
                    self.eye_r_x - 1,
                    right_happy_y,
                    self.eye_r.width + 2,
                    self.eye_r_height_current,
                    self.eye_r.border_radius,
                    BGCOLOR,
                );
            }
        }
    }

    fn draw_sweat(&mut self, img: &mut GrayImage) {
        let resets = self.sweat_drops.update();

        for (i, drop) in self.sweat_drops.0.iter_mut().enumerate() {
            if resets.contains(&i) {
                let pos = match i {
                    0 => SweatPosition::Left,
                    1 => SweatPosition::Center,
                    _ => SweatPosition::Right,
                };
                drop.reset(self.screen_width, pos);
            }

            let (x, y, w, h) = drop.params();
            draw_rounded_rect(
                img,
                self.screen_width,
                self.screen_height,
                x,
                y,
                w,
                h,
                3,
                MAINCOLOR,
            );
        }
    }
}

use image::GrayImage;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let eyes = RoboEyes::new(128, 64);
        assert_eq!(eyes.screen_width, 128);
        assert_eq!(eyes.screen_height, 64);
    }

    #[test]
    fn test_set_mood() {
        let mut eyes = RoboEyes::new(128, 64);
        eyes.set_mood(Mood::Happy);
        assert_eq!(eyes.mood, Mood::Happy);
    }

    #[test]
    fn test_set_position() {
        let mut eyes = RoboEyes::new(128, 64);
        eyes.set_position(Position::NorthEast);
    }

    #[test]
    fn test_draw_eyes() {
        let mut eyes = RoboEyes::new(128, 64);
        eyes.set_mood(Mood::Default);
        eyes.open();
        let img = eyes.draw_eyes(1000);
        assert_eq!(img.width(), 128);
        assert_eq!(img.height(), 64);
    }

    #[test]
    fn test_blink() {
        let mut eyes = RoboEyes::new(128, 64);
        eyes.blink();
    }
}
