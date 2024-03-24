// Copyright © 2022-2024 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::Enclosing;
use nalgebra::{
	base::allocator::Allocator, DefaultAllocator, DimName, OMatrix, OPoint, OVector, RealField,
};

/// Ball over real field `T` of dimension `D` with center and radius squared.
#[derive(Debug, Clone, PartialEq)]
pub struct Ball<T: RealField, D: DimName>
where
	DefaultAllocator: Allocator<T, D>,
{
	/// Ball's center.
	pub center: OPoint<T, D>,
	/// Ball's radius squared.
	pub radius_squared: T,
}

impl<T: RealField + Copy, D: DimName> Copy for Ball<T, D>
where
	OPoint<T, D>: Copy,
	DefaultAllocator: Allocator<T, D>,
{
}

impl<T: RealField, D: DimName> Enclosing<T, D> for Ball<T, D>
where
	DefaultAllocator: Allocator<T, D>,
{
	#[inline]
	fn contains(&self, point: &OPoint<T, D>) -> bool {
		(point - &self.center).norm_squared() <= self.radius_squared
	}
	fn with_bounds(bounds: &[OPoint<T, D>]) -> Option<Self>
	where
		DefaultAllocator: Allocator<T, D, D>,
	{
		let length = bounds.len().checked_sub(1).filter(|&len| len <= D::USIZE)?;
		let points = OMatrix::<T, D, D>::from_fn(|row, column| {
			if column < length {
				bounds[column + 1].coords[row].clone() - bounds[0].coords[row].clone()
			} else {
				T::zero()
			}
		});
		let points = points.view((0, 0), (D::USIZE, length));
		let matrix = OMatrix::<T, D, D>::from_fn(|row, column| {
			if row < length && column < length {
				points.column(row).dot(&points.column(column)) * (T::one() + T::one())
			} else {
				T::zero()
			}
		});
		let matrix = matrix.view((0, 0), (length, length));
		let vector = OVector::<T, D>::from_fn(|row, _column| {
			if row < length {
				points.column(row).norm_squared()
			} else {
				T::zero()
			}
		});
		let vector = vector.view((0, 0), (length, 1));
		matrix.try_inverse().map(|matrix| {
			let vector = matrix * vector;
			let mut center = OVector::<T, D>::zeros();
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
