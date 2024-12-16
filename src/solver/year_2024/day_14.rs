use std::cmp::Ordering;
use std::fmt::Display;
use std::io::{stdin, BufRead};
use hashbrown::HashSet;
use itertools::Itertools;
use regex::Regex;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

pub fn part_1(input: Box<dyn BufRead>) -> anyhow::Result<impl Display> {
    let regex = Regex::new(r#"p=(\d+),(\d+) v=(-?\d+),(-?\d+)"#)?;
    
    const N: i32 = 100;
    
    let mut q = [0; 4];
    input.lines().map(Result::unwrap)
        .map(|line| {
            let capture = regex.captures(&line).unwrap();
            [1, 2, 3, 4]
                .map(|i| capture[i].parse::<i32>().unwrap())
        })
        .map(|[p_x, p_y, v_x, v_y]| {
            ((p_x + v_x * N).rem_euclid(WIDTH), (p_y + v_y * N).rem_euclid(HEIGHT))
        })
        .for_each(|(x, y)| { 
            // println!("({x}, {y})");
            match (x.cmp(&(WIDTH / 2)), y.cmp(&(HEIGHT / 2))) {
                (Ordering::Less, Ordering::Less) => q[0] += 1,
                (Ordering::Less, Ordering::Greater) => q[1] += 1,
                (Ordering::Greater, Ordering::Less) => q[2] += 1,
                (Ordering::Greater, Ordering::Greater) => q[3] += 1,
                (_, _) => {}
            }
        });
    
    // dbg!(q);
    let p_1 = q[0] * q[1] * q[2] * q[3];
    
    Ok(p_1)    
}

const S: &str = include_str!("../../../data/2024/full-14-24.txt");

pub fn part_2() {
    let regex = Regex::new(r#"p=(\d+),(\d+) v=(-?\d+),(-?\d+)"#).unwrap();
    let r = S.lines()
        .map(|line| {
            let capture = regex.captures(&line).unwrap();
            [1, 2, 3, 4]
                .map(|i| capture[i].parse::<i32>().unwrap())
        }).collect_vec();
    
    
    
    'outer: for n in 0.. {
        let mut robots = HashSet::new();
        for xy in r.iter().copied()
            .map(|[p_x, p_y, v_x, v_y]| {
                ((p_x + v_x * n).rem_euclid(WIDTH) as usize, (p_y + v_y * n).rem_euclid(HEIGHT) as usize)
            }) {
            if !robots.insert(xy) {
                continue 'outer;
            }
        }
        
        // let robots = r.iter().copied()
        //     .map(|[p_x, p_y, v_x, v_y]| {
        //         ((p_x + v_x * n).rem_euclid(WIDTH) as usize, (p_y + v_y * n).rem_euclid(HEIGHT) as usize)
        //     }).collect::<HashSet<(usize, usize)>>();
        
        // if robots.len() != r.len() {
        //     continue 'outer;
        // }
        
        for y in 0..HEIGHT as usize {
            for x in 0..WIDTH as usize {
                if robots.contains(&(x, y)) {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!("{n}");
        let mut wait = String::new();
        stdin().read_line(&mut wait).unwrap();
        println!();
        println!();
    }
        
}

