mod code;
mod hack;
mod parse;
use code::*;
use parse::*;
use std::env;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn main() -> ParseResult<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("USAGE: {} <filename>", &args[0]);
        return Ok(());
    }
    let filename = &args[1];
    let lines = read_lines(filename);
    let instructions = parse_lines(lines)?;
    for inst in instructions {
        println!("{}", inst.code());
    }
    Ok(())
}
