use std::fmt::Display;
use std::io::BufRead;
use itertools::Itertools;

pub fn part_1(input: Box<dyn BufRead>) -> anyhow::Result<impl Display> {
    let mut lines = input.lines();
    let a = lines.next().unwrap()?;
    let a = a.strip_prefix("Register A: ").unwrap().parse::<u32>()?;
    let b = lines.next().unwrap()?;
    let b = b.strip_prefix("Register B: ").unwrap().parse::<u32>()?;
    let c = lines.next().unwrap()?;
    let c = c.strip_prefix("Register C: ").unwrap().parse::<u32>()?;
    
    lines.next().unwrap()?;
    let program = lines.next().unwrap()?
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect_vec();
    let output = {
        let mut output = Vec::new();
        let (mut a, mut b, mut c) = (a, b, c);
        let mut pointer = 0;
        while pointer < program.len() - 1 {
            let (instruction, operand) = (program[pointer], program[pointer + 1]);
            
            let operand = match operand {
                4 => a,
                5 => b,
                6 => c,
                _ => operand,
            };
            
            match instruction {
                0 => {
                    a >>= operand;
                },
                1 => {
                    b ^= operand;
                },
                2 => {
                    b = operand % 8;
                },
                3 => {
                    if a != 0 {
                        pointer = operand as usize;
                    }
                },
                4 => {
                    b ^= c;
                },
                5 => {
                    output.push(operand % 8);
                },
                6 => {
                    b = a >> operand;
                }
                7 => {
                    c = a >> operand;
                }
                _ => panic!("{instruction}")
            }
            
            if instruction != 3 && a != 0 {
                pointer += 2;
            }
        }
        
        output
    };

    Ok(output.into_iter().join(","))
}