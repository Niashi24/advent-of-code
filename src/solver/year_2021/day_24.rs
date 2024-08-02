use std::collections::HashSet;
use std::io::BufRead;
use std::str::FromStr;
use itertools::Itertools;
use smallvec::SmallVec;
use crate::day::CombinedSolver;

pub struct Day24;

impl CombinedSolver for Day24 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let instructions = input.lines().map(Result::unwrap)
            .map(|l| Instruction::from_str(&l))
            .collect::<Result<Vec<_>, _>>().unwrap();


        let mut memo = Failed::new();
        let part_1 = solve(Memory::default(), 0, ModelNumber::new(), &instructions, &mut memo).unwrap();
        let part_1 = part_1.into_iter().join("");

        Ok((part_1, "".to_string()))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default)]
struct Memory {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl Memory {
    fn get_register(&self, r: Register) -> i64 {
        match r {
            Register::W => self.w,
            Register::X => self.x,
            Register::Y => self.y,
            Register::Z => self.z   ,
        }
    }

    fn set_register(&mut self, r: Register, v: i64) {
        match r {
            Register::W => self.w = v,
            Register::X => self.x = v,
            Register::Y => self.y = v,
            Register::Z => self.z = v,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Register {
    W, X, Y, Z
}

impl FromStr for Register {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Self::W),
            "x" => Ok(Self::X),
            "y" => Ok(Self::Y),
            "z" => Ok(Self::Z),
            _ => Err(format!("unknown register: {:?}", s))
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum OType {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

impl OType {
    fn eval(&self, a: i64, b: i64) -> Option<i64> {
        match self {
            OType::Add => Some(a + b),
            OType::Mul => Some(a * b),
            OType::Div => a.checked_div(b),
            OType::Mod => (a >= 0 && b > 0).then(|| a % b),
            OType::Eql => Some((a == b) as i64),
        }
    }
}

impl FromStr for OType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" => Ok(Self::Add),
            "mul" => Ok(Self::Mul),
            "div" => Ok(Self::Div),
            "mod" => Ok(Self::Mod),
            "eql" => Ok(Self::Eql),
            _ => Err(format!("unknown op: {:?}", s))
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Operation(OType, Register, Input);

impl Operation {
    fn execute(&self, mem: &mut Memory) -> bool {
        let a = mem.get_register(self.1);
        let b = self.2.eval(mem);
        if let Some(v) = self.0.eval(a, b) {
            mem.set_register(self.1, v);
            true
        } else {
            false
        }
    }
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((op, out, inp)) = s.split_whitespace().next_tuple() else {
            return Err(format!("No three args: {:?}", s));
        };

        let op = op.parse()?;
        let out = out.parse()?;
        let inp = inp.parse()?;

        Ok(Self(op, out, inp))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Instruction {
    I(Register),
    O(Operation),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(op) = s.parse::<Operation>() {
            Ok(Self::O(op))
        } else {
            let Some((op, r)) = s.split_once(" ") else {
                return Err(format!("invalid op: {:?}", s));
            };
            if op != "inp" {
                return Err(format!("invalid op: {:?}", s));
            }
            let r = r.parse()?;
            Ok(Self::I(r))
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Input {
    R(Register),
    V(i64),
}

impl Input {
    fn eval(&self, mem: &Memory) -> i64 {
        match *self {
            Input::R(r) => mem.get_register(r),
            Input::V(v) => v,
        }
    }
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(r) = s.parse::<Register>() {
            Ok(Input::R(r))
        } else if let Ok(i) = s.parse::<i64>() {
            Ok(Input::V(i))
        } else {
            Err(format!("invalid input: {:?}", s))
        }
    }
}

type ModelNumber = SmallVec<[u8; 14]>;

type Failed = HashSet<(Memory, usize)>;

static mut hit: usize = 0;
static mut total: usize = 0;

fn solve(mut state: Memory, mut i: usize, cur: ModelNumber, instructions: &[Instruction], failed: &mut Failed) -> Option<ModelNumber> {
    unsafe { total += 1; }
    if failed.contains(&(state, i)) {
        unsafe { hit += 1}
        return None;
    }

    print!("{} {}: ", failed.len(), unsafe { total - hit });
    println!("{}", cur.iter().copied().join(""));

    // dbg!(failed.len());

    loop {
        match instructions.get(i) {
            None => {
                return if state.z == 0 {
                    Some(cur)
                } else {
                    None
                };
            }
            Some(&op) => {
                match op {
                    Instruction::I(r) => {
                        // i += 1;
                        for n in (1u8..=9).rev() {
                            let mut state = state;
                            state.set_register(r, n as i64);
                            let mut model = cur.clone();
                            model.push(n);

                            if let Some(sol) = solve(state, i + 1, model, instructions, failed) {
                                return Some(sol);
                            } else {
                                failed.insert((state, i));
                            }
                        }
                        failed.insert((state, i));
                        return None;
                    }
                    Instruction::O(op) => {
                        i += 1;
                        op.execute(&mut state);
                    }
                }
            }
        }
    }
}
