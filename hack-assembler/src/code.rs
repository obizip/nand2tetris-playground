use crate::hack::*;

pub trait Coder {
    fn code(&self) -> String;
}

impl Coder for Comparison {
    fn code(&self) -> String {
        use Comparison::*;
        let code = match self {
            Zero => "0101010",
            One => "0111111",
            MinusOne => "0111010",
            D => "0001100",
            A => "0110000",
            NotD => "0001101",
            NotA => "0110001",
            MinusD => "0001111",
            MinusA => "0110011",
            DPlusOne => "0011111",
            APlusOne => "0110111",
            DMinusOne => "0001110",
            AMinusOne => "0110010",
            DPlusA => "0000010",
            DMinusA => "0010011",
            AMinusD => "0000111",
            DAndA => "0000000",
            DOrA => "0010101",
            M => "1110000",
            NotM => "1110001",
            MinusM => "1110011",
            MPlusOne => "1110111",
            MMinusOne => "1110010",
            DPlusM => "1000010",
            DMinusM => "1010011",
            MMinusD => "1000111",
            DAndM => "1000000",
            DOrM => "1010101",
        };
        code.to_string()
    }
}

impl Coder for Destination {
    fn code(&self) -> String {
        use Destination::*;
        let dest = match self {
            Null => "000",
            M => "001",
            D => "010",
            DM => "011",
            A => "100",
            AM => "101",
            AD => "110",
            ADM => "111",
        };
        dest.to_string()
    }
}

impl Coder for Jump {
    fn code(&self) -> String {
        use Jump::*;
        let jump = match self {
            Null => "000",
            JGT => "001",
            JEQ => "010",
            JGE => "011",
            JLT => "100",
            JNE => "101",
            JLE => "110",
            JMP => "111",
        };
        jump.to_string()
    }
}

impl Coder for Symbol {
    fn code(&self) -> String {
        let Symbol(value) = self;
        format!("{value:015b}")
    }
}

impl Coder for Value {
    fn code(&self) -> String {
        match self {
            Value::Symbol(symbol) => symbol.code(),
            Value::Constant(constant) => {
                format!("{constant:015b}")
            }
        }
    }
}

impl Coder for CInstruction {
    fn code(&self) -> String {
        "111".to_string() + &self.comp.code() + &self.dest.code() + &self.jump.code()
    }
}

impl Coder for Instruction {
    fn code(&self) -> String {
        match self {
            Instruction::C(instruction) => instruction.code(),
            Instruction::A(value) => "0".to_string() + &value.code(),
        }
    }
}
