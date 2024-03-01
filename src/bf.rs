use alloc::vec::Vec;
use nom::{branch::alt, bytes::complete::{is_not, tag}, multi::many0, sequence::delimited, IResult};

use crate::{print, serial::serial_recv};

#[derive(PartialEq,Clone)]
pub enum Instruction {
    Add, Sub,
    Lower, Higher,
    Loop(Vec<Instruction>),
    Output, Input,
    Nop,
}

pub struct Interpreter {
    pub mem: Vec<u8>,
    pub p: usize,
}

impl Interpreter {
    pub fn run(&mut self, src: Vec<Instruction>) {
        if self.mem.len() <= self.p {
            self.mem.resize(self.p + 1024, 0);
        }
        let mut position = 0;
        //print!("{}",self.src[position]);
        while position < src.len() {
            match &src[position] {
                Instruction::Add => self.mem[self.p] = self.mem[self.p].wrapping_add(1),
                Instruction::Sub => self.mem[self.p] = self.mem[self.p].wrapping_sub(1),
                Instruction::Lower => self.p -= 1,
                Instruction::Higher => self.p += 1,
                Instruction::Loop(loop_part) => while self.mem[self.p] != 0 { self.run(loop_part.to_vec())},
                Instruction::Output => print!("{}", char::from_u32(self.mem[self.p] as u32).unwrap()),
                Instruction::Input => self.mem[self.p] = unsafe{serial_recv()},
                _ => (),
            }
            position += 1;
        }
    }
}

fn parse_add(input: &str) -> IResult<&str,Instruction> {
    let (input, _) = tag("+")(input)?;
    Ok((input, Instruction::Add))
}

fn parse_sub(input: &str) -> IResult<&str,Instruction> {
    let (input, _) = tag("-")(input)?;
    Ok((input, Instruction::Sub))
}

fn parse_lower(input: &str) -> IResult<&str,Instruction> {
    let (input, _) = tag("<")(input)?;
    Ok((input, Instruction::Lower))
}

fn parse_higher(input: &str) -> IResult<&str,Instruction> {
    let (input, _) = tag(">")(input)?;
    Ok((input, Instruction::Higher))
}

fn parse_output(input: &str) -> IResult<&str,Instruction> {
    let (input, _) = tag(".")(input)?;
    Ok((input, Instruction::Output))
}

fn parse_input(input: &str) -> IResult<&str,Instruction> {
    let (input, _) = tag(",")(input)?;
    Ok((input, Instruction::Input))
}

fn parse_nop(input: &str) -> IResult<&str,Instruction> {
    let (input, _) = is_not("+-<>.,[]")(input)?;
    Ok((input, Instruction::Nop))
}

fn parse_loop(input: &str) -> IResult<&str,Instruction> {
    let (input, instrs) = delimited(tag("["),many0(parse),tag("]"))(input)?;
    Ok((input, Instruction::Loop(instrs)))
}


pub fn parse(input: &str) -> IResult<&str,Instruction> {
    alt((parse_nop, parse_add, parse_sub, parse_lower, parse_higher, parse_output, parse_input, parse_loop))(input)
}