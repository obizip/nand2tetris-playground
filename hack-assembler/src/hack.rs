#[derive(Debug)]
pub enum Comparison {
    Zero,
    One,
    MinusOne,
    D,
    A,
    NotD,
    NotA,
    MinusD,
    MinusA,
    DPlusOne,
    APlusOne,
    DMinusOne,
    AMinusOne,
    DPlusA,
    DMinusA,
    AMinusD,
    DAndA,
    DOrA,
    M,
    NotM,
    MinusM,
    MPlusOne,
    MMinusOne,
    DPlusM,
    DMinusM,
    MMinusD,
    DAndM,
    DOrM,
}

#[derive(Debug)]
pub enum Destination {
    Null,
    M,
    D,
    DM,
    A,
    AM,
    AD,
    ADM,
}

#[derive(Debug)]
pub enum Jump {
    Null,
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
}

#[derive(Debug)]
pub struct Symbol(pub usize);

#[derive(Debug)]
pub enum Value {
    Symbol(Symbol),
    Constant(usize),
}

#[derive(Debug)]
pub struct CInstruction {
    pub dest: Destination,
    pub comp: Comparison,
    pub jump: Jump,
}

impl CInstruction {
    pub fn new(dest: Destination, comp: Comparison, jump: Jump) -> Self {
        Self { dest, comp, jump }
    }
}

#[derive(Debug)]
pub enum Instruction {
    C(CInstruction),
    A(Value),
}
