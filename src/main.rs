mod algebra;
mod camera;
mod color;
mod hittable;
mod material;
mod ray;

use std::ops::Deref;

use crate::algebra::{Vector, VectorExt};
use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::{Hittable, HittableCollection, Sphere};
use crate::ray::Ray;
use euclid::Point3D;
use image::RgbImage;
use rand::random;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn ray_color(r: &Ray, hittable: &dyn Hittable, depth: u32) -> Color {
    if depth == 0 {
        return Color::default();
    }

    let hit = hittable.hit(r, 0.001, f32::MAX);
    if let Some(hit) = hit {
        if let Some((scattered, attenuation)) = hit.material.scatter(r, &hit) {
            return attenuation * ray_color(&scattered, hittable, depth - 1);
        }
        return Color::default();
    }

    let unit_direction = r.direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0).mix(&Color::new(0.5, 0.7, 1.0), t)
}

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 640;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 16;
const MAX_DEPTH: u32 = 100;

fn main() {
    let img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // Camera
    let camera = Camera::default();

    // World
    let material_ground = Arc::new(material::Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(material::Dielectric::new(1.5));
    let material_left = Arc::new(material::Dielectric::new(1.5));
    let material_right = Arc::new(material::Metal::new(Color::new(0.8, 0.6, 0.2), 0.5));

    let mut world = HittableCollection::new();
    world.add(Box::new(Sphere::new(
        Point3D::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3D::new(0.0, 0.0, -1.0),
        0.3,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3D::new(-1.0, 0.0, -1.0),
        0.7,
        material_left.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3D::new(-1.0, 0.0, -1.0),
        -0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3D::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));
    let world = Arc::new(world);

    let img = Mutex::new(img);

    let time = Instant::now();

    for j in 0..IMAGE_HEIGHT {
        println!("Rendering line {}/{}", j, IMAGE_HEIGHT);
        let v = (0..IMAGE_WIDTH)
            .map(|i| (i, world.clone()))
            .collect::<Vec<_>>();
        v.into_par_iter().for_each(|(i, world)| {
            let mut color = Color::default();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + random::<f32>()) / IMAGE_WIDTH as f32;
                let v = (j as f32 + random::<f32>()) / IMAGE_HEIGHT as f32;
                let ray = camera.get_ray(u, v);
                color += ray_color(&ray, world.deref(), MAX_DEPTH);
            }
            img.lock().unwrap().put_pixel(
                i,
                IMAGE_HEIGHT - j - 1,
                color
                    .desample(SAMPLES_PER_PIXEL)
                    .gamma_correct()
                    .as_image_color(),
            );
        });
    }

    println!("Rendering time: {}s", time.elapsed().as_secs_f32());

    img.lock().unwrap().save("test.png").unwrap();
}
