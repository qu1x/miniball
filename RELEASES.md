# Version 0.4.0 (2024-03-24)

  * Lower trait bounds on `Ball` and `Enclosing`.
  * Use `T` in favor of `R` as does `nalgebra`.
  * Allow `no_std` by gating `stacker` dependency and `Deque` implementations behind `std` feature.

# Version 0.3.0 (2024-03-17)

  * Replace `Point` with `OPoint` supporting arithmetic at compile-time on stable Rust.
  * Replace `ArrayVec<T, { D + 1 }>` with `OVec<T, DimNameSum<D, U1>>` supporting stabe Rust.

# Version 0.2.0 (2023-04-01)

  * Update dependencies.

# Version 0.1.1 (2022-05-06)

  * Add more tests.
  * Update documentation.

# Version 0.1.0 (2022-05-05)

  * Implement minimum enclosing ball.
