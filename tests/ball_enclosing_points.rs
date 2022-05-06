// Copyright © 2022 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![allow(clippy::float_cmp)]

use miniball::{Ball, Enclosing};
use nalgebra::{
	distance, Point, Point1, Point2, Point3, Point6, Vector1, Vector2, Vector3, Vector6,
};
use std::collections::VecDeque;

#[test]
fn minimum_0_ball_enclosing_bounds() {
	let a = Point::<f64, 0>::origin();
	let Ball {
		center,
		radius_squared,
	} = Ball::enclosing_points(&mut [a].into_iter().collect::<VecDeque<_>>());
	assert_eq!(center, a);
	assert_eq!(radius_squared, 0.0);
}

#[test]
fn minimum_1_ball_enclosing_bounds() {
	let offset = Vector1::new(7.0);
	let a = Point1::new(1.0);
	let b = Point1::new(-1.0);
	let Ball {
		center,
		radius_squared,
	} = Ball::enclosing_points(
		&mut [a, b]
			.map(|bound| bound * 3.0)
			.map(|bound| bound + offset)
			.into_iter()
			.collect::<VecDeque<_>>(),
	);
	assert_eq!(center, offset.into());
	assert_eq!(radius_squared, 9.0);
}

#[test]
fn minimum_2_ball_enclosing_bounds() {
	let offset = Vector2::new(-3.0, 7.0);
	let a = Point2::new(-1.0, 0.0);
	let b = Point2::new(1.0, 0.0);
	let c = Point2::new(0.0, 1.0);
	let Ball {
		center,
		radius_squared,
	} = Ball::enclosing_points(
		&mut [a, b, c]
			.map(|bound| bound * 3.0)
			.map(|bound| bound + offset)
			.into_iter()
			.collect::<VecDeque<_>>(),
	);
	assert_eq!(center, offset.into());
	assert_eq!(radius_squared, 9.0);
}

#[test]
fn minimum_3_ball_enclosing_bounds() {
	let offset = Vector3::new(-3.0, 7.0, 4.8);
	let a = Point3::new(1.0, 1.0, 1.0);
	let b = Point3::new(1.0, -1.0, -1.0);
	let c = Point3::new(-1.0, 1.0, -1.0);
	let d = Point3::new(-1.0, -1.0, 1.0);
	let Ball {
		center,
		radius_squared,
	} = Ball::enclosing_points(
		&mut [a, b, c, d]
			.map(|bound| bound + offset)
			.into_iter()
			.collect::<VecDeque<_>>(),
	);
	assert_eq!(center, offset.into());
	assert_eq!(radius_squared, 3.0);
}

#[test]
fn minimum_3_ball_enclosing_3_line() {
	let offset = Vector3::new(-3.0, 7.0, 4.8);
	let a = Point3::new(-1.0, 0.0, 0.0);
	let b = Point3::new(-0.5, 0.0, 0.0);
	let c = Point3::new(0.5, 0.0, 0.0);
	let d = Point3::new(1.0, 0.0, 0.0);
	let Ball {
		center,
		radius_squared,
	} = Ball::enclosing_points(
		&mut [a, b, c, d]
			.map(|bound| bound * 3.0)
			.map(|bound| bound + offset)
			.into_iter()
			.collect::<VecDeque<_>>(),
	);
	assert_eq!(center, offset.into());
	assert_eq!(radius_squared, 9.0);
}

#[test]
fn minimum_6_ball_enclosing_6_cube() {
	for _randomize in 0..100 {
		// Uniform distribution in 4-cube centered around `offset` with room `diagonal_halved`.
		let offset = Vector6::new(-3.0, 7.0, 4.8, 1.2, 5.3, 7.4);
		let diagonal_halved = 3.0;
		let mut points = (0..10_000)
			.map(|_point| Point6::<f64>::from(Vector6::new_random() - Vector6::from_element(0.5)))
			.map(|point| point * diagonal_halved)
			.map(|point| point + offset)
			.collect::<VecDeque<_>>();
		for _reuse in 0..10 {
			// Computes 4-ball enclosing 4-cube.
			let Ball {
				center,
				radius_squared,
			} = Ball::enclosing_points(&mut points);
			let radius = radius_squared.sqrt();
			// Ensures enclosing 4-ball is roughly centered around uniform distribution in 4-cube
			// and radius roughly matches room diagonal halved, guaranteeing certain uniformity of
			// randomly distributed points.
			assert!((center - offset).map(f64::abs) < Vector6::from_element(1.0).into());
			assert!((radius - diagonal_halved).abs() < 1.0);
			// Epsilon of numeric stability for computing circumscribed 4-ball. This is related to
			// robustness of `Enclosing::with_bounds()` regarding floating-point inaccuracies.
			let epsilon = f64::EPSILON.sqrt();
			// Ensures all points are enclosed by 4-ball.
			let all_enclosed = points
				.iter()
				.all(|point| distance(point, &center) <= radius + epsilon);
			assert!(all_enclosed);
			// Ensures at least 2 points are on surface of 4-ball, mandatory to be minimum.
			let bounds_count = points
				.iter()
				.map(|point| distance(point, &center))
				.map(|distance| distance - radius)
				.map(f64::abs)
				.filter(|&deviation| deviation <= epsilon)
				.count();
			assert!(bounds_count >= 2);
		}
	}
}
