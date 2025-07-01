#![deprecated(since = "0.1.6", note = "rssat is deprecated. Please use the `satgalaxy` crate instead: https://crates.io/crates/satgalaxy")]

//! The `glucose` module provides access to the `GlucoseSolver`.
//!
//! This module is enabled when the `minisat` feature is activated.
//!
//! # Overview
//! The `GlucoseSolver` struct acts as a wrapper for the [Glucose](https://github.com/audemard/glucose) Solver, allowing users to
//! leverage its functionality for solving SAT problems.
//!
//! # Usage
//! To use the `glucose` module, ensure the `glucose` feature is enabled in your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! rssat = { version = "x.y.z", features = ["glucose"] }
//! ```
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]


use super::{RawStatus, SatSolver, Status};

/// `GlucoseSolver` is a wrapper for the [Glucose](https://github.com/audemard/glucose) SimpSolver.
/// It also allows creating a `Glucose_StdSimpSolver` instance for more low-level operations.
/// This struct is only available when the `minisat` feature is enabled.
/// # Example
/// ```rust
/// use rssat::solver::{GlucoseSolver, Status,Solver};
/// let mut solver = GlucoseSolver::new();
///     solver.add_clause(&vec![1, 2]);
///     solver.add_clause(&vec![-1, -2]);
///     solver.add_clause(&vec![3]);
/// 
/// match solver.solve() {
///    Status::SATISFIABLE(vec) => {
///         println!("Satisfiable solution: {:?}", vec);
///     },
///     Status::UNSATISFIABLE => {
///         println!("Unsatisfiable");
///     },
///     Status::UNKNOWN => {
///         println!("Unknown");
///     },
/// }
/// ```
///  # Usage
///  To use the `GlucoseSolver`, ensure the `glucose` feature is enabled in your `Cargo.toml`:
///  ```toml
///  [dependencies]
///  rssat = { version = "x.y.z", features = ["glucose"] }
pub struct GlucoseSolver;

impl GlucoseSolver {
   pub fn new() -> Self {
        println!("rssat is deprecated. Please use the `satgalaxy` crate instead: https://crates.io/crates/satgalaxy");
        Self {}
    }
    pub fn model(&mut self) -> Vec<i32> {
        println!("rssat is deprecated. Please use the `satgalaxy` crate instead: https://crates.io/crates/satgalaxy");
        vec![]
    }
}

impl SatSolver for GlucoseSolver {
      fn solve_model(&mut self) -> Status {
        println!("rssat is deprecated. Please use the `satgalaxy` crate instead: https://crates.io/crates/satgalaxy");
        Status::Unknown
    }

    fn add_clause(&mut self, clause: &Vec<i32>) {
        println!("rssat is deprecated. Please use the `satgalaxy` crate instead: https://crates.io/crates/satgalaxy");
    }
}
