#![deprecated(
    since = "0.1.6",
    note = "rssat is deprecated. Please use the `satgalaxy` crate instead: https://crates.io/crates/satgalaxy"
)]

//! The `minisat` module provides access to the `MinisatSolver`.
//!
//! This module is enabled when the `minisat` feature is activated.
//!
//! # Overview
//! The `MinisatSolver` struct acts as a wrapper for the [MiniSat](https://github.com/niklasso/minisat) SimpSolver, allowing users to
//! leverage its functionality for solving SAT problems.
//!
//! # Usage
//! To use the `minisat` module, ensure the `minisat` feature is enabled in your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! rssat = { version = "x.y.z", features = ["minisat"] }
//! ```
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// mod bindings {
//     include!(concat!(env!("OUT_DIR"), "/minisat_bindings.rs"));
// }
use super::{RawStatus, SatSolver, Status};
use std::ffi::{c_int, c_void};

/// `MinisatSolver` is a wrapper for the [MiniSat](https://github.com/niklasso/minisat) SimpSolver.
/// It also allows creating a `Minisat_StdSimpSolver` instance for more low-level operations.
/// This struct is only available when the `minisat` feature is enabled.
/// # Example
/// ```rust
/// use rssat::solver::{MinisatSolver, Status,Solver};
/// let mut solver = MinisatSolver::new();
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
///  To use the `MinisatSolver`, ensure the `minisat` feature is enabled in your `Cargo.toml`:
///  ```toml
///  [dependencies]
///  rssat = { version = "x.y.z", features = ["minisat"] }
pub struct MinisatSolver;

impl MinisatSolver {
    pub fn new() -> Self {
        println!("rssat is deprecated. Please use the `satgalaxy` crate instead: https://crates.io/crates/satgalaxy");
        Self {}
    }
    pub fn model(&mut self) -> Vec<i32> {
        println!("rssat is deprecated. Please use the `satgalaxy` crate instead: https://crates.io/crates/satgalaxy");
        vec![]
    }
}

impl SatSolver for MinisatSolver {
    fn solve_model(&mut self) -> Status {
        println!("rssat is deprecated. Please use the `satgalaxy` crate instead: https://crates.io/crates/satgalaxy");
        Status::Unknown
    }

    fn add_clause(&mut self, clause: &Vec<i32>) {
        println!("rssat is deprecated. Please use the `satgalaxy` crate instead: https://crates.io/crates/satgalaxy");

    }
}
impl Drop for MinisatSolver {
    fn drop(&mut self) {
        
    }
}
