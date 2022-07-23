mod camera;
mod color;
mod hittable;
mod ray;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::{Hittable, HittableCollection, Sphere};
use crate::ray::Ray;
use euclid::Point3D;
use image::RgbImage;
use rand::random;

fn ray_color(r: &Ray, hittable: &dyn Hittable) -> Color {
    let hit = hittable.hit(r, 0.001, f32::MAX);
    if let Some(hit) = hit {
        let normal = hit.normal();
        return Color::new(
            0.5 * (1.0 + normal.x),
            0.5 * (1.0 + normal.y),
            0.5 * (1.0 + normal.z),
        );
    }

    let unit_direction = r.direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0).mix(&Color::new(0.5, 0.7, 1.0), t)
}

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 640;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 16;

fn main() {
    let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // Camera
    let camera = Camera::default();

    // World
    let mut world = HittableCollection::new();
    world.add(Box::new(Sphere::new(Point3D::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(
        Point3D::new(0.0, -100.5, -1.0),
        100.0,
    )));

    for j in 0..IMAGE_HEIGHT {
        println!("Rendering line {}/{}", j, IMAGE_HEIGHT);
        for i in 0..IMAGE_WIDTH {
            let mut color = Color::default();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + random::<f32>()) / IMAGE_WIDTH as f32;
                let v = (j as f32 + random::<f32>()) / IMAGE_HEIGHT as f32;
                let ray = camera.get_ray(u, v);
                color += ray_color(&ray, &world);
            }
            img.put_pixel(
                i,
                IMAGE_HEIGHT - j - 1,
                color.desample(SAMPLES_PER_PIXEL).as_image_color(),
            );
        }
    }

    img.save("test.png").unwrap();
}
