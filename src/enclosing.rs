// Copyright © 2022-2024 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::{Deque, OVec};
use core::mem::size_of;
use nalgebra::{
	base::allocator::Allocator, DefaultAllocator, DimName, DimNameAdd, DimNameSum, OPoint,
	RealField, U1,
};
#[cfg(feature = "std")]
use stacker::maybe_grow;

#[cfg(not(feature = "std"))]
#[inline]
fn maybe_grow<R, F: FnOnce() -> R>(_red_zone: usize, _stack_size: usize, callback: F) -> R {
	callback()
}

/// Minimum enclosing ball.
pub trait Enclosing<T: RealField, D: DimName>
where
	Self: Clone,
	DefaultAllocator: Allocator<T, D>,
{
	#[doc(hidden)]
	/// Guaranteed stack size per recursion step.
	const RED_ZONE: usize =
		32 * 1_024 + (8 * D::USIZE + 2 * D::USIZE.pow(2)) * size_of::<OPoint<T, D>>();
	#[doc(hidden)]
	/// New stack space to allocate if within [`Self::RED_ZONE`].
	const STACK_SIZE: usize = Self::RED_ZONE * 1_024;

	/// Whether ball contains `point`.
	#[must_use]
	fn contains(&self, point: &OPoint<T, D>) -> bool;
	/// Returns circumscribed ball with all `bounds` on surface or `None` if it does not exist.
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
	fn with_bounds(bounds: &[OPoint<T, D>]) -> Option<Self>
	where
		DefaultAllocator: Allocator<T, D, D>;

	/// Returns minimum ball enclosing `points`.
	///
	/// Points should be randomly permuted beforehand to ensure expected time complexity. Accepts
	/// mutable reference to container implementing [`Deque`] to move potential points on surface to
	/// the front. This does not converge towards a reproducible total order but significantly
	/// speeds up further invocations if the use case involves adding new points, non-enclosed ones
	/// to the front and enclosed ones to the back.
	///
	/// Implements [Welzl's recursive algorithm] with move-to-front heuristic. No allocations happen
	/// unless the real field `T` is not [`Copy`] or the stack size enters the dimension-dependant
	/// red zone in which case temporary stack space will be allocated on the heap if the `std`
	/// feature is enabled.
	///
	/// [Welzl's recursive algorithm]: https://api.semanticscholar.org/CorpusID:17569809
	///
	/// # Complexity
	///
	/// Expected time complexity is *O*((*n*+1)(*n*+1)!*m*) for *m* randomly permuted
	/// *n*-dimensional points. The complexity constant in *m* is significantly reduced by reusing
	/// permuted points of previous invocations.
	///
	/// # Stability
	///
	/// Due to floating-point inaccuracies, the returned ball might not exactly be the minimum for
	/// degenerate (e.g., co-spherical) `points`. The accuracy is depending on the shape and order
	/// of `points` with an expected worst-case factor of `T::one() ± T::default_epsilon().sqrt()`
	/// where `T::one()` is exact.
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
	/// // Epsilon of numerical stability for computing circumscribed 4-ball. This is related to
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
	fn enclosing_points(points: &mut impl Deque<OPoint<T, D>>) -> Self
	where
		D: DimNameAdd<U1>,
		DefaultAllocator: Allocator<T, D, D> + Allocator<OPoint<T, D>, DimNameSum<D, U1>>,
		<DefaultAllocator as Allocator<OPoint<T, D>, DimNameSum<D, U1>>>::Buffer: Default,
	{
		assert!(!points.is_empty(), "empty point set");
		let mut bounds = OVec::<OPoint<T, D>, DimNameSum<D, U1>>::new();
		(0..bounds.capacity())
			.find_map(|_| {
				maybe_grow(Self::RED_ZONE, Self::STACK_SIZE, || {
					Self::enclosing_points_with_bounds(points, &mut bounds)
				})
			})
			.expect("numerical instability")
	}
	/// Returns minimum ball enclosing `points` with `bounds`.
	///
	/// Recursive helper for [`Self::enclosing_points()`].
	#[doc(hidden)]
	#[must_use]
	fn enclosing_points_with_bounds(
		points: &mut impl Deque<OPoint<T, D>>,
		bounds: &mut OVec<OPoint<T, D>, DimNameSum<D, U1>>,
	) -> Option<Self>
	where
		D: DimNameAdd<U1>,
		DefaultAllocator: Allocator<T, D, D> + Allocator<OPoint<T, D>, DimNameSum<D, U1>>,
		<DefaultAllocator as Allocator<OPoint<T, D>, DimNameSum<D, U1>>>::Buffer: Default,
	{
		// Take point from back.
		if let Some(point) = points.pop_back().filter(|_| !bounds.is_full()) {
			let ball = maybe_grow(Self::RED_ZONE, Self::STACK_SIZE, || {
				// Branch with one point less.
				Self::enclosing_points_with_bounds(points, bounds)
			});
			if let Some(ball) = ball.filter(|ball| ball.contains(&point)) {
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
			Self::with_bounds(bounds.as_slice())
		}
	}
}
