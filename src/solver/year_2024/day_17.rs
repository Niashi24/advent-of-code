use std::fmt::Display;
use std::io::BufRead;
use std::ops::Range;
use hashbrown::HashSet;
use itertools::Itertools;

fn parse(input: Box<dyn BufRead>) -> anyhow::Result<([u64; 3], Vec<u64>)> {
    let mut lines = input.lines();
    let a = lines.next().unwrap()?;
    let a = a.strip_prefix("Register A: ").unwrap().parse::<u64>()?;
    let b = lines.next().unwrap()?;
    let b = b.strip_prefix("Register B: ").unwrap().parse::<u64>()?;
    let c = lines.next().unwrap()?;
    let c = c.strip_prefix("Register C: ").unwrap().parse::<u64>()?;

    // dbg!(a, b, c);

    lines.next().unwrap()?;
    let program = lines.next().unwrap()?
        .strip_prefix("Program: ").unwrap()
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();

    Ok(([a, b, c], program))
}

pub fn part_1(input: Box<dyn BufRead>) -> anyhow::Result<impl Display> {
    let ([a, b, c], program) = parse(input)?;
    let p_1 = run(a, b, c, &program).into_iter().join(",");

    Ok(p_1)
}

pub fn part_2(input: Box<dyn BufRead>) -> anyhow::Result<impl Display> {
    let ([_, b, c], program) = parse(input)?;
    let mut failed_memo = HashSet::new();
    let p_2 = (0..)
        .filter(|&a| {
            if a % 100000 == 0 {
                println!("{a}: {}", failed_memo.len());
            }
            run_recurse(State {
                a: a,
                b,
                c,
                pointer: 0,
                out_index: 0,
            }, &program, &mut failed_memo)
        })
        .next().unwrap();
    Ok(dbg!(p_2))
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct State {
    a: u64,
    b: u64,
    c: u64,
    pointer: usize,
    out_index: usize,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct StateNoA {
    b: u64,
    c: u64,
    pointer: usize,
    out_index: usize,
}

impl From<State> for StateNoA {
    fn from(value: State) -> Self {
        Self {
            b: value.b,
            c: value.c,
            pointer: value.pointer,
            out_index: value.out_index,
        }
    }
}


static mut hit: usize = 0;
static mut total: usize = 0;

fn run_recurse(state: State, program: &[u64], failed: &mut HashSet<State>) -> bool {
    // unsafe { total += 1; }
    if failed.contains(&state) {
        // unsafe { hit += 1; }
        return false;
    }
    // unsafe { println!("hit rate: {:.2}", hit as f32 / total as f32 * 100.0) };
    // println!("{:?}", state);
    // dbg!(failed.len());
    
    let out = if state.pointer >= program.len() - 1 {
        false
    } else {
        let mut next = state;
        let (instruction, literal) = (program[state.pointer], program[state.pointer + 1]);

        let combo = match literal {
            4 => state.a,
            5 => state.b,
            6 => state.c,
            _ => literal,
        };
        
        next.pointer += 2;

        match instruction {
            0 => {
                next.a >>= combo;
                let out = run_recurse(next, program, failed);
                
                if out {
                    return out;
                }
                
                let lower = next.a << combo;
                let upper = (next.a + 1) << combo;
                
                // dbg!(out);
                // println!("removing: {}", upper - lower);
                for a in lower..upper {
                    // println!("failed: {a}");
                    failed.insert(State {
                        a,
                        ..state
                    });
                }
                
                out
            },
            1 => {
                next.b ^= literal;
                run_recurse(next, program, failed)
            },
            2 => {
                next.b = combo % 8;
                run_recurse(next, program, failed)
            },
            3 => {
                if state.a != 0 {
                    next.pointer = literal as usize;
                }
                run_recurse(next, program, failed)
            },
            4 => {
                next.b ^= state.c;
                run_recurse(next, program, failed)
            },
            5 => {
                let output = combo % 8;
                if program[state.out_index] != output {
                    false
                } else {
                    next.out_index += 1;
                    if next.out_index == program.len() {
                        true
                    } else {
                        run_recurse(next, program, failed)
                    }
                }
            },
            6 => {
                next.b = state.a >> combo;
                run_recurse(next, program, failed)
            }
            7 => {
                next.c = state.a >> combo;
                run_recurse(next, program, failed)
            }
            _ => panic!("{instruction}")
        }
    };
    
    if out {
        true
    } else {
        failed.insert(state);
        false
    }
}

fn run(mut a: u64, mut b: u64, mut c: u64, program: &[u64]) -> Vec<u64> {
    // println!("{a}");
    let mut output = Vec::new();
    let (mut a, mut b, mut c) = (a, b, c);
    let mut pointer = 0;
    while pointer < program.len() - 1 {
        let (instruction, literal) = (program[pointer], program[pointer + 1]);

        let combo = match literal {
            4 => a,
            5 => b,
            6 => c,
            _ => literal,
        };

        match instruction {
            0 => {
                a >>= combo;
            },
            1 => {
                b ^= literal;
            },
            2 => {
                b = combo % 8;
            },
            3 => {
                if a != 0 {
                    pointer = literal as usize;
                }
            },
            4 => {
                b ^= c;
            },
            5 => {
                output.push(combo % 8);
            },
            6 => {
                b = a >> combo;
            }
            7 => {
                c = a >> combo;
            }
            _ => panic!("{instruction}")
        }

        if !(instruction == 3 && a != 0) {
            pointer += 2;
        }
    }

    output
}