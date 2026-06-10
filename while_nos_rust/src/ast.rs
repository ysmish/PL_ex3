// Variable names are strings
pub type VarName = String;

// Arithmetic Expressions (AExp)
#[derive(Debug, Clone)]
pub enum AExp {
    Num(i32),
    Var(VarName),
    Add(Box<AExp>, Box<AExp>),
    Mult(Box<AExp>, Box<AExp>),
    Sub(Box<AExp>, Box<AExp>),
    Iand(Box<AExp>, Box<AExp>),
    // ----- new operators added in this exercise -----
    Shl(Box<AExp>, Box<AExp>), // x << y  =  x * 2^y
    Shr(Box<AExp>, Box<AExp>), // x >> y  =  floor(x / 2^y)
}

// Boolean Expressions (BExp)
#[derive(Debug, Clone)]
pub enum BExp {
    True,
    False,
    Aeq(AExp, AExp),
    Beq(Box<BExp>, Box<BExp>),
    Gte(AExp, AExp),
    Neg(Box<BExp>),
    And(Box<BExp>, Box<BExp>),
}

// Statements (Stm)
#[derive(Debug, Clone)]
pub enum Stm {
    Ass(VarName, AExp),
    Skip,
    Comp(Box<Stm>, Box<Stm>),
    If(BExp, Box<Stm>, Box<Stm>),
    While(BExp, Box<Stm>),
    // ----- new command added in this exercise -----
    // Do S While b  : takes a statement and a boolean expression (in that order)
    DoWhile(Box<Stm>, BExp),
}

// ---------- small helpers to keep the test builders readable ----------
fn var(name: &str) -> AExp {
    AExp::Var(name.to_string())
}
fn num(n: i32) -> AExp {
    AExp::Num(n)
}
fn ass(name: &str, e: AExp) -> Stm {
    Stm::Ass(name.to_string(), e)
}


// ----------- Test Cases Functions  --------
// let test1 = Skip;;
pub fn test1() -> Stm {
    Stm::Skip
}

// let test2 = Comp (Ass ("x", Num 3), Ass ("x", Add(Var "x", Num 1)));;
pub fn test2() -> Stm {
    Stm::Comp(
        Box::new(Stm::Ass("x".to_string(), AExp::Num(3))),
        Box::new(Stm::Ass(
            "x".to_string(),
            AExp::Add(
                Box::new(AExp::Var("x".to_string())),
                Box::new(AExp::Num(1)),
            ),
        )),
    )
}

// let test3 = If(Neg(Aeq(Var "x", Num 1)),Ass ("x", Num 3),Ass ("x", Num 7));;
pub fn test3() -> Stm {
    Stm::If(
        BExp::Neg(Box::new(BExp::Aeq(
            AExp::Var("x".to_string()),
            AExp::Num(1),
        ))),
        Box::new(Stm::Ass("x".to_string(), AExp::Num(3))),
        Box::new(Stm::Ass("x".to_string(), AExp::Num(7))),
    )
}

/*
let test4 = Comp (Ass("y", Num 1),
    While(Neg(Aeq(Var "x", Num 0)),
        Comp(Ass("y", Mult(Var "y", Var "x")),
            Ass("x", Sub(Var "x", Num 1))
        )
    )
);;
*/
pub fn test4() -> Stm {
    Stm::Comp(
        Box::new(Stm::Ass("y".to_string(), AExp::Num(1))),
        Box::new(Stm::While(
            BExp::Neg(Box::new(BExp::Aeq(
                AExp::Var("x".to_string()),
                AExp::Num(0),
            ))),
            Box::new(Stm::Comp(
                Box::new(Stm::Ass(
                    "y".to_string(),
                    AExp::Mult(
                        Box::new(AExp::Var("y".to_string())),
                        Box::new(AExp::Var("x".to_string())),
                    ),
                )),
                Box::new(Stm::Ass(
                    "x".to_string(),
                    AExp::Sub(
                        Box::new(AExp::Var("x".to_string())),
                        Box::new(AExp::Num(1)),
                    ),
                )),
            )),
        )),
    )
}

// =====================================================================
//  test5  ---  the program from Part B, section 5:
//
//    a := 84 ; b := 22 ; c := 0 ;
//    while b != 0 do ( a := a << 1 ; b := b >> 1 )
//
//  Run with state s0.  c is set to 0 and never changed, so it stays 0.
//  Each iteration doubles a and halves b (floor) until b reaches 0.
//  b: 22 -> 11 -> 5 -> 2 -> 1 -> 0   (5 iterations)
//  a: 84 << 5 = 84 * 32 = 2688
//  Result:  a = 2688,  b = 0,  c = 0.
// =====================================================================
pub fn test5() -> Stm {
    // loop body:  a := a << 1 ; b := b >> 1
    let loop_body = Stm::Comp(
        Box::new(ass("a", AExp::Shl(Box::new(var("a")), Box::new(num(1))))),
        Box::new(ass("b", AExp::Shr(Box::new(var("b")), Box::new(num(1))))),
    );
    // while b != 0 do ( ... )
    let while_loop = Stm::While(
        BExp::Neg(Box::new(BExp::Aeq(var("b"), num(0)))),
        Box::new(loop_body),
    );
    // a := 84 ; b := 22 ; c := 0 ; while ...
    Stm::Comp(
        Box::new(ass("a", num(84))),
        Box::new(Stm::Comp(
            Box::new(ass("b", num(22))),
            Box::new(Stm::Comp(
                Box::new(ass("c", num(0))),
                Box::new(while_loop),
            )),
        )),
    )
}

// =====================================================================
//  Section 7 : three new tests, each using a NEW operator, with new
//  initial states (s3, s4, s5 defined in semantics.rs).
// =====================================================================

// test6 : DoWhile (new command).  "Do x := x-10 While x>10"
//         x>10 is written as x >= 11 (equivalent over integers).
//         With s3 (x = 55)  ->  x = 5.
pub fn test6() -> Stm {
    Stm::DoWhile(
        Box::new(ass("x", AExp::Sub(Box::new(var("x")), Box::new(num(10))))),
        BExp::Gte(var("x"), num(11)),
    )
}

// test7 : Shl (new operator).  "x := x << 4"  (x * 16)
//         With s4 (x = 5)  ->  x = 80.
pub fn test7() -> Stm {
    ass("x", AExp::Shl(Box::new(var("x")), Box::new(num(4))))
}

// test8 : Shr (new operator).  "x := x >> 3"  (floor(x / 8))
//         With s5 (x = 80)  ->  x = 10.
pub fn test8() -> Stm {
    ass("x", AExp::Shr(Box::new(var("x")), Box::new(num(3))))
}