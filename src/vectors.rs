#![allow(dead_code)]

use std::fmt::{Debug,Display};
use std::ops::{Add, AddAssign, Sub, Mul};
use derive_more::{Mul as MulDerive, MulAssign, Div as DivDerive, DivAssign, Neg};	

macro_rules! vec3alike {
	($t:ident) => (
		#[derive(MulDerive, MulAssign, DivDerive, DivAssign, Neg, Debug, Copy, Clone, Default)]
		pub struct $t (pub f64, pub f64, pub f64);
		
		impl $t {
			pub fn x(&self) -> f64 { self.0 }
			pub fn y(&self) -> f64 { self.1 }
			pub fn z(&self) -> f64 { self.2 }
		}
		
		impl Add<Self> for $t {
			type Output = Self;
			fn add(self, rhs: Self) -> Self {
				Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
			}
		}
		
		impl AddAssign<Self> for $t {
			fn add_assign(&mut self, rhs: Self) {
				self.0 += rhs.0;
				self.1 += rhs.1;
				self.2 += rhs.2;
			}
		}
		
		impl Sub<Self> for $t {
			type Output = Self;
			fn sub(self, rhs: Self) -> Self {
				Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
			}
		}
		
		impl Mul<Self> for $t {
			type Output = Self;
			fn mul(self, rhs: Self) -> Self {
				Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
			}
		}

		impl Mul<$t> for f64 {
			type Output = $t;
			fn mul(self, rhs: $t) -> $t {
				$t(self * rhs.0, self * rhs.1, self * rhs.2)
			}
		}
	);
}

macro_rules! vec3into {
	($from:ident, $into:ident) => (
		impl From<$from> for $into {
			fn from(item: $from) -> Self {
				Self(item.0, item.1, item.2)
			}
		}
	)
}

vec3alike!(Vector3);
vec3alike!(Color3);
vec3alike!(Point3);

vec3into!(Vector3, Color3);
vec3into!(Vector3, Point3);
vec3into!(Color3, Vector3);
vec3into!(Color3, Point3);
vec3into!(Point3, Vector3);
vec3into!(Point3, Color3);

impl Vector3 {
	pub fn length(&self) -> f64 {
		self.length_squared().sqrt()
	}

	pub fn length_squared(&self) -> f64 {
		self.0 * self.0 + self.1 * self.1 + self.2 * self.2
	}

	pub fn to_unit(&self) -> Self {
		*self / self.length()
	}

	pub fn dot(&self, other: &Self) -> f64 {
		return self.0 * other.0 + self.1 * other.1 + self.2 * other.2;
	}

	pub fn cross(&self, other: &Self) -> Self {
		Self(
			self.1 * other.2 - self.2 * other.1,
			self.2 * other.0 - self.0 * other.2,
			self.0 * other.1 - self.1 * other.0
		)
	}
}

impl Color3 {
	pub fn r(&self) -> f64 { self.0}
	pub fn g(&self) -> f64 { self.1}
	pub fn b(&self) -> f64 { self.2}

	fn ir(&self) -> u8 { (self.0 * 255.999) as u8 }
	fn ig(&self) -> u8 { (self.1 * 255.999) as u8 }
	fn ib(&self) -> u8 { (self.2 * 255.999) as u8 }
}

impl Display for Color3 {
	fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		write!(formatter, "{} {} {}\n", self.ir(), self.ig(), self.ib())
	}
}