// Copyright © 2022-2024 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use core::mem::take;
use nalgebra::{base::allocator::Allocator, DefaultAllocator, DimName, OVector};

/// Owned vector of item `T` and capacity `D`.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OVec<T: Default, D: DimName>
where
	OVector<T, D>: Default,
	DefaultAllocator: Allocator<T, D>,
{
	size: usize,
	data: OVector<T, D>,
}

impl<T: Default, D: DimName> OVec<T, D>
where
	OVector<T, D>: Default,
	DefaultAllocator: Allocator<T, D>,
{
	/// New empty vector.
	#[must_use]
	#[inline]
	pub fn new() -> Self {
		Self::default()
	}
	/// Maximum number of items.
	#[must_use]
	#[inline]
	pub fn capacity(&self) -> usize {
		self.data.len()
	}
	/// Number of items.
	#[must_use]
	#[inline]
	pub const fn len(&self) -> usize {
		self.size
	}
	/// Whether vector is empty.
	#[must_use]
	#[inline]
	pub const fn is_empty(&self) -> bool {
		self.len() == 0
	}
	/// Whether vector is full.
	#[must_use]
	#[inline]
	pub fn is_full(&self) -> bool {
		self.len() == self.data.len()
	}
	/// Immutable slice of items.
	#[must_use]
	#[inline]
	pub fn as_slice(&self) -> &[T] {
		&self.data.as_slice()[..self.len()]
	}
	/// Adds `item`.
	///
	/// # Panics
	///
	/// Panics if [`Self::is_full()`].
	#[inline]
	pub fn push(&mut self, item: T) {
		self.data[self.size] = item;
		self.size += 1;
	}
	/// Removes last item.
	///
	/// Returns `Some(T)` or `None` if [`Self::is_empty()`].
	#[inline]
	pub fn pop(&mut self) -> Option<T> {
		if self.is_empty() {
			None
		} else {
			self.size -= 1;
			Some(take(&mut self.data[self.size]))
		}
	}
}

impl<T: Default, D: DimName> Default for OVec<T, D>
where
	OVector<T, D>: Default,
	DefaultAllocator: Allocator<T, D>,
{
	fn default() -> Self {
		Self {
			size: 0,
			data: OVector::default(),
		}
	}
}
