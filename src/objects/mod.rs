use super::ray::{Ray};
use super::vectors::{Point3,Vector3};

mod sphere;
pub use sphere::Sphere;

pub struct HitRecord {
	point: Point3,
	normal: Vector3,
	t: f64,
}

pub trait Hittable {
	fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}