// imports
mod ast;
mod semantics;
mod nos;

use ast::{Stm, test1, test2, test3, test4, test5, test6, test7, test8, test9};
use semantics::{State, s0, s1, s2};
use nos::nos; 


fn main() {
    println!("--- Running Test Cases in Rust ---");
    
    // Helper to run a test and print the result for a specific variable
    let run_test = |test_name: &str, stm: Stm, initial_state: State, var: &str| {
        let final_state = nos((stm, initial_state)); 
        let value = final_state.get(var).unwrap_or(&0);
        println!("Test {} - {} = {}", test_name, var, value);
    };
    
    // test1 (Skip) starting with s0 (x=1) -> x=1
    run_test("test1", test1(), s0(), "x"); 
    
    // test2 (x=3; x=x+1) starting with s0 (x=1) -> x=4
    run_test("test2", test2(), s0(), "x"); 
    
    // test3 (If Neg(x==1)) starting with s0 (x=1). Neg(True) is False, so x=7
    run_test("test3", test3(), s0(), "x");
    
    // test4 (While loop, Factorial) starting with s1 (x=5) -> x=0
    run_test("test4", test4(), s1(), "x");

    // test4 (While loop, Factorial) starting with s1 (x=5) -> y=120
    run_test("test4", test4(), s1(), "y");
}