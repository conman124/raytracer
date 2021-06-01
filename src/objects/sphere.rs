use crate::ray::{Ray};
use crate::vectors::{Point3,Vector3};
use super::{Hittable,HitRecord};

pub struct Sphere {
	center: Point3,
	radius: f64,
}

impl Hittable for Sphere {
	fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
		let oc : Vector3 = (ray.origin - self.center).into();
		let a = ray.direction.length_squared();
		let half_b = oc.dot(&ray.direction);
		let c = oc.length_squared() - self.radius*self.radius;

		let discriminant = half_b * half_b - a * c;
		if discriminant < 0.0 { return None; }

		let sqrtd = discriminant.sqrt();
		let root = (-half_b - sqrtd) / a;
		if root < t_min || root > t_max {
			let root = (-half_b + sqrtd) / a;
			if root < t_min || root > t_max {
				return None;
			}
		}

		let hit = HitRecord {
			point: ray.at(root),
			t: root,
			normal: ((ray.at(root) - self.center) / self.radius).into()
		};

		Some(hit)
	}
}