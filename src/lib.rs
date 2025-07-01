#![doc = include_str!("../README.md")]
#![deprecated(since = "0.1.6", note = "rssat is deprecated. Please use the `satgalaxy` crate instead: https://crates.io/crates/satgalaxy")]
pub mod errors;

#[cfg(feature = "parser")]
pub mod parser;
pub mod solver;
#[cfg(test)]
mod tests {
    use solver::{SatSolver, Status};

    use super::*;

    #[test]
    #[cfg(feature = "cadical")]
    fn cadical() {
        let mut solver = solver::cadical::CaDiCaLSolver::new();
        solver.add_clause(&vec![1]);
        solver.add_clause(&vec![-1]);
        match solver.solve() {
            Status::SATISFIABLE(_vec) => {
                assert_eq!(1, 0);
            }
            Status::UNSATISFIABLE => {
                assert_eq!(true, true)
            }
            Status::UNKNOWN => {
                assert_eq!(10, 0);
            }
        }
    }
    #[test]
    #[cfg(feature = "minisat")]
    fn minisat() {
        let mut solver = solver::MinisatSolver::new();
        solver.add_clause(&vec![1]);
        solver.add_clause(&vec![-1]);
        match solver.solve_model() {
            Status::Satisfiable(vec) => {
                assert_eq!(1, 0);
            }
            Status::Unsatisfiable => {
                assert_eq!(true, true)
            }
            Status::Unknown => {
                assert_eq!(10, 0);
            }
        }
    }
    #[test]
    #[cfg(feature = "glucose")]
    fn glucose() {
        let mut solver = solver::glucose::GlucoseSolver::new();
        solver.add_clause(&vec![1]);
        solver.add_clause(&vec![-1]);
        match solver.solve() {
            Status::SATISFIABLE(vec) => {
                assert_eq!(1, 0);
            }
            Status::UNSATISFIABLE => {
                assert_eq!(true, true)
            }
            Status::UNKNOWN => {
                assert_eq!(10, 0);
            }
        }
    }
    #[test]
    #[cfg(feature = "dimacs")]
    fn dimacs() {
        use parser::parse_dimacs_cnf;

        let dimacs_content = "c This is a comment
        p cnf 3 2
        1 -3 0
        ";
        match parse_dimacs_cnf(dimacs_content, false) {
            Ok(cnf) => {
                assert_eq!(cnf.num_vars, 3);
                assert_eq!(cnf.num_clauses, 1);
            }
            Err(e) => assert_eq!("result", "should be ok"),
        }
    }
    #[test]
    #[cfg(feature = "dimacs")]
    fn dimacs_strict() {
        use parser::parse_dimacs_cnf;

        let dimacs_content = "c This is a comment
        p cnf 2 2
        1 -3 0
        ";
        match parse_dimacs_cnf(dimacs_content, true) {
            Ok(cnf) => {
                assert_eq!("result", "should be error")
            }
            Err(e) => assert!(true),
        }
    }
}
