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
    dbg!(p_2);
    
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
            (a, b) if a == b => smallvec![smallvec![]],
            (D::Up, D::Left) => smallvec![smallvec![D::Down, D::Left]],
            (D::Up, D::Down) => smallvec![smallvec![D::Down]],
            (D::Up, D::Right) => smallvec![smallvec![D::Down, D::Right], smallvec![D::Right, D::Down]], // multiple
            (D::Up, D::A) => smallvec![smallvec![D::Right]],
            
            (D::Left, D::Down) => smallvec![smallvec![D::Right]],
            (D::Left, D::Right) => smallvec![smallvec![D::Right, D::Right]],
            (D::Left, D::A) => smallvec![smallvec![D::Right, D::Right, D::Up], smallvec![D::Right, D::Up, D::Right]],
            
            (D::Down, D::Right) => smallvec![smallvec![D::Right]],
            (D::Down, D::A) => smallvec![smallvec![D::Up, D::Right], smallvec![D::Right, D::Up]], // multiple
            
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

fn all_transitions(inputs: impl Iterator<Item=IVec2>) -> Vec<Vec<Direction>> {
    let mut out = vec![vec![]];
    let mut current = IVec2::new(2, 3);
    for next in inputs {
        let transitions = numeric_transition(current, next);
        out = out.into_iter()
            .flat_map(|s| transitions.clone().into_iter()
                .map(move |x| {
                    let mut a = s.clone();
                    a.extend(x);
                    a.push(Direction::A);
                    a
                }))
            .collect();
        
        current = next;
    }
    out
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
        (Ordering::Less, Ordering::Less) => {  // going right and down
            let mut out = smallvec![];
            let mut right_down = smallvec![];
            right_down.extend(vec![Direction::Right; (a.x - b.x).abs() as usize]);
            right_down.extend(vec![Direction::Down; (a.y - b.y).abs() as usize]);
            if b.y != 3 {
                let mut down_right = right_down.clone();
                down_right.reverse();
                out.push(down_right);
            }
            out.push(right_down);

            out
        },
        (Ordering::Greater, Ordering::Greater) => {  // going left and up
            let mut out = smallvec![];
            let mut up_left = smallvec![];
            up_left.extend(vec![Direction::Up; (a.y - b.y).abs() as usize]);
            up_left.extend(vec![Direction::Left; (a.x - b.x).abs() as usize]);
            if a.y != 3 { // can move left first
                let mut left_up = up_left.clone();
                left_up.reverse();
                out.push(left_up);
            }
            out.push(up_left);
            
            out
        },
        (Ordering::Greater, Ordering::Less) => {  // going left and down
            let mut out = smallvec![];
            let mut left_down = smallvec![];
            left_down.extend(vec![Direction::Left; (a.x - b.x).abs() as usize]);
            left_down.extend(vec![Direction::Down; (a.y - b.y).abs() as usize]);
            if b.y != 3 { // can move down first
                let mut down_left = left_down.clone();
                down_left.reverse();
                out.push(down_left);
            }
            out.push(left_down);

            out
        },
        (Ordering::Less, Ordering::Greater) => {  // going right and up
            let mut out = smallvec![];
            let mut right_up = smallvec![];
            right_up.extend(vec![Direction::Up; (a.y - b.y).abs() as usize]);
            right_up.extend(vec![Direction::Right; (a.x - b.x).abs() as usize]);
            let mut up_right = right_up.clone();
            up_right.reverse();
            out.push(up_right);
            out.push(right_up);

            out
        },
        // _ => todo!(),
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
    let all_transitions = all_transitions(
        seq.chars().map(|c| IVec2::from(match c {
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
        }))
    );
    
    println!("{:?}: \n  {}", seq, all_transitions.clone().into_iter().map(|x| format!("{:?}", x)).join("\n  "));
    let out = all_transitions.into_iter()
        .map(|directions| {
            let mut c = Direction::A;
            let mut out = 0;
            for d in directions {
                out += recursive(c, d, depth);
                c = d;
            }
            out
        })
        .min().unwrap();
    
    // for c in seq.chars() {
    //     let x = IVec2::from(match c {
    //         '7' => [0, 0],
    //         '8' => [1, 0],
    //         '9' => [2, 0],
    //         '4' => [0, 1],
    //         '5' => [1, 1],
    //         '6' => [2, 1],
    //         '1' => [0, 2],
    //         '2' => [1, 2],
    //         '3' => [2, 2],
    //         '0' => [1, 3],
    //         'A' => [2, 3],
    //         x => panic!("{x:?}"),
    //     });
    //     
    //     let before = out;
    //     
    //     let mut c = Direction::A;
    //     let transitions = numeric_transition(current, x);
    //     // println!("    {:?} -> {:?}: {:?}", current.to_array(), x.to_array(), transitions);
    //     let a = {
    //         let mut out = out;
    //         for &x in &transitions {
    //             out += recursive(c, x, depth);
    //             c = x;
    //         }
    //         out += recursive(c, Direction::A, depth);
    //         out
    //     };
    //     let b = {
    //         100000
    //         // let mut out = out;
    //         // for x in transitions.into_iter().rev() {
    //         //     out += recursive(c, x, depth);
    //         //     c = x;
    //         // }
    //         // out += recursive(c, Direction::A, depth);
    //         // out
    //     };
    //     
    //     out = a.min(b);
    //     
    //     // println!("  {}", out - before);
    //     
    //     current = x;
    // }
    
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
        .min().unwrap();
    
    println!("{:?} -> {:?} @ {}: {}", a, b, depth, out);
    
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
