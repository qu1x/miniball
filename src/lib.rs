// Copyright © 2022-2024 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Minimum enclosing ball.
//!
//!   * Finds circumscribed *n*-ball of set of bounds, see [`Enclosing::with_bounds()`].
//!   * Finds minimum *n*-ball enclosing set of points, see [`Enclosing::enclosing_points()`].
//!
//! # Roadmap
//!
//!   * Find minimum enclosing *n*-ball of *n*-balls.
//!   * Find minimum-volume enclosing *n*-ellipsoid.
//!   * Improve numerical stability and performance.
//!
//! # Features
//!
//!   * `std` for spilling recursion stack over to the heap if necessary. Enabled by `default`.

#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![forbid(rustdoc::broken_intra_doc_links)]
#![allow(clippy::tabs_in_doc_comments)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod ball;
mod deque;
mod enclosing;
mod ovec;

pub use ball::Ball;
pub use deque::Deque;
pub use enclosing::Enclosing;
pub use nalgebra;
use ovec::OVec;
