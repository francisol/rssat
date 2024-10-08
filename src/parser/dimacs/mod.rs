use std::{cmp::max, fs, io::{self, Read}};

use crate::errors::ParserError;

use super::CnfFormula;
use  pest::Parser;
#[derive(pest_derive::Parser)]
#[grammar = "../pest/dimacs.pest"]
struct DIMACSParser;


/// Parses a DIMACS CNF format string into a `CnfFormula` struct.
/// # Example
/// ```rust
///  use rssat::parser::parse_dimacs_cnf;
/// 
/// let dimacs_content = "c This is a comment
/// p cnf 3 2
/// 1 -3 0
/// ";
///     match parse_dimacs_cnf(dimacs_content,false) {
///         Ok(cnf) => {
///             assert_eq!(cnf.num_vars,3);
///             assert_eq!(cnf.num_clauses,2);
///         },
///         Err(e) => println!("Error: {:?}", e),
 ///    }
/// ```
/// 
/// # Usage
///  To use the `parse_dimacs_cnf`, ensure the `dimacs_` feature is enabled in your `Cargo.toml`:
///  ```toml
///  [dependencies]
///  rssat = { version = "x.y.z", features = ["dimacs_"] }
/// ```
/// # Arguments
///
/// * `input` - A string slice that holds the content of the DIMACS CNF file.
/// * `strict` - A boolean flag that determines whether to enforce strict parsing rules.
///
/// # Returns
///
/// * `Ok(CnfFormula)` - If parsing is successful, returns a `CnfFormula` struct.
/// * `Err(ParserError)` - If parsing fails, returns a `ParserError`.
///
/// # Errors
///
/// This function will return an error if:
/// * The input does not conform to the DIMACS CNF format.
/// * The number of variables exceeds the declared number when in strict mode.
/// * Any integer parsing fails.
///
/// # Behavior
///
/// * Parses the input string according to DIMACS CNF format rules.
/// * In strict mode, it enforces the declared number of variables and clauses.
/// * Constructs a `CnfFormula` with parsed clauses and variable information.
pub fn parse_dimacs_cnf(input: &str,strict:bool) -> Result<CnfFormula,ParserError> {
    let mut cnf= CnfFormula::new();
    let mut num_vars=0;
    let mut variables=-1;
    let mut clauses=0;
    let pairs= DIMACSParser::parse(Rule::file,input)?;
    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::cluase =>{
                    if strict &&  clauses > 0&& cnf.clauses.len()>=clauses {
                        break;
                    }

                    let mut clause= Vec::<i32>::new();
                    for lit_pair in inner_pair.into_inner(){
                        let lit = lit_pair.as_str().parse::<i32>()?;
                        let abs=lit.abs();
                        num_vars =max(abs, num_vars);
                        clause.push(lit);
                    }
                    cnf.clauses.push(clause);
                },
                Rule::def =>{
                    for def_rule in inner_pair.into_inner() {
                        match def_rule.as_rule() {
                            Rule::variables =>{
                                variables=   def_rule.as_str().parse::<i32>()?;
                            },
                            Rule::clauses=>{
                                clauses=   def_rule.as_str().parse::<i32>().map(|o| {o.try_into().unwrap()})?;
                            
                            }
                            _ =>{}
                        }
                    }
                }
                _ => {}
            };
        }
    }
    if strict {
        if num_vars> variables {
            return Err(ParserError::TooManyVariables(num_vars,variables));
        }
        num_vars=variables;
    }
    cnf.num_vars=num_vars.try_into().unwrap();
    cnf.num_clauses=cnf.clauses.len();
    Ok(cnf)
}



/// Reads a DIMACS CNF file from a given path or standard input and parses it into a `CnfFormula`.
pub fn read_dimacs_from_file(path: &str,strict:bool) -> Result<CnfFormula,ParserError> {
    let data= if path=="-" {
        let mut buf = String::new();
        let _=io::stdin().read_to_string(&mut buf);
        buf
    }else{
        fs::read_to_string(path)?
    };
   parse_dimacs_cnf(&data,strict)

}