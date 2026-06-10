//planet smishky 1022 $ cargo run
//   Compiling while_nos_rust v0.1.0 (/u/students/smishky/PL_ex3/while_nos_rust)
//warning: variant `Iand` is never constructed
//  --> src/ast.rs:12:5
//   |
// 6 | pub enum AExp {
//   |          ---- variant in this enum
//...
//12 |     Iand(Box<AExp>, Box<AExp>),
//   |     ^^^^
//   |
//   = note: `AExp` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis
//   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default
//
//warning: variants `True`, `False`, `Beq`, and `And` are never constructed
//  --> src/ast.rs:21:5
//   |
//20 | pub enum BExp {
//   |          ---- variants in this enum
//21 |     True,
//   |     ^^^^
//22 |     False,
//   |     ^^^^^
//23 |     Aeq(AExp, AExp),
//24 |     Beq(Box<BExp>, Box<BExp>),
//   |     ^^^
//...
//27 |     And(Box<BExp>, Box<BExp>),
//   |     ^^^
//   |
//   = note: `BExp` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis
//
//warning: function `s2` is never used
//  --> src/semantics.rs:78:8
//   |
//78 | pub fn s2() -> State {
//   |        ^^
//
//warning: `while_nos_rust` (bin "while_nos_rust") generated 3 warnings
//    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.58s
//     Running `target/debug/while_nos_rust`
//--- Running Test Cases in Rust ---
//Test test1 - x = 1
//Test test2 - x = 4
//Test test3 - x = 7
//Test test4 - x = 0
//Test test4 - y = 120
//Test test5 - a = 2688
//Test test5 - b = 0
//Test test5 - c = 0
//Test test6 - x = 5
//Test test7 - x = 80
//Test test8 - x = 10

use crate::ast::*;
use crate::semantics::*;

// The main Natural Operational Semantics function:
// nos: (Stm, State) -> State
pub fn nos(c: (Stm, State)) -> State {
    let (stm, state) = c;

    match stm {
        // Assignment: [ass]
        //   (x := a, s) -> s[x -> A[[a]]s]
        Stm::Ass(x, e) => update(&x, &e, &state),

        // Skip: [skip]
        //   (skip, s) -> s
        Stm::Skip => state,

        // Composition: [comp]
        //   (S1, s) -> s'    (S2, s') -> s''
        //   --------------------------------
        //          (S1; S2, s) -> s''
        Stm::Comp(s1, s2) => {
            let s_prime = nos((*s1, state)); // run S1 on s  -> s'
            nos((*s2, s_prime))              // run S2 on s' -> s''
        }

        // If: [if_tt] and [if_ff]
        // (solve_b now returns "tt" / "ff" instead of a bool)
        Stm::If(b, s1, s2) => {
            if solve_b(&b, &state) == "tt" {
                nos((*s1, state)) // B[[b]]s = tt
            } else {
                nos((*s2, state)) // B[[b]]s = ff
            }
        }

        // While: [while_tt] and [while_ff]
        Stm::While(b, s) => {
            if solve_b(&b, &state) == "tt" {
                // [while_tt]: B[[b]]s = tt
                let s_prime = nos(((*s).clone(), state)); // run body once -> s'
                nos((Stm::While(b, s), s_prime))          // loop again on s'
            } else {
                // [while_ff]: B[[b]]s = ff  ->  (while b do S, s) -> s
                state
            }
        }

        // DoWhile: [dowhile_tt] and [dowhile_ff]   (from Part A, Q2)
        //
        //   [dowhile_ff]   (S, s) -> s'                       if B[[b]]s' = ff
        //                  ----------------------
        //                  (Do S While b, s) -> s'
        //
        //   [dowhile_tt]   (S, s) -> s'   (Do S While b, s') -> s''   if B[[b]]s' = tt
        //                  ------------------------------------------
        //                  (Do S While b, s) -> s''
        //
        // The body S is ALWAYS executed once; only then is b checked on the
        // resulting state. The rules never mention `while`.
        Stm::DoWhile(s, b) => {
            let s_prime = nos(((*s).clone(), state)); // always run the body once
            if solve_b(&b, &s_prime) == "tt" {
                nos((Stm::DoWhile(s, b), s_prime))    // b holds -> repeat
            } else {
                s_prime                                // b fails -> stop
            }
        }
    }
}