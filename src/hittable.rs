use crate::Ray;
use std::sync::Arc;

use crate::algebra::{Point, Vector};
use crate::material::Material;

pub struct HitRecord {
    pub p: Point,
    pub normal: Vector,
    pub t: f32,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(normal: Vector, t: f32, ray: &Ray, material: Arc<dyn Material>) -> HitRecord {
        let normal = normal.normalize();
        let p = ray.at(t);
        let front_face = normal.dot(ray.direction()) < 0.0;
        let normal = if front_face { normal } else { -normal };
        HitRecord {
            p,
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub struct Sphere {
    center: Point,
    radius: f32,
    material: Arc<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(center: Point, radius: f32, material: Arc<dyn Material + Send + Sync>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().square_length();
        let half_b = oc.dot(ray.direction());
        let c = oc.square_length() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_discriminant = discriminant.sqrt();
        let mut root = (-half_b - sqrt_discriminant) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_discriminant) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let t = root;
        let p = ray.at(t);
        let normal = (p - self.center) / self.radius;
        Some(HitRecord::new(normal, t, ray, self.material.clone()))
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HittableCollection {
    list: Vec<Box<dyn Hittable + Send + Sync>>,
}

impl HittableCollection {
    pub fn new() -> HittableCollection {
        HittableCollection { list: Vec::new() }
    }

    pub fn add(&mut self, hittable: Box<dyn Hittable + Send + Sync>) {
        self.list.push(hittable);
    }
}

impl Hittable for HittableCollection {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_record = None;
        for hittable in &self.list {
            if let Some(hit_record_) = hittable.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit_record_.t;
                hit_record = Some(hit_record_);
            }
        }
        hit_record
    }
}
