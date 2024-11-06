use std::fmt::{Display};
use image::GenericImageView;

#[derive(Debug)]
pub struct Tower {
    tower_type: TowerType,
    position: Position,
    level: usize,
    perks: Vec<Option<u32>>,

    damage: usize,
    attack_speed: usize,
    range: usize,
}


impl Tower {
    pub fn new_archer(position: Position) -> Self {
        Tower { tower_type: TowerType::Archer, position, level: 1, perks: vec![None, None, None], damage: 2, attack_speed: 4, range: 7 }
    }

    pub fn new_soldier(position: Position) -> Self {
        Tower { tower_type: TowerType::Soldier, position, level: 1, perks: vec![None, None, None], damage: 4, attack_speed: 2, range: 3 }
    }

    pub fn render_tower(&self, frame: &mut [u8], width: u32, height: u32) {
        let width = width as usize;
        let height = height as usize;

        // Load the original image and get its dimensions
        let image = image::open("assets/1.png").unwrap();
        let rgba_image = image.to_rgba8();
        let (orig_image_width, orig_image_height) = rgba_image.dimensions();
        let orig_image_width = orig_image_width as usize;
        let orig_image_height = orig_image_height as usize;
        let pixels = rgba_image.into_raw();


        // Determine the scaling factor to fit the image in the window
        let scale_x = width as f32 / orig_image_width as f32;
        let scale_y = height as f32 / orig_image_height as f32;
        let scale = scale_x.min(scale_y) / 4.0;

        // Calculate the scaled image dimensions
        let scaled_width = (orig_image_width as f32 * scale) as usize;
        let scaled_height = (orig_image_height as f32 * scale) as usize;

        // Center the scaled image
        let (center_x, center_y) = self.position.to_pixel(width, height);
        let start_x = center_x.saturating_sub(scaled_width / 2);
        let start_y = center_y.saturating_sub(scaled_height / 2);

        // Draw the scaled image by mapping scaled coordinates back to the original image
        for y in 0..scaled_height {
            for x in 0..scaled_width {
                // Calculate the corresponding original image coordinates
                let orig_x = ((x as f32 / scaled_width as f32) * orig_image_width as f32) as usize;
                let orig_y = ((y as f32 / scaled_height as f32) * orig_image_height as f32) as usize;

                // Get the index for the original image and the frame buffer
                let image_idx = (orig_y * orig_image_width + orig_x) * 4;
                let frame_idx = ((start_y + y) * width + (start_x + x)) * 4;

                // Copy pixel from original image to frame
                frame[frame_idx] = pixels[image_idx];         // Red
                frame[frame_idx + 1] = pixels[image_idx + 1]; // Green
                frame[frame_idx + 2] = pixels[image_idx + 2]; // Blue
                frame[frame_idx + 3] = pixels[image_idx + 3]; // Alpha
            }
        }
    }
}


#[derive(Debug)]
enum TowerType {
    Archer,
    Inferno,
    Soldier,
    Tesla,
    Ice,
}

#[derive(Debug)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }

    pub fn to_pixel(&self, width: usize, height: usize) -> (usize, usize) {
        let x = self.x as f32 / 100.0 * width as f32;
        let y = self.y as f32 / 100.0 * height as f32;

        (x as usize, y as usize)
    }
}

