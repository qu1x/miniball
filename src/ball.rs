// Copyright © 2022 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::Enclosing;
use nalgebra::{distance_squared, Point, RealField, SMatrix, SVector};

/// Ball over real field `R` of dimension `D` with center and radius squared.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ball<R: RealField, const D: usize> {
	/// Ball's center.
	pub center: Point<R, D>,
	/// Ball's radius squared.
	pub radius_squared: R,
}

impl<R: RealField, const D: usize> Enclosing<R, D> for Ball<R, D> {
	#[inline]
	fn contains(&self, point: &Point<R, D>) -> bool {
		distance_squared(&self.center, point) <= self.radius_squared
	}
	fn with_bounds(bounds: &[Point<R, D>]) -> Option<Self> {
		let length = (1..=D + 1)
			.contains(&bounds.len())
			.then(|| bounds.len() - 1)?;
		let points = SMatrix::<R, D, D>::from_fn(|row, column| {
			if column < length {
				bounds[column + 1].coords[row].clone() - bounds[0].coords[row].clone()
			} else {
				R::zero()
			}
		});
		let points = points.slice((0, 0), (D, length));
		let matrix = SMatrix::<R, D, D>::from_fn(|row, column| {
			if row < length && column < length {
				points.column(row).dot(&points.column(column)) * (R::one() + R::one())
			} else {
				R::zero()
			}
		});
		let matrix = matrix.slice((0, 0), (length, length));
		let vector = SVector::<R, D>::from_fn(|row, _column| {
			if row < length {
				points.column(row).norm_squared()
			} else {
				R::zero()
			}
		});
		let vector = vector.slice((0, 0), (length, 1));
		matrix.try_inverse().map(|matrix| {
			let vector = matrix * vector;
			let mut center = SVector::<R, D>::zeros();
			for point in 0..length {
				center += points.column(point) * vector[point].clone();
			}
			Ball {
				center: &bounds[0] + &center,
				radius_squared: center.norm_squared(),
			}
		})
	}
}
