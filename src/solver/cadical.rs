#![deprecated(since = "0.1.6", note = "rssat is deprecated. Please use the `satgalaxy` crate instead: https://crates.io/crates/satgalaxy")]

//! The `cadical` module provides access to the `CaDiCaLSolver`.
//!
//! This module is enabled when the `minisat` feature is activated.
//!
//! # Overview
//! The `CaDiCaLSolver` struct acts as a wrapper for the [CaDiCaL](https://github.com/arminbiere/cadical) Solver, allowing users to
//! leverage its functionality for solving SAT problems.
//!
//! # Usage
//! To use the `cadical` module, ensure the `cadical` feature is enabled in your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! rssat = { version = "x.y.z", features = ["cadical"] }
//! ```
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]



use super::{RawStatus, SatSolver, Status};

/// `CaDiCaLSolver` is a wrapper for the [CaDiCaL](https://github.com/arminbiere/cadical) Solver .
/// It also allows creating a `CaDiCaL_Solver` instance for more low-level operations.
/// This struct is only available when the `cadical` feature is enabled.
/// # Example
/// ```rust
/// use rssat::solver::{CaDiCaLSolver, Status,Solver};
/// let mut solver = CaDiCaLSolver::new();
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
///  To use the `CaDiCaLSolver`, ensure the `cadical` feature is enabled in your `Cargo.toml`:
///  ```toml
///  [dependencies]
///  rssat = { version = "x.y.z", features = ["cadical"] }
pub struct CaDiCaLSolver;

impl CaDiCaLSolver {
       pub fn new() -> Self {
        println!("rssat is deprecated. Please use the `satgalaxy` crate instead: https://crates.io/crates/satgalaxy");
        Self {}
    }
    pub fn model(&mut self) -> Vec<i32> {
        println!("rssat is deprecated. Please use the `satgalaxy` crate instead: https://crates.io/crates/satgalaxy");
        vec![]
    }
}

impl SatSolver for CaDiCaLSolver {
    fn solve_model(&mut self) -> Status {
        println!("rssat is deprecated. Please use the `satgalaxy` crate instead: https://crates.io/crates/satgalaxy");
        Status::Unknown
    }

    fn add_clause(&mut self, clause: &Vec<i32>) {
        println!("rssat is deprecated. Please use the `satgalaxy` crate instead: https://crates.io/crates/satgalaxy");

    }
}
