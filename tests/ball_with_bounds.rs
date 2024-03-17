// Copyright © 2022 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![allow(clippy::float_cmp)]

use miniball::{Ball, Enclosing};
use nalgebra::{center, Point, Point1, Point2, Point3, Vector1, Vector2, Vector3, U0, U1, U2, U3};

#[test]
fn circumscribed_0_ball_with_0_bounds() {
	let ball = Ball::<f64, U0>::with_bounds(&[]);
	assert_eq!(ball, None);
}

#[test]
fn circumscribed_0_ball_with_1_bounds() {
	let a = Point::<f64, 0>::origin();
	let Ball {
		center,
		radius_squared,
	} = Ball::with_bounds(&[a]).unwrap();
	assert_eq!(center, a);
	assert_eq!(radius_squared, 0.0);
}

#[test]
fn circumscribed_0_ball_with_2_bounds() {
	let a = Point::<f64, 0>::origin();
	let b = Point::<f64, 0>::origin();
	let ball = Ball::with_bounds(&[a, b]);
	assert_eq!(ball, None);
}

#[test]
fn circumscribed_1_ball_with_0_bounds() {
	let ball = Ball::<f64, U1>::with_bounds(&[]);
	assert_eq!(ball, None);
}

#[test]
fn circumscribed_1_ball_with_1_bounds() {
	let a = Point1::new(1.0);
	let Ball {
		center,
		radius_squared,
	} = Ball::with_bounds(&[a]).unwrap();
	assert_eq!(center, a);
	assert_eq!(radius_squared, 0.0);
}

#[test]
fn circumscribed_1_ball_with_2_bounds() {
	let offset = Vector1::new(7.0);
	let a = Point1::new(1.0);
	let b = Point1::new(-1.0);
	let Ball {
		center,
		radius_squared,
	} = Ball::with_bounds(&[a, b].map(|bound| bound * 3.0).map(|bound| bound + offset)).unwrap();
	assert_eq!(center, offset.into());
	assert_eq!(radius_squared, 9.0);
}

#[test]
fn circumscribed_2_ball_with_0_bounds() {
	let ball = Ball::<f64, U2>::with_bounds(&[]);
	assert_eq!(ball, None);
}

#[test]
fn circumscribed_2_ball_with_1_bounds() {
	let a = Point2::new(1.0, -1.0);
	let Ball {
		center,
		radius_squared,
	} = Ball::with_bounds(&[a]).unwrap();
	assert_eq!(center, a);
	assert_eq!(radius_squared, 0.0);
}

#[test]
fn circumscribed_2_ball_with_2_bounds() {
	let offset = Vector2::new(-3.0, 7.0);
	let a = Point2::new(1.0, 0.0);
	let b = Point2::new(-1.0, 0.0);
	let Ball {
		center,
		radius_squared,
	} = Ball::with_bounds(&[a, b].map(|bound| bound * 3.0).map(|bound| bound + offset)).unwrap();
	assert_eq!(center, offset.into());
	assert_eq!(radius_squared, 9.0);
}

#[test]
fn circumscribed_2_ball_with_3_bounds() {
	let offset = Vector2::new(-3.0, 7.0);
	let a = Point2::new(1.0, 0.0);
	let b = Point2::new(0.0, 1.0);
	let c = Point2::new(-1.0, 0.0);
	let Ball {
		center,
		radius_squared,
	} = Ball::with_bounds(
		&[a, b, c]
			.map(|bound| bound * 3.0)
			.map(|bound| bound + offset),
	)
	.unwrap();
	assert_eq!(center, offset.into());
	assert_eq!(radius_squared, 9.0);
}

#[test]
fn circumscribed_2_ball_with_3_points() {
	let offset = Vector2::new(-3.0, 7.0);
	let a = Point2::new(1.0, 0.0);
	let b = Point2::new(0.0, 1.0);
	let c = center(&a, &b);
	let ball = Ball::with_bounds(
		&[a, b, c]
			.map(|bound| bound * 3.0)
			.map(|bound| bound + offset),
	);
	assert_eq!(ball, None);
}

#[test]
fn circumscribed_3_ball_with_0_bounds() {
	let ball = Ball::<f64, U3>::with_bounds(&[]);
	assert_eq!(ball, None);
}

#[test]
fn circumscribed_3_ball_with_1_bounds() {
	let a = Point3::new(1.0, -1.0, 0.0);
	let Ball {
		center,
		radius_squared,
	} = Ball::with_bounds(&[a]).unwrap();
	assert_eq!(center, a);
	assert_eq!(radius_squared, 0.0);
}

#[test]
fn circumscribed_3_ball_with_2_bounds() {
	let offset = Vector3::new(-3.0, 7.0, 4.8);
	let a = Point3::new(1.0, 0.0, 0.0);
	let b = Point3::new(-1.0, 0.0, 0.0);
	let Ball {
		center,
		radius_squared,
	} = Ball::with_bounds(&[a, b].map(|bound| bound * 3.0).map(|bound| bound + offset)).unwrap();
	assert_eq!(center, offset.into());
	assert_eq!(radius_squared, 9.0);
}

#[test]
fn circumscribed_3_ball_with_3_bounds() {
	let offset = Vector3::new(-3.0, 7.0, 4.8);
	let a = Point3::new(1.0, 0.0, 0.0);
	let b = Point3::new(0.0, 1.0, 0.0);
	let c = Point3::new(-1.0, 0.0, 0.0);
	let Ball {
		center,
		radius_squared,
	} = Ball::with_bounds(
		&[a, b, c]
			.map(|bound| bound * 3.0)
			.map(|bound| bound + offset),
	)
	.unwrap();
	assert_eq!(center, offset.into());
	assert_eq!(radius_squared, 9.0);
}

#[test]
fn circumscribed_3_ball_with_3_points() {
	let offset = Vector3::new(-3.0, 7.0, 4.8);
	let a = Point3::new(1.0, 0.0, 0.0);
	let b = Point3::new(0.0, 1.0, 0.0);
	let c = center(&a, &b);
	let ball = Ball::with_bounds(
		&[a, b, c]
			.map(|bound| bound * 3.0)
			.map(|bound| bound + offset),
	);
	assert_eq!(ball, None);
}

#[test]
fn circumscribed_3_ball_with_4_bounds() {
	let offset = Vector3::new(-3.0, 7.0, 4.8);
	let a = Point3::new(1.0, 1.0, 1.0);
	let b = Point3::new(1.0, -1.0, -1.0);
	let c = Point3::new(-1.0, 1.0, -1.0);
	let d = Point3::new(-1.0, -1.0, 1.0);
	let Ball {
		center,
		radius_squared,
	} = Ball::with_bounds(&[a, b, c, d].map(|bound| bound + offset)).unwrap();
	assert_eq!(center, offset.into());
	assert_eq!(radius_squared, 3.0);
}

#[test]
fn circumscribed_3_ball_with_4_points() {
	let offset = Vector3::new(-3.0, 7.0, 4.8);
	let a = Point3::new(1.0, 1.0, 1.0);
	let b = Point3::new(1.0, -1.0, -1.0);
	let c = Point3::new(-1.0, 1.0, -1.0);
	let d = (a + b.coords + c.coords) / 3.0;
	let ball = Ball::with_bounds(&[a, b, c, d].map(|bound| bound + offset));
	assert_eq!(ball, None);
}
