use std::fmt::{Display, Formatter};
use std::io::BufRead;
use glam::IVec2;
use itertools::{Either, Itertools};
use pathfinding::prelude::bfs;

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

            p_1 += dbg!((path.len() - 1)) * seq.strip_suffix("A").unwrap().parse::<usize>()?;
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

// impl Display for RobotState {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let c = self.confirm_numeric();
//         let d = match self.direction.to_array() {
//             [1, 0] => '^',
//             [0, 1] => '<',
//             [1, 1] => 'v',
//             [2, 1] => '>',
//             [2, 0] => 'A',
//             x => panic!("{x:?}"),
//         };
//         
//         write!(f, "{d}-{c}")
//     }
// }

// impl Default for RobotState {
//     fn default() -> Self {
//         Self {
//             numeric: IVec2::new(2, 3),
//             direction: IVec2::new(2, 0),
//         }
//     }
// }
