//! Example: BotEyes real-time demonstration
//!
//! ```cargo run --example demo```
//!
//! This opens a window and animates robot eyes in real-time.
//! Press keys to change moods and modes.

use boteyes::{Mood, RoboEyes};
use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};
use std::time::Duration;

const WIDTH: usize = 256;
const HEIGHT: usize = 128;

fn main() {
    let mut eyes = RoboEyes::new(WIDTH as u32, HEIGHT as u32);
    eyes.set_mood(Mood::Default);
    eyes.open();
    // idle mode: enabled, interval=2s, variation=4s, x_range=100%, y_range=100%
    eyes.set_idle_mode(true, 2, 4, 50, 50);

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "BotEyes Demo - Press keys: 1=Default, 2=Tired, 3=Angry, 4=Happy, C=Cyclops, S=Sweat, SPACE=Blink, ESC=Quit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X4,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to open window");

    let mut frame_count = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Handle keyboard input
        for key in window.get_keys_pressed(KeyRepeat::No) {
            match key {
                Key::Key1 => eyes.set_mood(Mood::Default),
                Key::Key2 => eyes.set_mood(Mood::Tired),
                Key::Key3 => eyes.set_mood(Mood::Angry),
                Key::Key4 => eyes.set_mood(Mood::Happy),
                Key::C => eyes.set_cyclops(!eyes.is_cyclops()),
                Key::S => eyes.set_sweat(!eyes.has_sweat()),
                Key::Space => eyes.blink(),
                _ => {}
            }
        }

        let current_time = frame_count * 16; // ~60 FPS

        // Draw frame and convert to buffer
        let img = eyes.draw_eyes(current_time);
        for (i, pixel) in img.pixels().enumerate() {
            let gray = pixel.0[0];
            let color = if gray > 128 {
                0xFFFFFFFF // White
            } else {
                0xFF444444 // Dark gray background
            };
            buffer[i] = color;
        }

        // Update window
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .expect("Failed to update buffer");

        frame_count += 1;
        std::thread::sleep(Duration::from_millis(16));
    }

    println!("Demo closed.");
}
