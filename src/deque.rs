// Copyright © 2022-2024 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[cfg(feature = "std")]
use std::collections::{LinkedList, VecDeque};

/// Minimum double-ended queue interface.
pub trait Deque<T> {
	/// Returns the number of elements in the deque.
	#[must_use]
	fn len(&self) -> usize;

	/// Removes the first element and returns it, or `None` if the deque is empty.
	fn pop_front(&mut self) -> Option<T>;
	/// Removes the last element from the deque and returns it, or `None` if it is empty.
	fn pop_back(&mut self) -> Option<T>;

	/// Prepends an element to the deque.
	fn push_front(&mut self, value: T);
	/// Appends an element to the back of the deque.
	fn push_back(&mut self, value: T);

	/// Returns `true` if the deque is empty.
	#[must_use]
	fn is_empty(&self) -> bool {
		self.len() == 0
	}
}

#[cfg(feature = "std")]
impl<T> Deque<T> for VecDeque<T> {
	#[inline]
	fn len(&self) -> usize {
		Self::len(self)
	}

	#[inline]
	fn pop_front(&mut self) -> Option<T> {
		Self::pop_front(self)
	}
	#[inline]
	fn pop_back(&mut self) -> Option<T> {
		Self::pop_back(self)
	}

	#[inline]
	fn push_front(&mut self, value: T) {
		Self::push_front(self, value);
	}
	#[inline]
	fn push_back(&mut self, value: T) {
		Self::push_back(self, value);
	}
}

#[cfg(feature = "std")]
impl<T> Deque<T> for LinkedList<T> {
	#[inline]
	fn len(&self) -> usize {
		Self::len(self)
	}

	#[inline]
	fn pop_front(&mut self) -> Option<T> {
		Self::pop_front(self)
	}
	#[inline]
	fn pop_back(&mut self) -> Option<T> {
		Self::pop_back(self)
	}

	#[inline]
	fn push_front(&mut self, value: T) {
		Self::push_front(self, value);
	}
	#[inline]
	fn push_back(&mut self, value: T) {
		Self::push_back(self, value);
	}
}
