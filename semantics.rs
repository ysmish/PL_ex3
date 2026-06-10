use std::collections::HashMap;
use crate::ast::*;

// The 'state' is a map from variable name (String) to value (i32).
pub type State = HashMap<VarName, i32>;

// solve_a: AExp -> State -> i32
pub fn solve_a(e: &AExp, s: &State) -> i32 {
    match e {
        AExp::Num(m) => *m,
        // Default to 0 if the variable is not found
        AExp::Var(x) => *s.get(x).unwrap_or(&0), 
        AExp::Add(e1, e2) => solve_a(e1, s) + solve_a(e2, s), 
        AExp::Mult(e1, e2) => /* please insert your implementation here */,
        AExp::Sub(e1, e2) => /* please insert your implementation here */,
    }
}

// BVal is simply the Rust 'bool' type, mapping to OCaml's "tt" and "ff"
pub type BVal = bool;

// solve_b: BExp -> State -> BVal (bool)
pub fn solve_b(e: &BExp, s: &State) -> BVal {
    match e {
        BExp::True => true, 
        BExp::False => false, 
        BExp::Neg(e1) => !solve_b(e1, s), 
        BExp::Beq(e1, e2) => solve_b(e1, s) == solve_b(e2, s), 
        BExp::Aeq(e1, e2) => solve_a(e1, s) == solve_a(e2, s), 
        BExp::Gte(e1, e2) => solve_a(e1, s) >= solve_a(e2, s), 
        BExp::And(e1, e2) => /* please insert your implementation here */,
    }
}

// state update: to get a new state
pub fn update(x: &VarName, e: &AExp, s: &State) -> State {
    let mut new_state = s.clone();
    let value = solve_a(e, s);
    new_state.insert(x.clone(), value);
    new_state
}



// ----------- Test Cases States  --------
// Initial state s0 (x = 1)
pub fn s0() -> State {
    let mut s = HashMap::new();
    s.insert("x".to_string(), 1); 
    s
}

// Initial state s1 (x = 5)
pub fn s1() -> State {
    let mut s = HashMap::new();
    s.insert("x".to_string(), 5);
    s
}

// Initial state s2 (x = 10, y = 5)
pub fn s2() -> State {
    let mut s = HashMap::new();
    s.insert("x".to_string(), 10);
    s.insert("y".to_string(), 5);
    s
}