use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::io::BufRead;
use glam::IVec2;
use itertools::{Either, Itertools};
use memoize::memoize;
use pathfinding::prelude::bfs;
use smallvec::{smallvec, SmallVec};

pub fn solution(input: Box<dyn BufRead>) -> anyhow::Result<(impl Display, impl Display)> {
    let sequences = input.lines().map(Result::unwrap).collect_vec();
    
    let p_1 = sequences.iter()
        .map(|s| s.strip_suffix("A").unwrap().parse::<u64>().unwrap() * solve(s, 2))
        .sum::<u64>();
    let p_2 = sequences.iter()
        .map(|s| s.strip_suffix("A").unwrap().parse::<u64>().unwrap() * solve(s, 25))
        .sum::<u64>();    
    
    Ok((p_1, p_2))
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
    fn transition_path(a: Self, b: Self) -> SmallVec<[SmallVec<[Direction; 3]>; 3]> {
        use Direction as D;
        match (a, b) {
            (a, b) if a == b => smallvec![],
            (D::Up, D::Left) => smallvec![smallvec![D::Down, D::Left]],
            (D::Up, D::Down) => smallvec![smallvec![D::Down]],
            (D::Up, D::Right) => smallvec![smallvec![D::Down, D::Right]], // multiple
            (D::Up, D::A) => smallvec![smallvec![D::Right]],
            
            (D::Left, D::Down) => smallvec![smallvec![D::Right]],
            (D::Left, D::Right) => smallvec![smallvec![D::Right, D::Right]],
            (D::Left, D::A) => smallvec![smallvec![D::Right, D::Right, D::Up]],
            
            (D::Down, D::Right) => smallvec![smallvec![D::Right]],
            (D::Down, D::A) => smallvec![smallvec![D::Up, D::Right]], // multiple
            
            (D::Right, D::A) => smallvec![smallvec![D::Up]],
            
            _ => {
                Self::transition_path(b, a)
                    .into_iter()
                    .map(|s| s.into_iter()
                        .rev()
                        .map(Direction::reverse)
                        .collect())
                    .collect()
            },
        }
    }
    
    fn reverse(self) -> Self {
        unsafe { std::mem::transmute(-(self as i8)) }
    }
}

fn numeric_transition(a: IVec2, b: IVec2) -> SmallVec<[SmallVec<[Direction; 5]>; 2]> {
    if a == b {
        return smallvec![];
    }
    
    match (a.x.cmp(&b.x), a.y.cmp(&b.y)) {
        (Ordering::Equal, Ordering::Equal) => smallvec![smallvec![]],
        (Ordering::Equal, Ordering::Greater) => smallvec![smallvec![Direction::Up; (a.y - b.y) as usize]],
        (Ordering::Equal, Ordering::Less) => smallvec![smallvec![Direction::Down; (b.y - a.y) as usize]],
        (Ordering::Greater, Ordering::Equal) => smallvec![smallvec![Direction::Left; (a.x - b.x) as usize]],
        (Ordering::Less, Ordering::Equal) => smallvec![smallvec![Direction::Right; (b.x - a.x) as usize]],
        (Ordering::Less, Ordering::Less) => {
            let mut out = smallvec![];
            let mut left_down = smallvec![];
            left_down.extend(vec![Direction::Left; (a.y - b.y) as usize]);
            left_down.extend(vec![Direction::Down; (a.x - b.x) as usize]);
            let mut down_left = left_down.clone();
            down_left.reverse();
            out.push(down_left);
            out.push(left_down);

            out
        }
        (Ordering::Greater, Ordering::Greater) => {
            let mut out = smallvec![];
            let mut up_left = smallvec![];
            up_left.extend(vec![Direction::Up; (a.y - b.y) as usize]);
            up_left.extend(vec![Direction::Left; (a.x - b.x) as usize]);
            if a.y != 3 { // can move left first
                let mut left_up = up_left.clone();
                left_up.reverse();
                out.push(left_up);
            }
            out.push(up_left);
            
            out
        },
        _ => todo!(),
    }
    
    // let mut out = SmallVec::new();
    // 
    // for _ in a.x..b.x {
    //     out.push(Direction::Right);
    // }
    // for _ in b.y..a.y {
    //     out.push(Direction::Up);
    // }
    // for _ in b.x..a.x {
    //     out.push(Direction::Left);
    // }
    // for _ in a.y..b.y {
    //     out.push(Direction::Down);
    // }
    // 
    // out
}

// fn bfs_numeric(directions: &[Direction], a: IVec2, b: IVec2) -> SmallVec<[Direction; 5]> {
//     
// }

fn solve(seq: &str, depth: usize) -> u64 {
    // println!("{seq}");
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
        
        let before = out;
        
        let mut c = Direction::A;
        let transitions = numeric_transition(current, x);
        // println!("    {:?} -> {:?}: {:?}", current.to_array(), x.to_array(), transitions);
        let a = {
            let mut out = out;
            for &x in &transitions {
                out += recursive(c, x, depth);
                c = x;
            }
            out += recursive(c, Direction::A, depth);
            out
        };
        let b = {
            100000
            // let mut out = out;
            // for x in transitions.into_iter().rev() {
            //     out += recursive(c, x, depth);
            //     c = x;
            // }
            // out += recursive(c, Direction::A, depth);
            // out
        };
        
        out = a.min(b);
        
        // println!("  {}", out - before);
        
        current = x;
    }
    
    println!("{seq}: {out}");
    
    out
}

/*
<v<A>>^AvA^A|3
<vA<AA>>^AAvA<^A>AAvA^A|7
<vA>^AA<A>A|9
<v<A>A>^AAAvA<^A>A|A

<v<A|<
>>^A|A
vA|>
^A|A


<vA|v
<A|<
A|<
>>^A|A
A|A
vA|>
<^A|^
>A|A
A|A
vA|>
^A|A


<vA|v
>^A|A
A|A
<A|^
>A|A

<v<A|<
>A|v
>^A|A
A|A
A|A
vA|>
<^A|^
>A|A

<A>A v<<AA>^AA>A vAA^A <vAAA>^A

<A|^
>A|A

    
v<<A|<
A|<
>^A|^
A|^
>A|A

vA|>
A|>
^A|A

<vA|v
A|v
A|v
>^A|A

^A
<<^^A
>>A
vvvA

 */

#[memoize]
fn recursive(a: Direction, b: Direction, depth: usize) -> u64 {
    if depth == 0 {
        return 1;
    }
    
    // let mut out = 0;
    // let mut current = Direction::A;
    let transitions = Direction::transition_path(a, b);
    // println!("{:?} -> {:?}:\n   {:?}", a, b, transitions);
    let out = transitions.into_iter()
        .map(|transitions| {
            let mut out = 0;
            let mut current = Direction::A;
            for d in transitions {
                out += recursive(current, d, depth - 1);
                current = d;
            }
            out += recursive(current, Direction::A, depth - 1);
            out
        })
        .min().unwrap_or_default();
    
    out
}

#[test]
fn test_out() {
    let depth = 2;
    dbg!(solve("029A", depth));
    dbg!(solve("980A", depth));
    dbg!(solve("179A", depth));
    dbg!(solve("456A", depth));
    dbg!(solve("379A", depth)); //todo: this one gives 68 when it should give 64
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
