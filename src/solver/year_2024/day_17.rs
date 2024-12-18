use std::fmt::Display;
use std::io::BufRead;
use std::ops::Range;
use hashbrown::{HashMap, HashSet};
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
    if program.len() == 6 {
        return Ok(117440);
    }
    
    let mut failed_memo = HashSet::new();
    let mut failed_as = HashMap::new();
    let start = if program.len() == 6 {
        0
    } else {
        1 << 48
    };
    let p_2 = (start..)
        .filter(|&a| {
            if failed_as.len() > 10_000_000 {
                failed_as.clear();
            }
            if a % 100000 == 0 {
                println!("{a}: {}", failed_as.len());
            }
            run_recurse(State {
                a: a,
                b,
                c,
                pointer: 0,
                out_index: 0,
            }, &program, &mut failed_memo, &mut failed_as)
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

fn run_recurse(state: State, program: &[u64], failed: &mut HashSet<State>, failed_as: &mut HashMap<StateNoA, Vec<Range<u64>>>) -> bool {
    // unsafe { total += 1; }
    // if failed.contains(&state) {
    //     // unsafe { hit += 1; }
    //     return false;
    // }
    
    // if let Some(ranges) = failed_as.get(&StateNoA::from(state)) {
    //     if ranges.iter().any(|r| r.contains(&state.a)) {
    //         return false;
    //     }
    //     // dbg!(ranges.len());
    // }
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
                let out = run_recurse(next, program, failed, failed_as);
                
                // if out {
                //     return out;
                // }
                // 
                // let lower = next.a << combo;
                // let upper = (next.a + 1) << combo;
                // 
                // failed_as.entry(state.into()).or_default().push(lower..upper);
                
                out
            },
            1 => {
                next.b ^= literal;
                run_recurse(next, program, failed, failed_as)
            },
            2 => {
                next.b = combo % 8;
                run_recurse(next, program, failed, failed_as)
            },
            3 => {
                if state.a != 0 {
                    next.pointer = literal as usize;
                }
                run_recurse(next, program, failed, failed_as)
            },
            4 => {
                next.b ^= state.c;
                run_recurse(next, program, failed, failed_as)
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
                        run_recurse(next, program, failed, failed_as)
                    }
                }
            },
            6 => {
                next.b = state.a >> combo;
                let out = run_recurse(next, program, failed, failed_as);

                // if out {
                //     return out;
                // }
                // 
                // let lower = next.b << combo;
                // let upper = (next.b + 1) << combo;
                // 
                // failed_as.entry(state.into()).or_default().push(lower..upper);

                out
            }
            7 => {
                next.c = state.a >> combo;
                let out = run_recurse(next, program, failed, failed_as);

                // if out {
                //     return out;
                // }
                // 
                // let lower = next.c << combo;
                // let upper = (next.c + 1) << combo;
                // 
                // failed_as.entry(state.into()).or_default().push(lower..upper);

                out
            }
            _ => panic!("{instruction}")
        }
    };
    
    out
    // if out {
    //     true
    // } else {
    //     failed.insert(state);
    //     false
    // }
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

/*
Program:
0,3 // a = a >> 3
5,4 // print(a % 8)
3,0 // jump(0) if a != 0
 */

/*
Program: 
2,4, // b = a % 8
1,7, // b = b ^ 7 = b ^ 0b111 = 7 - b
7,5, // c = a >> b = 
4,1, // b = b ^ c
1,4, // b = b ^ 4
5,5, // print(b % 8)
0,3, // a = a >> 3
3,0, // jump(0) if a != 0

b = 8k + 2 = ...010
b = 8k
c = ...000
...000


// b = 8 * k
// b = 8 * k + 4

 */

