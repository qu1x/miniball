use miniball::{
	nalgebra::{Point3, Vector3},
	{Ball, Enclosing},
};
use rand::thread_rng;
use rand_distr::{Distribution, UnitSphere};
use std::collections::VecDeque;

fn main() {
	let m = 100_000;
	println!("n = 3, m = {m}");
	println!();
	let center = Vector3::new(-3.0, 7.0, 4.8);
	let radius = 3.0;
	let radius_squared = radius * radius;
	let mut points = UnitSphere
		.sample_iter(&mut thread_rng())
		.take(m)
		.map(Point3::<f64>::from)
		.map(|point| point * radius + center)
		.collect::<VecDeque<_>>();
	let ball = (0..8)
		.map(|_| {
			let ball = Ball::enclosing_points(&mut points);
			let epsilon = ball.radius_squared / radius_squared - 1.0;
			println!("sample with accuracy: 1{epsilon:+.1e}");
			ball
		})
		.min()
		.unwrap();
	println!();
	let epsilon = ball.radius_squared / radius_squared - 1.0;
	println!("result with accuracy: 1{epsilon:+.1e}");
}
