use euclid::default::{Point3D, Vector3D};

pub struct Ray {
    origin: Point3D<f32>,
    direction: Vector3D<f32>,
}

impl Ray {
    pub fn new(origin: Point3D<f32>, direction: Vector3D<f32>) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> Point3D<f32> {
        self.origin + self.direction * t
    }

    pub fn origin(&self) -> Point3D<f32> {
        self.origin
    }

    pub fn direction(&self) -> Vector3D<f32> {
        self.direction
    }
}
