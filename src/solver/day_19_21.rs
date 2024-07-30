use std::collections::{HashMap, HashSet};
use std::f32::consts::FRAC_1_SQRT_2;
use std::hash::Hash;
use std::io::BufRead;

use glam::f32::*;
use glam::IVec3;
use itertools::Itertools;

use crate::day::CombinedSolver;

pub struct Day1921;

impl CombinedSolver for Day1921 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let mut scanners = parse_scanners(input.lines().map(Result::unwrap));

        let mut found = scanners.swap_remove(0);
        let mut pairs = offset_pairs(found.iter().copied());
        let mut to_process = vec![];
        
        let mut positions = vec![IVec3::ZERO];

        while !scanners.is_empty() {
            while let Some(scanner) = scanners.pop() {
                if let Some((o, r)) = find_pos(&pairs, &scanner) {
                    positions.push(o);
                    for v in scanner.into_iter().map(|v| off_rot(v, o, r)) {
                        if !found.contains(&v) {
                            pairs.extend(found.iter().copied()
                                .map(|b| get_offset(v, b)));
                            found.insert(v);
                        }
                    }
                } else {
                    to_process.push(scanner);
                }
            }
            
            std::mem::swap(&mut to_process, &mut scanners);
        }

        let part_1 = found.len();
        let part_2 = positions.iter().copied()
            .tuple_combinations()
            .map(|(a, b)| (a - b).abs().to_array().into_iter().sum::<i32>())
            .max().unwrap();

        Ok((part_1.to_string(), part_2.to_string()))
    }
}

fn parse_scanners(lines: impl Iterator<Item=String>) -> Vec<HashSet<IVec3>> {
    let mut sets: Vec<HashSet<IVec3>> = Vec::new();
    let mut current_set = HashSet::new();
    let mut lines = lines.peekable();

    while let Some(line) = lines.next() {
        if line.starts_with("--- scanner") {
            if !current_set.is_empty() {
                sets.push(current_set);
                current_set = HashSet::new();
            }
        } else if !line.is_empty() {
            let arr = line.split(",")
                .map(|i| i.parse::<i32>().unwrap())
                .collect_vec()
                .try_into().unwrap();

            current_set.insert(IVec3::from_array(arr));
        }
    }

    if !current_set.is_empty() {
        sets.push(current_set);
    }

    sets
}

const ROTS: [Quat; 24] = [
    Quat::from_array([0.0, 0.0, 1.0, 0.0]),
    Quat::from_array([0.0, 1.0, 0.0, 0.0]),
    Quat::from_array([1.0, 0.0, 0.0, 0.0]),
    Quat::from_array([0.0, 0.0, 0.0, 1.0]),
    Quat::from_array([0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2, 0.0]),
    Quat::from_array([0.0, FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0]),
    Quat::from_array([FRAC_1_SQRT_2, 0.0, 0.0, -FRAC_1_SQRT_2]),
    Quat::from_array([FRAC_1_SQRT_2, 0.0, 0.0, FRAC_1_SQRT_2]),
    Quat::from_array([FRAC_1_SQRT_2, -FRAC_1_SQRT_2, 0.0, 0.0]),
    Quat::from_array([0.0, 0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2]),
    Quat::from_array([0.0, 0.0, FRAC_1_SQRT_2, FRAC_1_SQRT_2]),
    Quat::from_array([FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0, 0.0]),
    Quat::from_array([0.5, -0.5, 0.5, -0.5]),
    Quat::from_array([0.5, -0.5, -0.5, 0.5]),
    Quat::from_array([0.5, 0.5, -0.5, -0.5]),
    Quat::from_array([0.5, 0.5, 0.5, 0.5]),
    Quat::from_array([0.5, -0.5, -0.5, -0.5]),
    Quat::from_array([0.5, 0.5, -0.5, 0.5]),
    Quat::from_array([0.5, -0.5, 0.5, 0.5]),
    Quat::from_array([0.5, 0.5, 0.5, -0.5]),
    Quat::from_array([FRAC_1_SQRT_2, 0.0, -FRAC_1_SQRT_2, 0.0]),
    Quat::from_array([0.0, FRAC_1_SQRT_2, 0.0, FRAC_1_SQRT_2]),
    Quat::from_array([FRAC_1_SQRT_2, 0.0, FRAC_1_SQRT_2, 0.0]),
    Quat::from_array([0.0, FRAC_1_SQRT_2, 0.0, -FRAC_1_SQRT_2]),
];

fn rotate_offset(a: &HashSet<IVec3>, r: Quat, o: IVec3) -> HashSet<IVec3> {
    a.iter()
        .copied()
        .map(|v| off_rot(v, o, r))
        .collect()
}

fn find_pos(a_pairs: &HashMap<IVec3, IVec3>, b: &HashSet<IVec3>) -> Option<(IVec3, Quat)> {
    ROTS.into_iter()
        .flat_map(|r| {
            let b = rotate_offset(b, r, IVec3::ZERO);
            let b_pairs = offset_pairs(b.iter().copied());
            let x = key_intersection(&b_pairs, &a_pairs)
                .nth(12)
                .map(|(&a_1, &b_1)| {
                    (b_1 - a_1, r)
                }); x
        })
        .next()
}

fn key_intersection<'a, K: Eq + Hash, V>(a: &'a HashMap<K, V>, b: &'a HashMap<K, V>) -> impl Iterator<Item=(&'a V, &'a V)> {
    a.iter().flat_map(|(k, v)| b.get(k).map(|v_2| (v, v_2)))
}

fn off_rot(a: IVec3, o: IVec3, r: Quat) -> IVec3 {
    (r * a.as_vec3()).round().as_ivec3() + o
}

fn offset_pairs(it: impl Iterator<Item=IVec3> + Clone) -> HashMap<IVec3, IVec3> {
    it.tuple_combinations()
        .map(|(a, b)| get_offset(a, b))
        .collect()
}

fn get_offset(a: IVec3, b: IVec3) -> (IVec3, IVec3) {
    let x = a - b;
    if x.x < 0 {
        (-x, b)
    } else {
        (x, a)
    }
}