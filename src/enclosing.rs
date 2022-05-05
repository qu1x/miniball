// Copyright © 2022 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::Deque;
use arrayvec::ArrayVec;
use nalgebra::{Point, RealField};
use stacker::maybe_grow;
use std::mem::size_of;

/// Minimum enclosing ball.
pub trait Enclosing<R: RealField, const D: usize>: Clone {
	#[doc(hidden)]
	/// Guaranteed stack size per recursion step.
	const RED_ZONE: usize = 32 * 1_024 + (8 * D + 2 * D.pow(2)) * size_of::<Point<R, D>>();
	#[doc(hidden)]
	/// New stack space to allocate if within [`Self::RED_ZONE`].
	const STACK_SIZE: usize = Self::RED_ZONE * 1_024;

	/// Whether ball contains `point`.
	#[must_use]
	fn contains(&self, point: &Point<R, D>) -> bool;
	/// Returns circumscribed ball with `bounds` if it exists.
	///
	/// # Example
	///
	/// Finds circumscribed 3-ball of 3-simplex (tetrahedron):
	///
	/// ```
	/// use miniball::{
	/// 	nalgebra::{Point3, Vector3},
	/// 	{Ball, Enclosing},
	/// };
	///
	/// // 3-simplex.
	/// let a = Point3::new(1.0, 1.0, 1.0);
	/// let b = Point3::new(1.0, -1.0, -1.0);
	/// let c = Point3::new(-1.0, 1.0, -1.0);
	/// let d = Point3::new(-1.0, -1.0, 1.0);
	/// // Center of 3-simplex.
	/// let offset = Vector3::new(-3.0, 7.0, 4.8);
	/// // Computes circumscribed 3-ball of 3-simplex.
	/// let Ball {
	/// 	center,
	/// 	radius_squared,
	/// } = Ball::with_bounds(&[a, b, c, d].map(|bound| bound + offset)).unwrap();
	/// // Ensures enclosing 3-ball is centered around 3-simplex.
	/// assert_eq!(center, offset.into());
	/// // Ensures enclosing 3-ball's radius matches center-to-point distances of 3-simplex.
	/// assert_eq!(radius_squared, 3.0);
	/// ```
	#[must_use]
	fn with_bounds(bounds: &[Point<R, D>]) -> Option<Self>;

	/// Returns minimum ball enclosing `points`.
	///
	/// Points should be randomly permuted beforehand to ensure expected time complexity. It accepts
	/// a mutable deque to permute the points from inside to outside. This does not converge into a
	/// final reproducible order but reusing points by adding new enclosed points to the front and
	/// new points on the outside to the back will significantly speed up further invocations.
	///
	/// Implements [Welzl's recursive algorithm] with move-to-front heuristic. It is allocation-free
	/// until stack size enters dimension-dependant red zone in which case temporary stack space
	/// will be allocated. Allocations will also happen if real field `R` is not [`Copy`].
	///
	/// [Welzl's recursive algorithm]: https://api.semanticscholar.org/CorpusID:17569809
	///
	/// # Complexity
	///
	/// Expected time complexity is *O(cn)* for *n* randomly permuted points. Complexity constant
	/// *c* is significantly reduced by reusing permuted points of previous invocations.
	///
	/// # Example
	///
	/// Finds minimum 4-ball enclosing 4-cube (tesseract):
	///
	/// ```
	/// use miniball::{
	/// 	nalgebra::{distance, Point4, Vector4},
	/// 	{Ball, Enclosing},
	/// };
	/// use std::collections::VecDeque;
	///
	/// // Uniform distribution in 4-cube centered around `offset` with room `diagonal_halved`.
	/// let offset = Vector4::new(-3.0, 7.0, 4.8, 1.2);
	/// let diagonal_halved = 3.0;
	/// let mut points = (0..60_000)
	/// 	.map(|_point| Point4::<f64>::from(Vector4::new_random() - Vector4::from_element(0.5)))
	/// 	.map(|point| point * diagonal_halved)
	/// 	.map(|point| point + offset)
	/// 	.collect::<VecDeque<_>>();
	/// // Computes 4-ball enclosing 4-cube.
	/// let Ball {
	/// 	center,
	/// 	radius_squared,
	/// } = Ball::enclosing_points(&mut points);
	/// let radius = radius_squared.sqrt();
	/// // Ensures enclosing 4-ball is roughly centered around uniform distribution in 4-cube and
	/// // radius roughly matches room diagonal halved, guaranteeing certain uniformity of randomly
	/// // distributed points.
	/// assert!((center - offset).map(f64::abs) <= Vector4::from_element(1.0).into());
	/// assert!((radius - diagonal_halved).abs() <= 1.0);
	/// // Epsilon of numeric stability for computing circumscribed 4-ball. This is related to
	/// // robustness of `Enclosing::with_bounds()` regarding floating-point inaccuracies.
	/// let epsilon = f64::EPSILON.sqrt();
	/// // Ensures all points are enclosed by 4-ball.
	/// let all_enclosed = points
	/// 	.iter()
	/// 	.all(|point| distance(point, &center) <= radius + epsilon);
	/// assert!(all_enclosed);
	/// // Ensures at least 2 points are on surface of 4-ball, mandatory to be minimum.
	/// let bounds_count = points
	/// 	.iter()
	/// 	.map(|point| distance(point, &center))
	/// 	.map(|distance| distance - radius)
	/// 	.map(f64::abs)
	/// 	.filter(|&deviation| deviation <= epsilon)
	/// 	.count();
	/// assert!(bounds_count >= 2);
	/// ```
	#[must_use]
	#[inline]
	fn enclosing_points(points: &mut impl Deque<Point<R, D>>) -> Self
	where
		ArrayVec<Point<R, D>, { D + 1 }>:,
	{
		maybe_grow(Self::RED_ZONE, Self::STACK_SIZE, || {
			Self::enclosing_points_with_bounds(points, &mut ArrayVec::new())
				.expect("Empty point set")
		})
	}
	/// Returns minimum ball enclosing `points` with `bounds`.
	///
	/// Recursive helper for [`Self::enclosing_points()`].
	#[doc(hidden)]
	#[must_use]
	fn enclosing_points_with_bounds(
		points: &mut impl Deque<Point<R, D>>,
		bounds: &mut ArrayVec<Point<R, D>, { D + 1 }>,
	) -> Option<Self> {
		// Take point from back.
		if bounds.len() < bounds.capacity() && let Some(point) = points.pop_back() {
			let ball = maybe_grow(Self::RED_ZONE, Self::STACK_SIZE, || {
				// Branch with one point less.
				Self::enclosing_points_with_bounds(points, bounds)
			});
			if let Some(ball) = ball && ball.contains(&point) {
				// Move point to back.
				points.push_back(point);
				Some(ball)
			} else {
				// Move point to bounds.
				bounds.push(point);
				let ball = maybe_grow(Self::RED_ZONE, Self::STACK_SIZE, || {
					// Branch with one point less and one bound more.
					Self::enclosing_points_with_bounds(points, bounds)
				});
				// Move point to front.
				points.push_front(bounds.pop().unwrap());
				ball
			}
		} else {
			// Circumscribed ball with bounds.
			Self::with_bounds(bounds)
		}
	}
}
