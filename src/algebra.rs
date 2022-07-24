use euclid::default::{Point3D, Vector3D};

use rand::Rng;

pub type Scalar = f32;
pub type Vector = Vector3D<Scalar>;
pub type Point = Point3D<Scalar>;

pub trait VectorExt {
    fn random() -> Self;
    fn random_in_range(min: Scalar, max: Scalar) -> Self;
    fn random_in_unit_sphere() -> Self;
    fn near_zero(&self) -> bool;
    fn reflect(&self, normal: Self) -> Self;
    fn refract(&self, normal: Self, eta: Scalar) -> Self;
}

impl VectorExt for Vector {
    fn random() -> Vector {
        let mut rng = rand::thread_rng();
        let x = rng.gen::<Scalar>();
        let y = rng.gen::<Scalar>();
        let z = rng.gen::<Scalar>();
        Vector::new(x, y, z)
    }

    fn random_in_range(min: Scalar, max: Scalar) -> Vector {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(min..max);
        let y = rng.gen_range(min..max);
        let z = rng.gen_range(min..max);
        Vector::new(x, y, z)
    }

    fn random_in_unit_sphere() -> Vector {
        let _rng = rand::thread_rng();
        let mut point;
        loop {
            point = Vector::random_in_range(-1.0, 1.0);
            if point.square_length() < 1.0 {
                break;
            }
        }
        point.normalize()
    }

    fn near_zero(&self) -> bool {
        const EPSILON: Scalar = 1e-8;
        self.x.abs() < EPSILON && self.y.abs() < EPSILON && self.z.abs() < EPSILON
    }

    fn reflect(&self, normal: Vector) -> Vector {
        *self - normal * 2.0 * self.dot(normal)
    }

    fn refract(&self, normal: Vector, eta: Scalar) -> Vector {
        let cos_theta = self.dot(-normal).min(1.0);
        let r_out_perpendicular = (*self + normal * cos_theta) * eta;
        let r_out_parallel = -normal * (1.0 - r_out_perpendicular.square_length()).abs().sqrt();
        r_out_perpendicular + r_out_parallel
    }
}
