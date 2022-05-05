// Copyright © 2022 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Minimum enclosing ball.
//!
//! **NOTE**: This crate requires nightly Rust.
//!
//!   * Finds circumscribed *n*-ball of set of bounds, see [`Enclosing::with_bounds()`].
//!   * Finds minimum *n*-ball enclosing set of points, see [`Enclosing::enclosing_points()`].
//!
//! # Roadmap
//!
//!   * Implement [`Enclosing`] for `Ellipsoid` structure.
//!   * Implement approximation algorithm as part of `ApproxEnclosing` trait.
//!   * Implement finding minimum enclosing ball of balls.

#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![allow(clippy::tabs_in_doc_comments)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(let_chains)]

mod ball;
mod deque;
mod enclosing;

pub use ball::Ball;
pub use deque::Deque;
pub use enclosing::Enclosing;
pub use nalgebra;
