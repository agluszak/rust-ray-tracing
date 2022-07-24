use crate::algebra::Scalar;
use crate::hittable::HitRecord;
use crate::{Color, Ray, Vector, VectorExt};
use rand::random;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    // Note we could just as well only scatter with some probability p and have attenuation be albedo/p
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit.normal + Vector::random_in_unit_sphere();

        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

        let scattered = Ray::new(hit.p, scatter_direction);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: Scalar,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: Scalar) -> Metal {
        assert!((0.0..=1.0).contains(&fuzz));
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = ray.direction().reflect(hit.normal).normalize();
        let scattered = Ray::new(
            hit.p,
            reflected + Vector::random_in_unit_sphere() * self.fuzz,
        );
        let attenuation = self.albedo;
        if scattered.direction().dot(hit.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refraction_index: Scalar,
}

impl Dielectric {
    pub fn new(refraction_index: Scalar) -> Dielectric {
        Dielectric { refraction_index }
    }

    fn reflectance(&self, cosine: Scalar) -> Scalar {
        // Use Schlick's approximation to calculate the reflectance
        let r0 = (1.0 - self.refraction_index) / (1.0 + self.refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let direction = ray.direction().normalize();
        let cos_theta = (-direction).dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let can_refract = refraction_ratio * sin_theta <= 1.0;

        let result_direction = if !can_refract || random::<Scalar>() < self.reflectance(cos_theta) {
            direction.reflect(hit.normal)
        } else {
            direction.refract(hit.normal, refraction_ratio)
        };

        let scattered = Ray::new(hit.p, result_direction);
        Some((scattered, attenuation))
    }
}
