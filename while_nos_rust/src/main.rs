// imports
mod ast;
mod semantics;
mod nos;

use ast::{Stm, test1, test2, test3, test4, test5, test6, test7, test8};
use semantics::{State, s0, s1, s3, s4, s5};
use nos::nos;

fn main() {
    println!("--- Running Test Cases in Rust ---");

    // Helper to run a test and print the result for a specific variable
    let run_test = |test_name: &str, stm: Stm, initial_state: State, var: &str| {
        let final_state = nos((stm, initial_state));
        let value = final_state.get(var).unwrap_or(&0);
        println!("Test {} - {} = {}", test_name, var, value);
    };

    // test1 (Skip) with s0 (x=1) -> x=1
    run_test("test1", test1(), s0(), "x");

    // test2 (x=3; x=x+1) with s0 (x=1) -> x=4
    run_test("test2", test2(), s0(), "x");

    // test3 (If Neg(x==1)) with s0 (x=1) -> x=7
    run_test("test3", test3(), s0(), "x");

    // test4 (While factorial) with s1 (x=5) -> x=0, y=120
    run_test("test4", test4(), s1(), "x");
    run_test("test4", test4(), s1(), "y");

    // ---- Part B, section 5 : program run with state s0 ----
    // a := 84 ; b := 22 ; c := 0 ; while b!=0 do (a := a<<1 ; b := b>>1)
    run_test("test5", test5(), s0(), "a"); // a = 2688
    run_test("test5", test5(), s0(), "b"); // b = 0
    run_test("test5", test5(), s0(), "c"); // c = 0

    // ---- Part B, section 7 : three new tests with new states ----
    run_test("test6", test6(), s3(), "x"); // DoWhile : 55 -> 5
    run_test("test7", test7(), s4(), "x"); // Shl     : 5  -> 80
    run_test("test8", test8(), s5(), "x"); // Shr     : 80 -> 10
}