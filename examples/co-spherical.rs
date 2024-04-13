use miniball::{
	nalgebra::{Point3, Vector3},
	{Ball, Enclosing},
};
use rand::{seq::SliceRandom, thread_rng};
use rand_distr::{Distribution, UnitSphere};
use std::collections::VecDeque;

type T = f64;

fn main() {
	// The epsilon chosen in [`Enclosing::contains()`] for [`Ball`].
	//
	// It should be large enough to prevent duplicate bounds.
	let epsilon = T::EPSILON.sqrt();

	let mut rng = thread_rng();

	let m = 100_000;
	let s = 8;
	let center = Vector3::new(-3.0, 7.0, 4.8);
	let inner_radius = 3.0 - epsilon;
	let inner = UnitSphere
		.sample_iter(&mut rng)
		.take(m)
		.map(Point3::from)
		.map(|point| point * inner_radius + center)
		.collect::<Vec<_>>();
	let outer_radius = 3.0;
	let outer = UnitSphere
		.sample_iter(&mut rng)
		.take(m)
		.map(Point3::from)
		.map(|point| point * outer_radius + center)
		.collect::<Vec<_>>();
	let mut inner_then_outer = VecDeque::new();
	inner_then_outer.extend(inner.iter().cloned());
	inner_then_outer.extend(outer.iter().cloned());
	let mut outer_then_inner = VecDeque::new();
	outer_then_inner.extend(outer.iter().cloned());
	outer_then_inner.extend(inner.iter().cloned());
	let mut random = Vec::new();
	random.extend(inner.iter().cloned());
	random.extend(outer.iter().cloned());
	random.shuffle(&mut rng);
	let mut random = VecDeque::from(random);

	println!("Demonstrates the accuracy depending on the order of points.");
	println!("Takes minimum of {s} samples permuted by move-to-front heuristic.");
	println!("Rerun multiple times to see the accuracy changing for random points.");
	println!();
	println!("On-surface tolerance: 1{epsilon:+.1e}");
	println!();
	println!("n = 3, m = {m}, inner-then-outer (worst accuracy)");
	println!();
	let radius_squared = outer_radius * outer_radius;
	let ball = (0..s)
		.map(|_| {
			let ball = Ball::enclosing_points(&mut inner_then_outer);
			let epsilon = ball.radius_squared / radius_squared - 1.0;
			println!("Sample with accuracy: 1{epsilon:+.1e}");
			ball
		})
		.min()
		.unwrap();
	println!();
	let epsilon = ball.radius_squared / radius_squared - 1.0;
	println!("Result with accuracy: 1{epsilon:+.1e}");

	println!();
	println!("n = 3, m = {m}, outer-then-inner (best accuracy)");
	println!();
	let radius_squared = outer_radius * outer_radius;
	let ball = (0..s)
		.map(|_| {
			let ball = Ball::enclosing_points(&mut outer_then_inner);
			let epsilon = ball.radius_squared / radius_squared - 1.0;
			println!("Sample with accuracy: 1{epsilon:+.1e}");
			ball
		})
		.min()
		.unwrap();
	println!();
	let epsilon = ball.radius_squared / radius_squared - 1.0;
	println!("Result with accuracy: 1{epsilon:+.1e}");

	println!();
	println!("n = 3, m = {m}, random-inner/outer (from worst to best accuracy)");
	println!();
	let radius_squared = outer_radius * outer_radius;
	let ball = (0..s)
		.map(|_| {
			let ball = Ball::enclosing_points(&mut random);
			let epsilon = ball.radius_squared / radius_squared - 1.0;
			println!("Sample with accuracy: 1{epsilon:+.1e}");
			ball
		})
		.min()
		.unwrap();
	println!();
	let epsilon = ball.radius_squared / radius_squared - 1.0;
	println!("Result with accuracy: 1{epsilon:+.1e}");
}
