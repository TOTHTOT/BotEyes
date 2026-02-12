//! Example: BotEyes screenshot generation
//!
//! ```cargo run --example screenshot```
//!
//! This generates screenshots of all moods, positions, and animations,
//! saving them to the output directory.

use boteyes::{Mood, Position, RoboEyes};
use image::GrayImage;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create output directory
    fs::create_dir_all("output")?;

    // Create RoboEyes instance (128x64 OLED resolution)
    let mut eyes = RoboEyes::new(128, 64);

    // Create a reusable buffer (more efficient than creating new one each frame)
    let mut buffer = GrayImage::new(128, 64);

    // Animate eyes open first
    eyes.set_mood(Mood::Default);
    eyes.set_position(Position::Center);
    eyes.open();
    for i in 0..20 {
        eyes.draw_into(&mut buffer, i as u64 * 20);
    }

    // Default mode
    eyes.set_mood(Mood::Default);
    eyes.draw_into(&mut buffer, 1000);
    buffer.save("output/default.png")?;
    println!("Saved: output/default.png");

    // Tired mode
    eyes.set_mood(Mood::Tired);
    eyes.draw_into(&mut buffer, 1000);
    buffer.save("output/tired.png")?;
    println!("Saved: output/tired.png");

    // Angry mode
    eyes.set_mood(Mood::Angry);
    eyes.draw_into(&mut buffer, 1000);
    buffer.save("output/angry.png")?;
    println!("Saved: output/angry.png");

    // Happy mode
    eyes.set_mood(Mood::Happy);
    eyes.draw_into(&mut buffer, 1000);
    buffer.save("output/happy.png")?;
    println!("Saved: output/happy.png");

    // Cyclops mode
    eyes.set_mood(Mood::Default);
    eyes.set_cyclops(true);
    eyes.draw_into(&mut buffer, 1000);
    buffer.save("output/cyclops.png")?;
    println!("Saved: output/cyclops.png");

    // All positions
    for pos in [
        Position::North,
        Position::NorthEast,
        Position::East,
        Position::SouthEast,
        Position::South,
        Position::SouthWest,
        Position::West,
        Position::NorthWest,
        Position::Center,
    ]
    .iter()
    {
        eyes.set_cyclops(false);
        eyes.set_mood(Mood::Default);
        eyes.set_position(*pos);
        eyes.draw_into(&mut buffer, 1000);
        buffer.save(format!("output/position_{:?}.png", pos))?;
        println!("Saved: output/position_{:?}.png", pos);
    }

    // Blink animation
    eyes.set_mood(Mood::Default);
    eyes.set_sweat(false);
    eyes.set_cyclops(false);
    eyes.set_position(Position::Center);
    for i in 0..10 {
        eyes.blink();
        eyes.draw_into(&mut buffer, i as u64 * 50);
        buffer.save(format!("output/blink_{}.png", i))?;
        println!("Saved: output/blink_{}.png", i);
    }

    // Confused animation
    eyes.anim_confused();
    for i in 0..10 {
        eyes.draw_into(&mut buffer, i as u64 * 50);
        buffer.save(format!("output/confused_{}.png", i))?;
        println!("Saved: output/confused_{}.png", i);
    }

    // Laugh animation
    eyes.anim_laugh();
    eyes.set_position(Position::Center);
    for i in 0..10 {
        eyes.draw_into(&mut buffer, i as u64 * 50);
        buffer.save(format!("output/laugh_{}.png", i))?;
        println!("Saved: output/laugh_{}.png", i);
    }

    // Sweat animation
    eyes.set_sweat(true);
    eyes.set_position(Position::Center);
    for i in 0..20 {
        eyes.draw_into(&mut buffer, i as u64 * 100);
        buffer.save(format!("output/sweat_{}.png", i))?;
        println!("Saved: output/sweat_{}.png", i);
    }

    println!("\nAll screenshots saved to output/");

    Ok(())
}
