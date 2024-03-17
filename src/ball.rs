// Copyright © 2022-2024 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::Enclosing;
use nalgebra::{
	base::allocator::Allocator, DefaultAllocator, DimNameAdd, DimNameSum, OMatrix, OPoint, OVector,
	RealField, U1,
};

/// Ball over real field `R` of dimension `D` with center and radius squared.
#[derive(Debug, Clone, PartialEq)]
pub struct Ball<R: RealField, D: DimNameAdd<U1>>
where
	DefaultAllocator: Allocator<R, D>,
{
	/// Ball's center.
	pub center: OPoint<R, D>,
	/// Ball's radius squared.
	pub radius_squared: R,
}

impl<R: RealField + Copy, D: DimNameAdd<U1>> Copy for Ball<R, D>
where
	OPoint<R, D>: Copy,
	DefaultAllocator: Allocator<R, D>,
{
}

impl<R: RealField, D: DimNameAdd<U1>> Enclosing<R, D> for Ball<R, D>
where
	DefaultAllocator:
		Allocator<R, D> + Allocator<R, D, D> + Allocator<OPoint<R, D>, DimNameSum<D, U1>>,
	<DefaultAllocator as Allocator<OPoint<R, D>, DimNameSum<D, U1>>>::Buffer: Default,
{
	#[inline]
	fn contains(&self, point: &OPoint<R, D>) -> bool {
		(point - &self.center).norm_squared() <= self.radius_squared
	}
	fn with_bounds(bounds: &[OPoint<R, D>]) -> Option<Self> {
		let length = bounds.len().checked_sub(1).filter(|&len| len <= D::USIZE)?;
		let points = OMatrix::<R, D, D>::from_fn(|row, column| {
			if column < length {
				bounds[column + 1].coords[row].clone() - bounds[0].coords[row].clone()
			} else {
				R::zero()
			}
		});
		let points = points.view((0, 0), (D::USIZE, length));
		let matrix = OMatrix::<R, D, D>::from_fn(|row, column| {
			if row < length && column < length {
				points.column(row).dot(&points.column(column)) * (R::one() + R::one())
			} else {
				R::zero()
			}
		});
		let matrix = matrix.view((0, 0), (length, length));
		let vector = OVector::<R, D>::from_fn(|row, _column| {
			if row < length {
				points.column(row).norm_squared()
			} else {
				R::zero()
			}
		});
		let vector = vector.view((0, 0), (length, 1));
		matrix.try_inverse().map(|matrix| {
			let vector = matrix * vector;
			let mut center = OVector::<R, D>::zeros();
			for point in 0..length {
				center += points.column(point) * vector[point].clone();
			}
			Self {
				center: &bounds[0] + &center,
				radius_squared: center.norm_squared(),
			}
		})
	}
}
