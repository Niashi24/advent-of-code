use std::fmt::{Display, Formatter};
use std::io::BufRead;
use glam::IVec2;
use itertools::{Either, Itertools};
use memoize::memoize;
use pathfinding::prelude::bfs;
use smallvec::{smallvec, SmallVec};

pub fn part_1(input: Box<dyn BufRead>) -> anyhow::Result<impl Display> {
    let sequences = input.lines().map(Result::unwrap).collect_vec();
    
    
    let p_1 = {
        let mut p_1 = 0;

        for seq in &sequences {
            let path = bfs(&State::<Part1>::default(), |s| {
                let mut out = Vec::with_capacity(5);
                out.extend([IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y]
                    .into_iter()
                    .filter_map(|dir| s.clone().mv(dir)));
                out.extend(s.clone().confirm(seq));

                out
            }, |s| s.success(seq)).unwrap();

            p_1 += (path.len() - 1) * seq.strip_suffix("A").unwrap().parse::<usize>()?;
        }
        
        p_1
    };

    let p_2 = {
        let mut p_2 = 0;

        for seq in &sequences {
            let path = bfs(&State::<Part2>::default(), |s| {
                let mut out = Vec::with_capacity(5);
                out.extend([IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y]
                    .into_iter()
                    .filter_map(|dir| s.clone().mv(dir)));
                out.extend(s.clone().confirm(seq));

                out
            }, |s| s.success(seq)).unwrap();

            p_2 += dbg!((path.len() - 1)) * seq.strip_suffix("A").unwrap().parse::<usize>()?;
        }
        
        p_2
    };
    
    
    
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

type Transitions = SmallVec<[SmallVec<[Direction; 3]>; 2]>;

impl Direction {
    fn transition(a: Self, b: Self) -> Transitions {
        use Direction as D;
        match (a, b) {
            (a, b) if a == b => smallvec![],
            (D::Up, D::Left) => smallvec![smallvec![D::Down, D::Left]],
            (D::Up, D::Down) => smallvec![smallvec![D::Down]],
            (D::Up, D::Right) => smallvec![smallvec![D::Down, D::Right], smallvec![D::Right, D::Down]], // multiple
            (D::Up, D::A) => smallvec![smallvec![D::Right]],
            
            (D::Left, D::Down) => smallvec![smallvec![D::Right]],
            // (D::Left, D::Up) => smallvec![D::Right, D::Up],
            (D::Left, D::Right) => smallvec![smallvec![D::Right, D::Right]],
            (D::Left, D::A) => smallvec![smallvec![D::Right, D::Right, D::Up], smallvec![D::Right, D::Up, D::Right]],
            
            // (D::Down, D::Up) => smallvec![D::Up],
            // (D::Down, D::Left) => smallvec![D::Left],
            (D::Down, D::Right) => smallvec![smallvec![D::Right]],
            (D::Down, D::A) => smallvec![smallvec![D::Up, D::Right], smallvec![D::Right, D::Up]], // multiple
            
            // (D::Right, D::Up) => smallvec![D::Up, D::Left], // multiple
            // (D::Right, D::Left) => smallvec![D::Left, D::Left],
            // (D::Right, D::Down) => smallvec![D::Left],
            (D::Right, D::A) => smallvec![smallvec![D::Up]],
            // (D::A, D::Up) => smallvec![D::Left],
            
            // (D::A, D::Left) => smallvec![D::Down, D::Left, D::Left],
            // (D::A, D::Right) => smallvec![D::Down],
            // (D::A, D::Down) => smallvec![D::Down, D::Left], // multiple
            _ => Self::transition(b, a).into_iter().map(|x| x.into_iter().rev().map(Direction::reverse).collect()).collect(),
        }
    }
    
    fn reverse(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::A => Direction::A,
        }
    }
}

enum Numeric {
    Num(u8),
    A,
}

// impl Numeric {
//     fn transition(a: Self, b: Self) -> SmallVec<[Direction; 5]> {
//         
//     }
// }

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
    
    println!("{} -> {}: {:?}", a, b, out);
    
    out
}

impl TryFrom<char> for Direction {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            'A' => Ok(Direction::A),
            _ => Err(value),
        }
    }
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

#[test]
fn test_out() {
    let depth = 2;
    dbg!(solve("029A", depth));
    dbg!(solve("980A", depth));
    dbg!(solve("179A", depth));
    dbg!(solve("456A", depth));
    dbg!(solve("379A", depth));
    // dbg!(solve("869A", depth) * 869 +
    //     solve("170A", depth) * 170 +
    //     solve("319A", depth) * 319 +
    //     solve("349A", depth) * 349 +
    //     solve("489A", depth) * 489);
    
    // let base = "A^^^<Av>A^AvvvA".chars().map(|c| Direction::try_from(c).unwrap()).collect_vec();
    // 
    // 
    // let mut out = 0;
    // for (a, b) in base.iter().copied().tuple_windows() {
    //     out += recursive(a, b, 25);
    // }
    // dbg!(out);
}

#[memoize]
fn recursive(a: Direction, b: Direction, depth: usize) -> u64 {
    if depth == 0 {
        return 1;
    }
    
    let mut out = 0;
    let mut current = Direction::A;
    let transitions = Direction::transition(a, b);
    // println!("{:?} -> {:?}:\n   {:?}", a, b, transitions);
    // let mut explorations = vec![(current, 0)];
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
        .min().unwrap_or(1);
    
    out
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

type Part1 = DirectionRobot<DirectionRobot<NumericRobot>>;
type Part2 = DirectionRobot<
    DirectionRobot<
        DirectionRobot<
            DirectionRobot<
                DirectionRobot<
                    DirectionRobot<
                        DirectionRobot<
                            DirectionRobot<
                                DirectionRobot<
                                    DirectionRobot<
                                        DirectionRobot<
                                            DirectionRobot<
                                                DirectionRobot<
                                                    DirectionRobot<
                                                        DirectionRobot<
                                                            DirectionRobot<
                                                                DirectionRobot<
                                                                    DirectionRobot<
                                                                        DirectionRobot<
                                                                            DirectionRobot<
                                                                                DirectionRobot<
                                                                                    DirectionRobot<
                                                                                        DirectionRobot<
                                                                                            DirectionRobot<
                                                                                                DirectionRobot<
                                                                                                    NumericRobot
                                                                                                >
                                                                                            >
                                                                                        >
                                                                                    >
                                                                                >
                                                                            >
                                                                        >
                                                                    >
                                                                >
                                                            >
                                                        >
                                                    >
                                                >
                                            >
                                        >
                                    >
                                >
                            >
                        >
                    >
                >
            >
        >
    >
>;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct State<R: Robot> {
    robot_state: R,
    spelled: String,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct State2 {
    robot_state: DirectionRobot<
        DirectionRobot<
            DirectionRobot<
                DirectionRobot<
                    DirectionRobot<
                        DirectionRobot<
                            DirectionRobot<
                                DirectionRobot<
                                    DirectionRobot<
                                        DirectionRobot<
                                            DirectionRobot<
                                                DirectionRobot<
                                                    DirectionRobot<
                                                        DirectionRobot<
                                                            DirectionRobot<
                                                                DirectionRobot<
                                                                    DirectionRobot<
                                                                        DirectionRobot<
                                                                            DirectionRobot<
                                                                                DirectionRobot<
                                                                                    DirectionRobot<
                                                                                        DirectionRobot<
                                                                                            DirectionRobot<
                                                                                                DirectionRobot<
                                                                                                    DirectionRobot<
                                                                                                        NumericRobot
                                                                                                    >
                                                                                                >
                                                                                            >
                                                                                        >
                                                                                    >
                                                                                >
                                                                            >
                                                                        >
                                                                    >
                                                                >
                                                            >
                                                        >
                                                    >
                                                >
                                            >
                                        >
                                    >
                                >
                            >
                        >
                    >
                >
            >
        >
    >,
    spelled: String,
}

impl<R: Robot> Display for State<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.spelled, self.robot_state)
    }
}

impl<R: Robot> State<R> {
    pub fn mv(mut self, dir: IVec2) -> Option<Self> {
        self.robot_state = self.robot_state.mv(dir)?;
        Some(self)
    }
    
    pub fn confirm(mut self, target: &str) -> Option<Self> {
        match self.robot_state.confirm() {
            Either::Left(x) => { 
                self.robot_state = x?;
                Some(self)
            },
            Either::Right((state, output)) => {
                self.robot_state = state;
                self.spelled.push(output);
                if target.starts_with(&self.spelled) {
                    Some(self)
                } else {
                    None
                }
            }
        }
    }
    
    pub fn success(&self, target: &str) -> bool {
        &self.spelled == target
    }
}

trait Robot: Display {
    fn mv(self, dir: IVec2) -> Option<Self> where Self: Sized;
    fn confirm(self) -> Either<Option<Self>, (Self, char)> where Self: Sized;
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct NumericRobot {
    pos: IVec2,
}

impl Default for NumericRobot {
    fn default() -> Self {
        Self {
            pos: IVec2::new(2, 3),
        }
    }
}

impl Display for NumericRobot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self.pos.to_array() {
            [0, 0] => '7',
            [1, 0] => '8',
            [2, 0] => '9',
            [0, 1] => '4',
            [1, 1] => '5',
            [2, 1] => '6',
            [0, 2] => '1',
            [1, 2] => '2',
            [2, 2] => '3',
            [1, 3] => '0',
            [2, 3] => 'A',
            x => panic!("{x:?}"),
        })
    }
}

impl Robot for NumericRobot {
    fn mv(mut self, dir: IVec2) -> Option<Self> {
        self.pos += dir;
        if self.pos.x < 0 || self.pos.y < 0
            || self.pos.x > 2 || self.pos.y > 3
            || (self.pos.x == 0 && self.pos.y == 3) {
            None
        } else {
            Some(self)
        }
    }

    fn confirm(mut self) -> Either<Option<Self>, (Self, char)> {
        let c = match self.pos.to_array() {
            [0, 0] => '7',
            [1, 0] => '8',
            [2, 0] => '9',
            [0, 1] => '4',
            [1, 1] => '5',
            [2, 1] => '6',
            [0, 2] => '1',
            [1, 2] => '2',
            [2, 2] => '3',
            [1, 3] => '0',
            [2, 3] => 'A',
            x => panic!("{x:?}"),
        };
        Either::Right((self, c))
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct DirectionRobot<R: Robot> {
    robot: R,
    pos: IVec2,
}

impl<R: Default + Robot> Default for DirectionRobot<R> {
    fn default() -> Self {
        Self {
            robot: R::default(),
            pos: IVec2::new(2, 0),
        }
    }
}

impl<R: Robot> Display for DirectionRobot<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let d = match self.pos.to_array() {
            [1, 0] => '^',
            [0, 1] => '<',
            [1, 1] => 'v',
            [2, 1] => '>',
            [2, 0] => 'A',
            x => panic!("{x:?}"),
        };
        write!(f, "{}: {}", d, self.robot)
    }
}

impl<R: Robot> Robot for DirectionRobot<R> {
    fn mv(mut self, dir: IVec2) -> Option<Self> {
        self.pos += dir;
        if self.pos.x < 0 || self.pos.y < 0
            || self.pos.x > 2 || self.pos.y > 1
            || (self.pos.x == 0 && self.pos.y == 0) {
            None
        } else {
            Some(self)
        }
    }

    fn confirm(mut self) -> Either<Option<Self>, (Self, char)> {
        if self.pos.x == 2 && self.pos.y == 0 {
            return match self.robot.confirm() {
                Either::Left(Some(l)) => {
                    self.robot = l;
                    Either::Left(Some(self))
                }
                Either::Left(None) => Either::Left(None),
                Either::Right((robot, c)) => {
                    self.robot = robot;
                    Either::Right((self, c))
                }
            };
        }
        
        let dir = match self.pos.to_array() {
            [1, 0] => IVec2::NEG_Y,
            [0, 1] => IVec2::NEG_X,
            [1, 1] => IVec2::Y,
            [2, 1] => IVec2::X,
            x => panic!("{x:?}"),
        };
        
        match self.robot.mv(dir) {
            None => Either::Left(None),
            Some(r) => {
                self.robot = r;
                Either::Left(Some(self))
            }
        }
    }
}


