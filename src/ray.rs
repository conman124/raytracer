#![allow(dead_code)]

use super::vectors::{Vector3,Point3};

#[derive(Copy,Clone,Default)]
pub struct Ray {
	pub origin: Point3,
	pub direction: Vector3,
}

impl Ray {
	pub fn new(origin: Point3, direction: Vector3) -> Ray {
		Ray{origin, direction}
	}

	pub fn at(&self, t: f64) -> Point3 {
		self.origin + (t * self.direction).into()
	}
}