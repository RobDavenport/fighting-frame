use std::sync::OnceLock;

use glam::Vec3A;

pub const TEXTURE_WIDTH: usize = 64;
pub const TEXTURE_HEIGHT: usize = 64;

const R_SCALE: f32 = 20.0;
const G_SCALE: f32 = 24.0;
const B_SCALE: f32 = 16.0;

pub fn generate_texture() -> &'static [u8] {
    const BYTES_PER_PIXEL: usize = 4; // RGBA
    const TEXTURE_SIZE: usize = TEXTURE_WIDTH * TEXTURE_HEIGHT * BYTES_PER_PIXEL;

    static TEXTURE: OnceLock<[u8; TEXTURE_SIZE]> = OnceLock::new();

    TEXTURE
        .get_or_init(|| {
            let mut data = [0u8; TEXTURE_SIZE];

            for y in 0..TEXTURE_HEIGHT {
                for x in 0..TEXTURE_WIDTH {
                    // Normalized coordinates
                    let nx = x as f32 / (TEXTURE_WIDTH - 1) as f32;
                    let ny = y as f32 / (TEXTURE_HEIGHT - 1) as f32;

                    // Create color patterns using trigonometric functions
                    let r = ((nx * R_SCALE).sin() * 127.0 + 128.0) as u8;
                    let g = ((ny * G_SCALE).cos() * 127.0 + 128.0) as u8;
                    let b = (((nx + ny) * B_SCALE).sin() * 127.0 + 128.0) as u8;

                    // Calculate the index in the byte array
                    let index = (y * TEXTURE_WIDTH + x) * BYTES_PER_PIXEL;
                    data[index] = r;
                    data[index + 1] = g;
                    data[index + 2] = b;
                    data[index + 3] = 255;
                }
            }
            data
        })
        .as_slice()
}

pub fn generate_matcap_bytes(size: u32) -> Vec<u8> {
    let mut pixels = Vec::with_capacity((size * size * 4) as usize);
    let center = (size as f32 / 2.0, size as f32 / 2.0);
    let radius = size as f32 / 2.0;
    let inv_radius = 1.0 / radius;

    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - center.0;
            let dy = y as f32 - center.1;
            let distance_sq = dx * dx + dy * dy;

            if distance_sq > radius * radius {
                pixels.extend(&[0, 0, 0, 0]);
                continue;
            }

            // Normalized coordinates with flipped Y
            let nx = dx * inv_radius;
            let ny = -dy * inv_radius;
            let nz = (1.0 - nx * nx - ny * ny).sqrt();

            // Base color with dynamic pattern
            let angle = nx.atan2(ny) * 2.0;
            let stripe = (angle * 3.0).sin().signum() * 0.3 + 0.7;
            let base_color = Vec3A::new(
                (nx * 5.0).sin().abs() * 0.5 + 0.25,
                (ny * 5.0).cos().abs() * 0.4 + 0.3,
                stripe * 0.8,
            );

            // Toon shading with stepped lighting
            let light_dir = Vec3A::new(0.5, -0.5, 0.7).normalize();
            let diffuse = nx * light_dir.x + ny * light_dir.y + nz * light_dir.z;
            let toon_diffuse = (diffuse * 4.0).floor() / 4.0;

            // Sharp specular
            let reflect_dir = 2.0 * diffuse * Vec3A::new(nx, ny, nz) - light_dir;
            let specular = reflect_dir.z.max(0.0).powf(32.0).step(0.8);

            // Rim lighting effect
            let rim = (1.0 - nz).powi(4) * 0.5;

            // Outline detection
            let outline = (distance_sq.sqrt() > radius - 2.0) as u8 as f32;

            // Combine all elements
            let mut color = base_color * (toon_diffuse * 0.8 + 0.3)
                + Vec3A::splat(rim) * Vec3A::new(0.8, 0.9, 1.0)
                + Vec3A::new(1.0, 0.9, 0.7) * specular;

            // Apply outline (black border)
            color = color * (1.0 - outline) + Vec3A::ZERO * outline;

            // Convert to RGBA
            pixels.push((color.x.min(1.0).max(0.0) * 255.0) as u8);
            pixels.push((color.y.min(1.0).max(0.0) * 255.0) as u8);
            pixels.push((color.z.min(1.0).max(0.0) * 255.0) as u8);
            pixels.push(255);
        }
    }

    pixels
}

trait StepExt {
    fn step(&self, edge: f32) -> f32;
}

impl StepExt for f32 {
    fn step(&self, edge: f32) -> f32 {
        if *self >= edge { 1.0 } else { 0.0 }
    }
}
