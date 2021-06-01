mod vectors;
mod ray;
mod objects;

use vectors::{Color3,Point3,Vector3};
use ray::Ray;

use indicatif::{ProgressBar};

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
	let oc: Vector3 = (ray.origin - center.clone()).into();
	let a = ray.direction.length_squared();
	let half_b = oc.dot(&ray.direction);
	let c = oc.length_squared() - radius*radius;
	let discriminant = half_b*half_b - a * c;
	if discriminant < 0.0 {
		-1.0
	} else {
		(-half_b - discriminant.sqrt()) / a
	}
}

fn ray_color(ray: &Ray) -> Color3 {
	let t = hit_sphere(&Point3(0.0,0.0,-1.0), 0.5, ray);
	if t > 0.0 {
		let n = (Vector3::from(ray.at(t)) - Vector3(0.0, 0.0, -1.0)).to_unit();
		return 0.5 * Color3(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
	}
	let unit_dir = ray.direction.to_unit();
	let t = 0.5 * (unit_dir.y() + 1.0);
	(1.0 - t) * Color3(1.0, 1.0, 1.0) + t*Color3(0.5, 0.7, 1.0)
}

fn main() {
	const ASPECT_RATIO: f64 = 16.0 / 9.0;
	const IMAGE_WIDTH: u32 = 400;
	const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

	const VIEWPORT_HEIGHT: f64 = 2.0;
	const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;
	const FOCAL_LENGTH: f64 = 1.0;

	const HORIZONTAL: Vector3 = Vector3(VIEWPORT_WIDTH, 0.0, 0.0);
	const VERTICAL: Vector3 = Vector3(0.0, VIEWPORT_HEIGHT, 0.0);
	const ORIGIN: Point3 = Point3(0.0, 0.0, 0.0);
	let lower_left_corner: Vector3 = Vector3::from(ORIGIN) - HORIZONTAL / 2.0 - VERTICAL / 2.0 - Vector3(0.0, 0.0, FOCAL_LENGTH);

	let bar = ProgressBar::new((IMAGE_HEIGHT * IMAGE_WIDTH).into());

	print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
	for j in (0..IMAGE_HEIGHT).rev() {
		for i in 0..IMAGE_WIDTH {
			let u: f64 = i as f64 / (IMAGE_WIDTH - 1) as f64;
			let v: f64 = j as f64 / (IMAGE_HEIGHT - 1) as f64;
			let ray = Ray::new(ORIGIN, lower_left_corner + u * HORIZONTAL + v * VERTICAL - ORIGIN.into());
			let color = ray_color(&ray);
			print!("{}", color);
		}
		bar.inc(IMAGE_WIDTH.into());
	}
	bar.finish();
}
