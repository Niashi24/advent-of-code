use std::collections::{HashMap, HashSet};
use std::f32::consts::FRAC_1_SQRT_2;
use std::hash::{Hash, Hasher};
use std::io::BufRead;
use glam::f32::*;
use glam::IVec3;
use itertools::Itertools;
use num::Signed;
use crate::day::SeparatedSolver;

pub struct Day1921;

type Set<T> = HashSet<T>;

impl SeparatedSolver for Day1921 {
    fn part_1(&self, input: Box<dyn BufRead>) -> anyhow::Result<String> {
        let mut scanners = parse_scanners(input.lines().map(Result::unwrap));

        // for ((i, a), (j, b)) in scanners.iter().enumerate().tuple_combinations() {
        //     println!("{i} -> {j}: {:?}", find_pos(a, b));
        //     println!("{j} -> {i}: {:?}", find_pos(b, a));
        // }

        let ref_scanner = scanners.remove(0);
        // let positions = scanners.iter()
        //     .filter_map(|b| find_pos(&ref_scanner, b).map(|p| (p, b)))
        //     .collect_vec();

        let mut found = ref_scanner;
        let mut to_process = vec![];

        while !scanners.is_empty() {
            while let Some(scanner) = scanners.pop() {
                if let Some((o, r)) = find_pos(&found, &scanner) {
                    // println!("{o:?} {r:?}");
                    for v in off_rot_iter(scanner.into_iter(), o, r) {
                        // println!("    {}", v);
                        found.insert(v);
                    }
                    // let scanner = rotate_offset(&scanner, r, o);


                    // found.push(scanner);
                } else {
                    to_process.push(scanner);
                }
            }

            // println!("To process: {}", to_process.len());
            if to_process.len() == 1 {
                // println!("{:#?}", to_process.first().unwrap());
            }

            // print_all(&found);
            std::mem::swap(&mut to_process, &mut scanners);
            // current_ref_i += 1;
        }

        let part_1 = found.len();

        Ok(part_1.to_string())
    }

    fn part_2(&self, input: Box<dyn BufRead>) -> anyhow::Result<String> {
        Ok("".to_string())
    }
}

fn print_all(set: &Set<IVec3>) {
    let mut vec = set.iter().copied().collect_vec();
    vec.sort_by(|a, b| {
        a.x.cmp(&b.x)
            .then_with(|| a.y.cmp(&b.y))
            .then_with(|| a.z.cmp(&b.z))
    });

    for v in vec {
        println!("{v}");
    }
}

fn parse_scanners(lines: impl Iterator<Item=String>) -> Vec<Set<IVec3>> {
    let mut sets: Vec<Set<IVec3>> = Vec::new();
    let mut current_set = Set::new();
    let mut lines = lines.peekable();

    while let Some(line) = lines.next() {
        if line.starts_with("--- scanner") {
            if !current_set.is_empty() {
                sets.push(current_set);
                current_set = Set::new();
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

fn rotate_offset(a: &Set<IVec3>, r: Quat, o: IVec3) -> Set<IVec3> {
    a.iter()
        .copied()
        .map(|v| v.as_vec3())
        .map(|v| r * v)
        .map(|v| v.round().as_ivec3())
        .map(|v| v + o)
        .collect()
}

fn off_rot_iter(it: impl Iterator<Item=IVec3>, o: IVec3, r: Quat) -> impl Iterator<Item=IVec3> {
    it
        .map(|v| v.as_vec3())
        .map(move |v| r * v)
        .map(|v| v.round().as_ivec3())
        .map(move |v| v + o)
}

fn offset_pairs(it: impl Iterator<Item=IVec3> + Clone) -> HashMap<IVec3, IVec3> {
    it.tuple_combinations()
        .map(|(a, b)| (a - b, (a, b)))
        .map(|(o, (a, b))| if o.x < 0 {
            (-o, b)
        } else {
            (o, a)
        })
        .collect()
}

fn find_pos(a: &Set<IVec3>, b: &Set<IVec3>) -> Option<(IVec3, Quat)> {
    let a_pairs = offset_pairs(a.iter().copied());
    // a_pairs.clone().for_each(|a| println!("{}",a.0));
    // println!("-------");

    let mut counts = counts_by_key(ROTS.into_iter()
        .flat_map(|r| {
            let b = rotate_offset(b, r, IVec3::ZERO);
            let b_pairs = offset_pairs(b.iter().copied());
            // if r == Quat::from_array([0.0, 0.0, 0.0, 1.0]) {
            //     println!("{r}: {:?}", b_pairs.iter().next());
            // }

            // println!("----");
            // b_pairs.clone().for_each(|a| println!("{}",a.0));
            // dbg!(key_intersection(&a_pairs, &b_pairs).count());
            // dbg!(a_pairs.len());
            // dbg!(b_pairs.len());

            // if r == Quat::from_array([0.0, 0.0, 0.0, 1.0]) && key_intersection(&a_pairs, &b_pairs).count() == 0 {
            //
            //     println!("----------");
            //     println!("a:");
            //     print_all(&a_pairs.keys().copied().collect());
            //     // for x in a_pairs.keys() {
            //     //     println!("    {x}");
            //     // }
            //     println!("b:");
            //     print_all(&b_pairs.keys().copied().collect());
            //     // for x in b_pairs.keys() {
            //     //     println!("    {x}");
            //     // }
            // }

            // a_pairs.intersection(&b_pairs)
            // let x = most_frequent(key_intersection(&a_pairs, &b_pairs)
            //     .map(|(&a_1, &b_1)| {
            //         (a_1 - b_1, r)
            //     }));
            // x
            key_intersection(&b_pairs, &a_pairs)
                .map(|(&a_1, &b_1)| {
                    (b_1 - a_1, r)
                })
                .collect_vec()
        }));


    // .collect_vec();
    counts.retain(|x| x.1 >= 12);
    // println!("{:?}", counts);
    // assert!(counts.len() <= 1);

    counts.into_iter().next().map(|x| x.0)
    // let x = counts.into_iter().max_by_key(|&(_, count)| count).map(|(item, _)| item);


    // x

    //
    // if x.len() > 1 {
    //     None
    // } else {
    //     x.pop()
    // }

    // x.into_iter()
    //     .next()
}

fn counts_by_key<A, B>(items: impl Iterator<Item=(A, B)>) -> Vec<((A, B), usize)>
where
    A: Hash + Eq,
{
    struct Key<A, B>(A, B);
    impl<A: Hash, B> Hash for Key<A, B> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }
    impl<A: Eq, B> PartialEq for Key<A, B> {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    impl<A: Eq, B> Eq for Key<A, B> {}

    items.map(|(a, b)| Key(a, b))
        .counts()
        .into_iter()
        .map(|(k, n)| ((k.0, k.1), n))
        .collect()
}

fn counts_vec<T>(items: impl Iterator<Item=T>) -> Vec<(T, usize)>
where
    T: PartialEq + Clone + std::fmt::Debug,
{
    let mut counts: Vec<(T, usize)> = Vec::new();

    for item in items {
        let mut found = false;

        for &mut (ref existing_item, ref mut count) in &mut counts {
            if *existing_item == item {
                *count += 1;
                found = true;
                break;
            }
        }

        if !found {
            counts.push((item, 1));
        }
    }

    counts
}

fn most_frequent<T>(items: impl Iterator<Item=T>) -> Option<T>
where
    T: PartialEq + Clone + std::fmt::Debug,
{
    let mut counts: Vec<(T, usize)> = Vec::new();

    for item in items {
        let mut found = false;

        for &mut (ref existing_item, ref mut count) in &mut counts {
            if *existing_item == item {
                *count += 1;
                found = true;
                break;
            }
        }

        if !found {
            counts.push((item, 1));
        }
    }

    if !counts.is_empty() {

        // dbg!(&counts);
    }

    counts.into_iter().max_by_key(|&(_, count)| count).map(|(item, _)| item)
}

fn key_intersection<'a, K: Eq + Hash, V>(a: &'a HashMap<K, V>, b: &'a HashMap<K, V>) -> impl Iterator<Item=(&'a V, &'a V)> {
    a.iter().flat_map(|(k, v)| b.get(k).map(|v_2| (v, v_2)))
}

fn overlap(a: &Set<IVec3>, b: &Set<IVec3>) -> bool {
    a.intersection(b).count() >= 12
}