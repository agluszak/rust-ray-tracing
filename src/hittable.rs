use crate::Ray;
use euclid::default::{Point3D, Vector3D};

pub struct HitRecord {
    p: Point3D<f32>,
    normal: Vector3D<f32>,
    t: f32,
    front_face: bool,
}

impl HitRecord {
    pub fn new(normal: Vector3D<f32>, t: f32, ray: &Ray) -> HitRecord {
        let normal = normal.normalize();
        let p = ray.at(t);
        let front_face = normal.dot(ray.direction()) < 0.0;
        let normal = if front_face { normal } else { -normal };
        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }

    pub fn normal(&self) -> Vector3D<f32> {
        self.normal
    }
}

pub struct Sphere {
    center: Point3D<f32>,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3D<f32>, radius: f32) -> Sphere {
        Sphere { center, radius }
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
        Some(HitRecord::new(normal, t, ray))
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HittableCollection {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableCollection {
    pub fn new() -> HittableCollection {
        HittableCollection { list: Vec::new() }
    }

    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
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
