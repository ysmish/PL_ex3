use std::collections::HashMap;
use crate::ast::*;

// The 'state' is a map from variable name (String) to value (i32).
pub type State = HashMap<VarName, i32>;

// solve_a: AExp -> State -> i32   (this is A[[e]]s from the course)
pub fn solve_a(e: &AExp, s: &State) -> i32 {
    match e {
        AExp::Num(m) => *m,
        // Default to 0 if the variable is not found
        AExp::Var(x) => *s.get(x).unwrap_or(&0),
        AExp::Add(e1, e2) => solve_a(e1, s) + solve_a(e2, s),
        AExp::Mult(e1, e2) => solve_a(e1, s) * solve_a(e2, s),
        AExp::Sub(e1, e2) => solve_a(e1, s) - solve_a(e2, s),
        // Iand : integer bitwise-and (extra constructor in the AST)
        AExp::Iand(e1, e2) => solve_a(e1, s) & solve_a(e2, s),
        // ----- new operators (section 2/3) -----
        // A[[a1 << a2]]s = A[[a1]]s * 2^(A[[a2]]s)   -- Rust's << does exactly this
        AExp::Shl(e1, e2) => solve_a(e1, s) << solve_a(e2, s),
        // A[[a1 >> a2]]s = floor( A[[a1]]s / 2^(A[[a2]]s) ) -- arithmetic shift right
        AExp::Shr(e1, e2) => solve_a(e1, s) >> solve_a(e2, s),
    }
}

// Section 4: truth values are now represented as the STRINGS "tt" / "ff".
pub type BVal = String;

// Helper: evaluate a boolean expression to a plain Rust bool.
// (Allowed as a helper function; keeps the recursive logic readable.)
fn eval_b(e: &BExp, s: &State) -> bool {
    match e {
        BExp::True => true,
        BExp::False => false,
        BExp::Neg(e1) => !eval_b(e1, s),
        BExp::Beq(e1, e2) => eval_b(e1, s) == eval_b(e2, s),
        BExp::Aeq(e1, e2) => solve_a(e1, s) == solve_a(e2, s),
        BExp::Gte(e1, e2) => solve_a(e1, s) >= solve_a(e2, s),
        BExp::And(e1, e2) => eval_b(e1, s) && eval_b(e2, s),
    }
}

// solve_b: BExp -> State -> BVal, returning the strings "tt" / "ff".
pub fn solve_b(e: &BExp, s: &State) -> BVal {
    if eval_b(e, s) {
        "tt".to_string()
    } else {
        "ff".to_string()
    }
}

// state update: to get a new state s[x -> A[[e]]s]
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

// ----------- New initial states for the section-7 tests -----------
// s3 (x = 55)  -- used by test6 (DoWhile)
pub fn s3() -> State {
    let mut s = HashMap::new();
    s.insert("x".to_string(), 55);
    s
}

// s4 (x = 5)   -- used by test7 (Shl)
pub fn s4() -> State {
    let mut s = HashMap::new();
    s.insert("x".to_string(), 5);
    s
}

// s5 (x = 80)  -- used by test8 (Shr)
pub fn s5() -> State {
    let mut s = HashMap::new();
    s.insert("x".to_string(), 80);
    s
}