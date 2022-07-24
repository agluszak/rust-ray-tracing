use crate::algebra::{Point, Vector};
use crate::Ray;
use euclid::default::{Point3D, Vector3D};

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vector,
    vertical: Vector,
}

impl Camera {
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let direction =
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin;
        Ray::new(self.origin, direction)
    }
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3D::new(0.0, 0.0, 0.0);
        let horizontal = Vector3D::new(viewport_width, 0.0, 0.0);
        let vertical = Vector3D::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vector3D::new(0.0, 0.0, focal_length);
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
}
