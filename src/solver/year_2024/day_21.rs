use std::fmt::{Display, Formatter};
use std::io::BufRead;
use glam::IVec2;
use itertools::{Either, Itertools};
use memoize::memoize;
use pathfinding::prelude::bfs;
use smallvec::{smallvec, SmallVec};

pub fn part_1(input: Box<dyn BufRead>) -> anyhow::Result<impl Display> {
    let sequences = input.lines().map(Result::unwrap).collect_vec();
    
    let p_1 = sequences.iter()
        .map(|s| s.strip_suffix("A").unwrap().parse::<u64>().unwrap() * solve(s, 2))
        .sum::<u64>();
    let p_2 = sequences.iter()
        .map(|s| s.strip_suffix("A").unwrap().parse::<u64>().unwrap() * solve(s, 25))
        .sum::<u64>();    
    
    Ok(p_1)
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[repr(i8)]
enum Direction {
    Up = -1,
    Left = -2,
    Down = 1,
    Right = 2,
    A = 0,
}

impl Direction {
    fn transition_path(a: Self, b: Self) -> SmallVec<[Direction; 3]> {
        use Direction as D;
        match (a, b) {
            (a, b) if a == b => smallvec![],
            (D::Up, D::Left) => smallvec![D::Down, D::Left],
            (D::Up, D::Down) => smallvec![D::Down],
            (D::Up, D::Right) => smallvec![D::Down, D::Right], // multiple
            (D::Up, D::A) => smallvec![D::Right],
            
            (D::Left, D::Down) => smallvec![D::Right],
            (D::Left, D::Right) => smallvec![D::Right, D::Right],
            (D::Left, D::A) => smallvec![D::Right, D::Right, D::Up],
            
            (D::Down, D::Right) => smallvec![D::Right],
            (D::Down, D::A) => smallvec![D::Up, D::Right], // multiple
            
            (D::Right, D::A) => smallvec![D::Up],
            
            _ => Self::transition_path(b, a).into_iter().rev().map(Direction::reverse).collect(),
        }
    }
    
    fn reverse(self) -> Self {
        unsafe { std::mem::transmute(-(self as i8)) }
    }
}

fn numeric_transition(a: IVec2, b: IVec2) -> SmallVec<[Direction; 5]> {
    let mut out = SmallVec::new();
    
    for _ in a.x..b.x {
        out.push(Direction::Right);
    }
    for _ in b.y..a.y {
        out.push(Direction::Up);
    }
    for _ in b.x..a.x {
        out.push(Direction::Left);
    }
    for _ in a.y..b.y {
        out.push(Direction::Down);
    }
    
    out
}

fn solve(seq: &str, depth: usize) -> u64 {
    let mut current = IVec2::new(2, 3);
    let mut out = 0;
    for c in seq.chars() {
        let x = IVec2::from(match c {
            '7' => [0, 0],
            '8' => [1, 0],
            '9' => [2, 0],
            '4' => [0, 1],
            '5' => [1, 1],
            '6' => [2, 1],
            '1' => [0, 2],
            '2' => [1, 2],
            '3' => [2, 2],
            '0' => [1, 3],
            'A' => [2, 3],
            x => panic!("{x:?}"),
        });
        
        let mut c = Direction::A;
        for x in numeric_transition(current, x) {
            out += recursive(c, x, depth);
            c = x;
        }
        out += recursive(c, Direction::A, depth);
        
        current = x;
    }
    
    out
}

#[memoize]
fn recursive(a: Direction, b: Direction, depth: usize) -> u64 {
    if depth == 0 {
        return 1;
    }
    
    let mut out = 0;
    let mut current = Direction::A;
    let transitions = Direction::transition_path(a, b);
    // println!("{:?} -> {:?}:\n   {:?}", a, b, transitions);
    for d in transitions {
        out += recursive(current, d, depth - 1);
        
        current = d;
    }
    
    out += recursive(current, Direction::A, depth - 1);
    // println!("   {out}");
    
    out
}

#[test]
fn test_out() {
    let depth = 2;
    dbg!(solve("029A", depth));
    dbg!(solve("980A", depth));
    dbg!(solve("179A", depth));
    dbg!(solve("456A", depth));
    dbg!(solve("379A", depth));
}

/*
vA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
Av|<v+A
v<|<+A
<<|+A
<<|+A
<A|>>^+A
A>|v+A
>>|+A
>^|<^+A
^A|>+A
A<|<v<+A
<A|>>^+A
A>|v+A
>AvA<^AA>A<vAAA>^A
 */
