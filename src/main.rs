use std::f32::consts::PI;

use macroquad::prelude::*;
use ndarray::{Array2, Zip};

// use wave2d::{GRID_WIDTH, GRID_HEIGHT};

pub const GRID_WIDTH: usize = 128;
pub const GRID_HEIGHT: usize = 128;

pub const DX: f32 = 0.001;
pub const DY: f32 = 0.001;

pub const DT: f32 = 0.01;
pub const C: f32 = 0.01;

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

fn update_acceleration(acceleration: &mut Array2<f32>, value: &Array2<f32>) {
    // assume `accel.dim() == value.dim() == (GRID_WIDTH, GRID_HEIGHT)`
    let (nx, ny) = value.dim();
    // Zip::indexed is ergonomically the same as indexed_iter_mut
    Zip::indexed(acceleration).for_each(|(i, j), a| {
        let im1 = (i + nx - 1) % nx;
        let ip1 = (i + 1) % nx;
        let jm1 = (j + ny - 1) % ny;
        let jp1 = (j + 1) % ny;

        let p00 = value[(i, j)];
        let p01 = value[(im1, j)];
        let p21 = value[(ip1, j)];
        let p10 = value[(i, jm1)];
        let p12 = value[(i, jp1)];

        *a = C * C * ((p01 + p21 - 2.0 * p00) / (DX * DX) + (p10 + p12 - 2.0 * p00) / (DY * DY));
    });
}

fn update_velocity(velocity: &mut Array2<f32>, acceleration: &Array2<f32>, dt: f32) {
    Zip::indexed(velocity).for_each(|(i, j), v| {
        *v += acceleration[(i, j)] * dt;
    });
}

fn update_value(value: &mut Array2<f32>, velocity: &Array2<f32>, dt: f32) {
    Zip::indexed(value).for_each(|(i, j), p| {
        *p += velocity[(i, j)] * dt;
    });
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
    // Initialize a velocity array
    // let mut velocity_array = Array2::from_shape_fn((GRID_WIDTH, GRID_HEIGHT), |(i, j)| 0.01);
    let mut velocity_array: Array2<f32> = Array2::zeros((GRID_WIDTH, GRID_HEIGHT));
    // Initialize an acceleration array
    let mut acceleration_array: Array2<f32> = Array2::zeros((GRID_WIDTH, GRID_HEIGHT));
    update_acceleration(&mut acceleration_array, &value_array);
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

        // Show the FPS
        let fps = get_fps();
        let msg = format!("FPS: {fps}");
        draw_text(&msg, 10.0, 10.0, 15.0, WHITE);

        // Calculate the acceleration
        update_acceleration(&mut acceleration_array, &value_array);

        // Update the velocity based on acceleration (1/2)
        update_velocity(&mut velocity_array, &acceleration_array, DT * 0.5);

        // Update the position based on speed
        update_value(&mut value_array, &velocity_array, DT);

        // Update the velocity based on acceleration (2/2)
        update_velocity(&mut velocity_array, &acceleration_array, DT * 0.5);

        next_frame().await
    }
}
