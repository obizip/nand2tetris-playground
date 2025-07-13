use crate::hack::*;
use regex::Regex;
use std::collections::HashMap;
use thiserror::Error;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
struct Symbol1st(String);

#[derive(Debug)]
enum Value1st {
    Symbol(Symbol1st),
    Constant(usize),
}

#[derive(Debug)]
enum Instruction1st {
    C(CInstruction),
    A(Value1st),
    L(Symbol1st),
}

type SymbolTable = HashMap<String, usize>;

fn insert_default_values(table: &mut SymbolTable) {
    let labels = [
        ("SP", 0),
        ("LCL", 1),
        ("ARG", 2),
        ("THIS", 3),
        ("THAT", 4),
        ("SCREEN", 16384),
        ("KBD", 24576),
    ];
    for (k, v) in labels.into_iter() {
        table.insert(k.to_string(), v);
    }
    for i in 0..=15 {
        table.insert(format!("R{i}"), i);
    }
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("invalid symbol `{0}`")]
    InvalidSymbol(String),
    // #[error("value is empty")]
    // EmptyValue,
    #[error("invalid comparison `{0}`")]
    InvalidComparison(String),
    #[error("invalid destination `{0}`")]
    InvalidDestination(String),
    #[error("invalid jump `{0}`")]
    InvalidJump(String),
    #[error("invalid instruction `{0}`")]
    InvalidInstruction(String),
    // #[error("label must be symbol `{0}`")]
    // LabelMustBeSymbol(usize),
}

pub fn parse_lines(lines: Vec<String>) -> ParseResult<Vec<Instruction>> {
    let (instructions, table) = parse_lines_1st(lines)?;
    Ok(parse_instructions(instructions, table))
}

fn parse_instructions(
    instruction1sts: Vec<Instruction1st>,
    table: SymbolTable,
) -> Vec<Instruction> {
    let mut table = table.to_owned();
    let mut var_idx = 16;

    let mut instructions = Vec::new();

    for instruction1st in instruction1sts {
        let instruction = match instruction1st {
            Instruction1st::C(inst) => Instruction::C(inst),
            Instruction1st::A(value1st) => {
                let value = match value1st {
                    Value1st::Constant(c) => Value::Constant(c),
                    Value1st::Symbol(Symbol1st(symbol)) => match table.get(&symbol) {
                        Some(val) => Value::Symbol(Symbol(*val)),
                        None => {
                            table.insert(symbol.clone(), var_idx.clone());
                            var_idx += 1;
                            Value::Symbol(Symbol(var_idx - 1))
                        }
                    },
                };
                Instruction::A(value)
            }
            Instruction1st::L(_) => {
                continue;
            }
        };
        instructions.push(instruction);
    }
    instructions
}

fn parse_lines_1st(lines: Vec<String>) -> ParseResult<(Vec<Instruction1st>, SymbolTable)> {
    let mut instructions = Vec::new();
    let mut table = SymbolTable::new();
    insert_default_values(&mut table);
    let mut line_count = 0;

    for line in lines {
        let line = line.trim();
        if line.starts_with("//") || line.is_empty() {
            continue;
        }

        let instruction = Instruction1st::parse(line)?;
        match &instruction {
            Instruction1st::L(Symbol1st(symbol)) => {
                table.insert(symbol.to_owned(), line_count);
            }
            Instruction1st::A(_) => {
                line_count += 1;
            }
            Instruction1st::C(_) => {
                line_count += 1;
            }
        }

        instructions.push(instruction);
    }
    Ok((instructions, table))
}

// fn parse_lines_2nd(lines: Vec<Instruction1st>) -> ParseResult<Vec<Instruction>> {
//     let mut instructions = Vec::new();
// }

pub trait Parser<T> {
    fn parse(input: &str) -> ParseResult<T>;
}

impl Parser<Comparison> for Comparison {
    fn parse(input: &str) -> ParseResult<Comparison> {
        use Comparison::*;
        let comp = match input {
            "0" => Zero,
            "1" => One,
            "-1" => MinusOne,
            "D" => D,
            "A" => A,
            "!D" => NotD,
            "!A" => NotA,
            "-D" => MinusD,
            "-A" => MinusA,
            "D+1" => DPlusOne,
            "A+1" => APlusOne,
            "D-1" => DMinusOne,
            "A-1" => AMinusOne,
            "D+A" => DPlusA,
            "D-A" => DMinusA,
            "A-D" => AMinusD,
            "D&A" => DAndA,
            "D|A" => DOrA,
            "M" => M,
            "!M" => NotM,
            "-M" => MinusM,
            "M+1" => MPlusOne,
            "M-1" => MMinusOne,
            "D+M" => DPlusM,
            "D-M" => DMinusM,
            "M-D" => MMinusD,
            "D&M" => DAndM,
            "D|M" => DOrM,
            _ => {
                return Err(ParseError::InvalidComparison(input.to_string()));
            }
        };

        Ok(comp)
    }
}

impl Parser<Destination> for Destination {
    fn parse(input: &str) -> ParseResult<Destination> {
        use Destination::*;
        let dest = match input {
            "M" => M,
            "D" => D,
            "DM" => DM,
            "MD" => DM,
            "A" => A,
            "AM" => AM,
            "AD" => AD,
            "DA" => AD,
            "ADM" => ADM,
            _ => {
                return Err(ParseError::InvalidDestination(input.to_string()));
            }
        };

        Ok(dest)
    }
}

impl Parser<Jump> for Jump {
    fn parse(input: &str) -> ParseResult<Jump> {
        use Jump::*;
        let jump = match input {
            "JGT" => JGT,
            "JEQ" => JEQ,
            "JGE" => JGE,
            "JLT" => JLT,
            "JNE" => JNE,
            "JLE" => JLE,
            "JMP" => JMP,
            _ => {
                return Err(ParseError::InvalidJump(input.to_string()));
            }
        };

        Ok(jump)
    }
}

impl Parser<Symbol1st> for Symbol1st {
    fn parse(input: &str) -> ParseResult<Symbol1st> {
        let re = Regex::new(r"[A-Za-z\_\.\$\:]+[A-Za-z0-9\_\.\$\:]*").unwrap();
        if !re.is_match(&input) {
            return Err(ParseError::InvalidSymbol(input.to_string()));
        }

        Ok(Symbol1st(input.to_string()))
    }
}

impl Parser<Value1st> for Value1st {
    fn parse(input: &str) -> ParseResult<Value1st> {
        if input.chars().all(|c| c.is_digit(10)) {
            return Ok(Value1st::Constant(
                input.parse().expect("should be constant"),
            ));
        }

        let symbol = Symbol1st::parse(input)?;
        Ok(Value1st::Symbol(symbol))
    }
}

fn parse_a_instruction(line: &str) -> ParseResult<Option<Instruction1st>> {
    if !line.starts_with('@') {
        return Ok(None);
    }

    let value = Value1st::parse(&line[1..])?;
    Ok(Some(Instruction1st::A(value)))
}

fn parse_c_instruction(line: &str) -> ParseResult<Option<Instruction1st>> {
    let mut cur = line;
    let dest = if let Some(pos) = cur.find('=') {
        let dest = &cur[0..pos];
        cur = &cur[pos + 1..];
        Destination::parse(&dest)?
    } else {
        Destination::Null
    };

    let jump = if let Some(pos) = cur.find(';') {
        let jump = &cur[pos + 1..];
        cur = &cur[0..pos];
        Jump::parse(&jump)?
    } else {
        Jump::Null
    };

    let comp = Comparison::parse(&cur)?;

    let c_instruction = CInstruction::new(dest, comp, jump);

    Ok(Some(Instruction1st::C(c_instruction)))
}

fn parse_l_instruction(line: &str) -> ParseResult<Option<Instruction1st>> {
    if !(line.starts_with("(") && line.ends_with(")")) {
        return Ok(None);
    }

    let symbol = Symbol1st::parse(&line[1..line.len() - 1])?;
    Ok(Some(Instruction1st::L(symbol)))
}

impl Parser<Instruction1st> for Instruction1st {
    fn parse(line: &str) -> ParseResult<Instruction1st> {
        if let Some(instruction) = parse_a_instruction(line)? {
            return Ok(instruction);
        }

        if let Some(instruction) = parse_l_instruction(line)? {
            return Ok(instruction);
        }

        if let Some(instruction) = parse_c_instruction(line)? {
            return Ok(instruction);
        }

        Err(ParseError::InvalidInstruction(line.to_string()))
    }
}
