use std::fmt::Display;
use std::io::BufRead;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

pub fn part_1(input: Box<dyn BufRead>) -> anyhow::Result<impl Display> {
    let secrets = input.lines()
        .map(|s| s.unwrap().parse::<u64>().unwrap())
        .collect_vec();
    
    let p_1 = secrets.iter().copied()
        .map(|s| (0..2000).fold(s, |s, _| evolve(s)))
        .sum::<u64>();
    
    Ok(p_1)
}

pub fn part_2(input: Box<dyn BufRead>) -> anyhow::Result<impl Display> {
    let secrets = input.lines()
        .map(|s| s.unwrap().parse::<u64>().unwrap())
        .collect_vec();

    let costs_differences = secrets.iter().copied()
        .map(|s| gen(s))
        .collect_vec();

    let mut p_2 = 0;
    let mut cache = HashSet::<[i64; 4]>::new();
    for seq in costs_differences.iter()
        .flat_map(|(_, d)| d.windows(4)) {
        let seq = seq.try_into().unwrap();
        if !cache.insert(seq) {
            continue;
        }

        let mut score = 0;
        for (costs, differences) in &costs_differences {
            score += sell(seq, differences, costs);
        }

        p_2 = p_2.max(score);
        // dbg!(p_2);
        // println!("{:?}", seq);
    }
    
    Ok(p_2)
}

fn gen(mut secret: u64) -> (Vec<u64>, Vec<i64>) {
    let mut differences = Vec::with_capacity(2000);
    let mut costs = Vec::with_capacity(2001);
    costs.push(secret % 10);
    // println!("{}", secret);
    
    for i in 0..2000 {
        secret = evolve(secret);
        // println!("{}", secret);
        let cost = secret % 10;
        
        differences.push(cost as i64 - costs[i] as i64);
        costs.push(cost);
    }
    
    // println!("{}: {:?}", secret, costs.iter().map(|s| s.to_string()).join(""));

    (costs, differences)
}

fn sell(sequence: [i64; 4], differences: &[i64], costs: &[u64]) -> u64 {
    for i in 0..(differences.len() - sequence.len() + 1) {
        if &differences[i..i+4] == sequence {
            return costs[i + 4];
        }
    }
    0        
}

#[inline]
fn evolve(secret: u64) -> u64 {
    p_3(p_2(p_1(secret)))
}

#[inline]
fn p_1(secret: u64) -> u64 {
    prune(mix(secret, secret * 64))
}

#[inline]
fn p_2(secret: u64) -> u64 {
    prune(mix(secret, secret / 32))
}

#[inline]
fn p_3(secret: u64) -> u64 {
    prune(mix(secret, secret * 2048))
}

#[inline]
fn mix(secret: u64, other: u64) -> u64 {
    secret ^ other
}

#[inline]
fn prune(secret: u64) -> u64 {
    secret % 16777216
}