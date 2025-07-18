use std::f32::consts::PI;

use macroquad::prelude::*;
use ndarray::Array2;

// use wave2d::{GRID_WIDTH, GRID_HEIGHT};

pub const GRID_WIDTH: usize = 128;
pub const GRID_HEIGHT: usize = 128;

fn window_conf() -> Conf {
    // Set the initial window properties
     let mut conf = Conf {
        window_title: "wave2d".to_string(),
        window_width: GRID_WIDTH as i32,
        window_height: GRID_HEIGHT as i32,
        ..Default::default()
    };
    // turn vsync off
    conf.platform.swap_interval = Some(0);
    conf
}

fn get_color(p: f32) -> Color {
    let color1 = Color {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    let color2 = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    Color {
        r: p * color1.r + (1.0 - p) * color2.r,
        g: p * color1.g + (1.0 - p) * color2.g,
        b: p * color1.b + (1.0 - p) * color2.b,
        a: 1.0,
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    // Initialize a 2D f32 array for main value
    let mut value_array = Array2::from_shape_fn((GRID_WIDTH, GRID_HEIGHT), |(i, j)| {
        let x = i as f32;
        let y = j as f32;
        let width: f32 = GRID_WIDTH as f32;
        let height: f32 = GRID_HEIGHT as f32;
        0.5 * ((x * 2.0 * PI / width).sin() * (y * 2.0 * PI / height).sin()) + 0.5
    });
    let mut velocity_array = Array2::from_shape_fn((GRID_WIDTH, GRID_HEIGHT), |(i, j)| {
        0.01
    });
    // Initialize a velocity array
    // Initialize the image for rendering
    let mut image = Image::gen_image_color(GRID_WIDTH as u16, GRID_HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);
    
    // Main game loop
    loop {
        // Draw pixels to the texture based on value
        for i in 0..GRID_WIDTH {
            for j in 0..GRID_HEIGHT {
                let p = value_array[(i, j)];
                let color = get_color(p);
                image.set_pixel(i as u32, j as u32, color);
            }
        }
        texture.update(&image);

        // Render the texture
        clear_background(BLACK);
        draw_texture(&texture, 0.0, 0.0, WHITE);

        // TODO: Show the FPS
        let fps = get_fps();
        let msg = format!("FPS: {}", fps);
        draw_text(&msg, 10.0, 10.0, 15.0, WHITE);

        // TODO: Calculate the acceleration
        // TODO: Update the speed based on acceleration (1/2)

        // Update the position based on speed
        for i in 0..GRID_WIDTH {
            for j in 0..GRID_HEIGHT {
                let v = velocity_array[(i, j)];
                value_array[(i, j)] += v;
            }
        }

        // TODO: Update the speed based on acceleration (1/2)

        next_frame().await
    }
}
