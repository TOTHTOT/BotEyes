//! Drawing module - Graphics primitives
//!
//! Contains functions for drawing shapes on grayscale images:
//! - Rounded rectangles
//! - Filled triangles

use image::GrayImage;

/// Color for drawing (0 = black background, 255 = white foreground)
type Color = u8;

/// Draw a filled rounded rectangle
///
/// Creates a rectangle with rounded corners on the given image.
///
/// # Arguments
///
/// * `img` - Image buffer to draw on
/// * `screen_width` - Width of the display
/// * `screen_height` - Height of the display
/// * `x` - X coordinate of top-left corner
/// * `y` - Y coordinate of top-left corner
/// * `width` - Width of the rectangle
/// * `height` - Height of the rectangle
/// * `radius` - Corner radius
/// * `color` - Fill color (0-255)
///
/// # Notes
///
/// The corner radius is automatically limited to half of the
/// smaller dimension to prevent invalid shapes.
pub fn draw_rounded_rect(
    img: &mut GrayImage,
    screen_width: u32,
    screen_height: u32,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    radius: u32,
    color: Color,
) {
    let radius = radius.min(width / 2).min(height / 2);

    for dy in 0..height as i32 {
        for dx in 0..width as i32 {
            let px = x + dx;
            let py = y + dy;

            // Skip out of bounds pixels
            if px < 0 || px >= screen_width as i32 || py < 0 || py >= screen_height as i32 {
                continue;
            }

            // Check if point is inside the rounded corner
            if is_in_rounded_corner(dx, dy, width, height, radius) {
                continue;
            }

            img.put_pixel(px as u32, py as u32, image::Luma([color]));
        }
    }
}

/// Check if a point is inside a rounded corner
fn is_in_rounded_corner(dx: i32, dy: i32, width: u32, height: u32, radius: u32) -> bool {
    let radius = radius as i32;
    let width = width as i32;
    let height = height as i32;

    // Top-left corner
    if dx < radius && dy < radius {
        let cx = radius - dx;
        let cy = radius - dy;
        return (cx * cx + cy * cy) > (radius * radius);
    }

    // Top-right corner
    if dx >= width - radius && dy < radius {
        let cx = dx - (width - radius);
        let cy = radius - dy;
        return (cx * cx + cy * cy) > (radius * radius);
    }

    // Bottom-left corner
    if dx < radius && dy >= height - radius {
        let cx = radius - dx;
        let cy = dy - (height - radius);
        return (cx * cx + cy * cy) > (radius * radius);
    }

    // Bottom-right corner
    if dx >= width - radius && dy >= height - radius {
        let cx = dx - (width - radius);
        let cy = dy - (height - radius);
        return (cx * cx + cy * cy) > (radius * radius);
    }

    false
}

/// Draw a filled triangle
///
/// Uses barycentric coordinates to determine which pixels are inside
/// the triangle.
///
/// # Arguments
///
/// * `img` - Image buffer to draw on
/// * `screen_width` - Width of the display
/// * `screen_height` - Height of the display
/// * `x1, y1` - First vertex
/// * `x2, y2` - Second vertex
/// * `x3, y3` - Third vertex
/// * `color` - Fill color (0-255)
pub fn draw_triangle(
    img: &mut GrayImage,
    screen_width: u32,
    screen_height: u32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    x3: i32,
    y3: i32,
    color: Color,
) {
    // Calculate bounding box
    let min_x = x1.min(x2).min(x3).max(0);
    let max_x = x1.max(x2).max(x3).min(screen_width as i32 - 1);
    let min_y = y1.min(y2).min(y3).max(0);
    let max_y = y1.max(y2).max(y3).min(screen_height as i32 - 1);

    // Edge vectors from vertex 1
    let edge1_x = x2 - x1;
    let edge1_y = y2 - y1;
    let edge2_x = x3 - x1;
    let edge2_y = y3 - y1;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            // Barycentric coordinates
            let px = x - x1;
            let py = y - y1;

            let det = edge1_x * edge2_y - edge2_x * edge1_y;
            if det == 0 {
                continue;
            }

            let u = (px * edge2_y - py * edge2_x) as f32 / det as f32;
            let v = (edge1_x * py - edge1_y * px) as f32 / det as f32;

            if u >= 0.0 && v >= 0.0 && u + v <= 1.0 {
                img.put_pixel(x as u32, y as u32, image::Luma([color]));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corner_detection() {
        // Corner point (0,0) should be inside the rounded corner
        assert!(is_in_rounded_corner(0, 0, 10, 10, 5));

        // Center point should NOT be inside any corner
        assert!(!is_in_rounded_corner(5, 5, 10, 10, 5));

        // Point in the middle of the rectangle should NOT be in a corner
        assert!(!is_in_rounded_corner(4, 4, 10, 10, 5));
    }

    #[test]
    fn test_triangle_bounding_box() {
        let mut img = GrayImage::new(100, 100);

        // Draw a triangle in the middle
        draw_triangle(&mut img, 100, 100, 20, 20, 80, 20, 50, 80, 255);

        // Corners should be empty
        assert_eq!(img.get_pixel(0, 0)[0], 0);
        assert_eq!(img.get_pixel(99, 0)[0], 0);
        assert_eq!(img.get_pixel(0, 99)[0], 0);

        // Center should be filled
        assert_eq!(img.get_pixel(50, 50)[0], 255);
    }
}
