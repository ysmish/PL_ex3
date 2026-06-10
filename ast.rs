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
}







// ----------- Test Cases Functiond  --------
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
