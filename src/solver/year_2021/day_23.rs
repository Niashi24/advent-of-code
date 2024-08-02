use std::collections::{BTreeMap, HashMap};
use std::fmt::{Display, Formatter};
use std::io::BufRead;
use bimap::BiBTreeMap;
use itertools::Itertools;
use pathfinding::prelude::{astar, dijkstra, dijkstra_reach};
use smallvec::{SmallVec, smallvec};
use utils::extensions::IsNoneOr;
use crate::day::CombinedSolver;

pub struct Day23;

impl CombinedSolver for Day23 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let mut lines = input.lines().map(Result::unwrap);
        let mut map = BTreeMap::new();
        use PosNew as P;
        use Amphipod as A;

        let line = lines.nth(2).unwrap();
        let mut chars = line.chars().filter(|c| *c != '#');
        map.insert(P::I(IP::new(0, A::A)), chars.next().unwrap().try_into().unwrap());
        map.insert(P::I(IP::new(0, A::B)), chars.next().unwrap().try_into().unwrap());
        map.insert(P::I(IP::new(0, A::C)), chars.next().unwrap().try_into().unwrap());
        map.insert(P::I(IP::new(0, A::D)), chars.next().unwrap().try_into().unwrap());
        let line = lines.next().unwrap();
        let mut chars = line.chars().filter(|c| *c != '#' && *c != ' ');
        map.insert(P::I(IP::new(1, A::A)), chars.next().unwrap().try_into().unwrap());
        map.insert(P::I(IP::new(1, A::B)), chars.next().unwrap().try_into().unwrap());
        map.insert(P::I(IP::new(1, A::C)), chars.next().unwrap().try_into().unwrap());
        map.insert(P::I(IP::new(1, A::D)), chars.next().unwrap().try_into().unwrap());

        let state = State {
            map,
            max_depth: 1,
        };

        let (_, part_1) = astar(&state, State::successors, State::heuristic, State::success).unwrap();

        let (_, part_2) = astar(&state.part_2(), State::successors, State::heuristic, State::success).unwrap();


        Ok((part_1.to_string(), part_2.to_string()))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum PosNew {
    I(IP),
    O(u8),
}

impl PosNew {
    fn distance(self, other: Self) -> usize {
        use PosNew as P;
        use Amphipod as A;
        match (self, other) {
            (P::O(1), P::O(2)) => 1,
            (P::O(1), P::O(3)) => 3,
            (P::O(1), P::O(4)) => 5,
            (P::O(1), P::O(5)) => 7,
            (P::O(1), P::O(6)) => 9,
            (P::O(1), P::O(7)) => 10,
            (P::O(1), P::I(IP { depth, amp: A::A })) => 3 + depth as usize,
            (P::O(1), P::I(IP { depth, amp: A::B })) => 5 + depth as usize,
            (P::O(1), P::I(IP { depth, amp: A::C })) => 7 + depth as usize,
            (P::O(1), P::I(IP { depth, amp: A::D })) => 9 + depth as usize,

            (P::O(2), P::O(3)) => 2,
            (P::O(2), P::O(4)) => 4,
            (P::O(2), P::O(5)) => 6,
            (P::O(2), P::O(6)) => 8,
            (P::O(2), P::O(7)) => 9,
            (P::O(2), P::I(IP { depth, amp: A::A })) => 2 + depth as usize,
            (P::O(2), P::I(IP { depth, amp: A::B })) => 4 + depth as usize,
            (P::O(2), P::I(IP { depth, amp: A::C })) => 6 + depth as usize,
            (P::O(2), P::I(IP { depth, amp: A::D })) => 8 + depth as usize,

            (P::O(3), P::O(4)) => 2,
            (P::O(3), P::O(5)) => 4,
            (P::O(3), P::O(6)) => 6,
            (P::O(3), P::O(7)) => 7,
            (P::O(3), P::I(IP { depth, amp: A::A })) => 2 + depth as usize,
            (P::O(3), P::I(IP { depth, amp: A::B })) => 2 + depth as usize,
            (P::O(3), P::I(IP { depth, amp: A::C })) => 4 + depth as usize,
            (P::O(3), P::I(IP { depth, amp: A::D })) => 6 + depth as usize,

            (P::O(4), P::O(5)) => 2,
            (P::O(4), P::O(6)) => 4,
            (P::O(4), P::O(7)) => 5,
            (P::O(4), P::I(IP { depth, amp: A::A })) => 4 + depth as usize,
            (P::O(4), P::I(IP { depth, amp: A::B })) => 2 + depth as usize,
            (P::O(4), P::I(IP { depth, amp: A::C })) => 2 + depth as usize,
            (P::O(4), P::I(IP { depth, amp: A::D })) => 4 + depth as usize,

            (P::O(5), P::O(6)) => 2,
            (P::O(5), P::O(7)) => 3,
            (P::O(5), P::I(IP { depth, amp: A::A })) => 6 + depth as usize,
            (P::O(5), P::I(IP { depth, amp: A::B })) => 4 + depth as usize,
            (P::O(5), P::I(IP { depth, amp: A::C })) => 2 + depth as usize,
            (P::O(5), P::I(IP { depth, amp: A::D })) => 2 + depth as usize,

            (P::O(6), P::O(7)) => 1,
            (P::O(6), P::I(IP { depth, amp: A::A })) => 8 + depth as usize,
            (P::O(6), P::I(IP { depth, amp: A::B })) => 6 + depth as usize,
            (P::O(6), P::I(IP { depth, amp: A::C })) => 4 + depth as usize,
            (P::O(6), P::I(IP { depth, amp: A::D })) => 2 + depth as usize,

            (P::O(7), P::I(IP { depth, amp: A::A })) => 9 + depth as usize,
            (P::O(7), P::I(IP { depth, amp: A::B })) => 7 + depth as usize,
            (P::O(7), P::I(IP { depth, amp: A::C })) => 5 + depth as usize,
            (P::O(7), P::I(IP { depth, amp: A::D })) => 3 + depth as usize,

            (P::I(IP { depth: d_1, amp: a_1 }), P::I(IP { depth: d_2, amp: a_2 })) if a_1 == a_2 => d_1.abs_diff(d_2) as usize,

            (P::I(IP { depth: d_1, amp: A::A }), P::I(IP { depth: d_2, amp: A::B })) => 4 + (d_1 + d_2) as usize,
            (P::I(IP { depth: d_1, amp: A::A }), P::I(IP { depth: d_2, amp: A::C })) => 6 + (d_1 + d_2) as usize,
            (P::I(IP { depth: d_1, amp: A::A }), P::I(IP { depth: d_2, amp: A::D })) => 8 + (d_1 + d_2) as usize,

            (P::I(IP { depth: d_1, amp: A::B }), P::I(IP { depth: d_2, amp: A::C })) => 4 + (d_1 + d_2) as usize,
            (P::I(IP { depth: d_1, amp: A::B }), P::I(IP { depth: d_2, amp: A::D })) => 6 + (d_1 + d_2) as usize,

            (P::I(IP { depth: d_1, amp: A::C }), P::I(IP { depth: d_2, amp: A::D })) => 4 + (d_1 + d_2) as usize,

            _ if self == other => 0,

            _ => other.distance(self)
        }
    }

    fn neighbors(&self, max_depth: u8) -> SmallVec<[(Self, usize); 4]> {
        use PosNew as P;
        use Amphipod as A;
        // #############
        // #12.3.4.5.67#
        // ###2#2#2#2###
        //   #1#1#1#1#
        //   #########
        match *self {
            P::O(i) => {
                match i {
                    1 => smallvec![(P::O(2), 1)],
                    2 => smallvec![(P::O(1), 1), (P::O(3), 2), (P::I(IP::new(0, A::A)), 2)],
                    3 => smallvec![(P::O(2), 2), (P::O(4), 2), (P::I(IP::new(0, A::A)), 2), (P::I(IP::new(0, A::B)), 2)],
                    4 => smallvec![(P::O(3), 2), (P::O(5), 2), (P::I(IP::new(0, A::B)), 2), (P::I(IP::new(0, A::C)), 2)],
                    5 => smallvec![(P::O(4), 2), (P::O(6), 2), (P::I(IP::new(0, A::C)), 2), (P::I(IP::new(0, A::D)), 2)],
                    6 => smallvec![(P::O(7), 1), (P::O(5), 2), (P::I(IP::new(0, A::D)), 2)],
                    7 => smallvec![(P::O(6), 1)],

                    _ => panic!("{i}"),
                }
            }
            P::I(IP { depth: 0, amp }) => {
                match amp {
                    Amphipod::A => smallvec![(P::I(IP::new(1, amp)), 1), (P::O(2), 2), (P::O(3), 2)],
                    Amphipod::B => smallvec![(P::I(IP::new(1, amp)), 1), (P::O(3), 2), (P::O(4), 2)],
                    Amphipod::C => smallvec![(P::I(IP::new(1, amp)), 1), (P::O(4), 2), (P::O(5), 2)],
                    Amphipod::D => smallvec![(P::I(IP::new(1, amp)), 1), (P::O(5), 2), (P::O(6), 2)],
                }
            }
            P::I(x) => {
                let mut out = SmallVec::new();

                out.push((P::I(x.depth(x.depth - 1)), 1));
                if x.depth != max_depth {
                    out.push((P::I(x.depth(x.depth + 1)), 1));
                }

                out
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct IP {
    pub depth: u8,
    pub amp: Amphipod,
}

impl IP {
    pub fn new(depth: u8, amp: Amphipod) -> Self {
        Self {
            depth,
            amp,
        }
    }

    pub fn depth(self, depth: u8) -> Self {
        Self {
            depth,
            ..self
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Position {
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    A1,
    A2,
    B1,
    B2,
    C1,
    C2,
    D1,
    D2,
}

// #############
// #12.3.4.5.67#
// ###2#2#2#2###
//   #1#1#1#1#
//   #########

impl Position {
    fn neighbors(self) -> SmallVec<[(Position, usize); 4]> {
        use Position as P;
        match self {
            P::O1 => smallvec![(P::O2, 1)],
            P::O2 => smallvec![(P::O1, 1), (P::O3, 2), (P::A2, 2)],
            P::O3 => smallvec![(P::O2, 2), (P::O4, 2), (P::A2, 2), (P::B2, 2)],
            P::O4 => smallvec![(P::O3, 2), (P::O5, 2), (P::B2, 2), (P::C2, 2)],
            P::O5 => smallvec![(P::O4, 2), (P::O6, 2), (P::C2, 2), (P::D2, 2)],
            P::O6 => smallvec![(P::O7, 1), (P::O5, 2), (P::D2, 2)],
            P::O7 => smallvec![(P::O6, 1)],

            P::A1 => smallvec![(P::A2, 1)],
            P::A2 => smallvec![(P::A1, 1), (P::O2, 2), (P::O3, 2)],
            P::B1 => smallvec![(P::B2, 1)],
            P::B2 => smallvec![(P::B1, 1), (P::O3, 2), (P::O4, 2)],
            P::C1 => smallvec![(P::C2, 1)],
            P::C2 => smallvec![(P::C1, 1), (P::O4, 2), (P::O5, 2)],
            P::D1 => smallvec![(P::D2, 1)],
            P::D2 => smallvec![(P::D1, 1), (P::O5, 2), (P::O6, 2)],
        }
    }

    // fn is_outside(&self) -> bool {
    //     use Position as P;
    //     match self {
    //         P::O1 | P::O2 | P::O3 | P::O4 | P::O5 | P::O6 | P::O7 => true,
    //         _ => false,
    //     }
    // }

    fn is_inside(&self) -> Option<(Amphipod, bool)> {
        use Position as P;
        use Amphipod as A;
        match self {
            P::A1 => Some((A::A, true)),
            P::A2 => Some((A::A, false)),
            P::B1 => Some((A::B, true)),
            P::B2 => Some((A::B, false)),
            P::C1 => Some((A::C, true)),
            P::C2 => Some((A::C, false)),
            P::D1 => Some((A::D, true)),
            P::D2 => Some((A::D, false)),
            _ => None,
        }
    }

    // fn other_room(&self) -> Option<Self> {
    //     use Position as P;
    //     match self {
    //         P::A1 => Some(P::A2),
    //         P::A2 => Some(P::A1),
    //         P::A1 => Some(P::A2),
    //         P::A2 => Some(P::A1),
    //         _ => None,
    //     }
    // }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    const ALL: [Self; 4] = [
        Amphipod::A,
        Amphipod::B,
        Amphipod::C,
        Amphipod::D,
    ];

    fn energy(&self) -> usize {
        match self {
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
        }
    }

    fn is_room(&self, p: Position) -> bool {
        use Position as P;
        use Amphipod as A;
        match (*self, p) {
            (A::A, P::A1) | (A::A, P::A2) => true,
            (A::B, P::B1) | (A::B, P::B2) => true,
            (A::C, P::C1) | (A::C, P::C2) => true,
            (A::D, P::D1) | (A::D, P::D2) => true,
            _ => false,
        }
    }

    fn get_room(&self, depth: u8) -> PosNew {
        use PosNew as P;
        use Amphipod as A;
        match self {
            A::A => P::I(IP::new(depth, *self)),
            A::B => P::I(IP::new(depth, *self)),
            A::C => P::I(IP::new(depth, *self)),
            A::D => P::I(IP::new(depth, *self)),
        }

        // match (*self, inner) {
        //     (A::A, true) => P::A1,
        //     (A::A, false) => P::A2,
        //     (A::B, true) => P::B1,
        //     (A::B, false) => P::B2,
        //     (A::C, true) => P::C1,
        //     (A::C, false) => P::C2,
        //     (A::D, true) => P::D1,
        //     (A::D, false) => P::D2,
        // }
    }

    fn to_char(self) -> char {
        match self {
            Amphipod::A => 'A',
            Amphipod::B => 'B',
            Amphipod::C => 'C',
            Amphipod::D => 'D',
        }
    }
}

impl TryFrom<char> for Amphipod {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Amphipod as A;
        match value {
            'A' => Ok(A::A),
            'B' => Ok(A::B),
            'C' => Ok(A::C),
            'D' => Ok(A::D),
            _ => Err(value),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    map: BTreeMap<PosNew, Amphipod>,
    max_depth: u8,
}

impl State {
    fn part_2(mut self) -> Self {
        use PosNew as P;
        use Amphipod as A;

        self.max_depth = 3;
        let a = self.map.remove(&P::I(IP::new(1, A::A))).unwrap();
        let b = self.map.remove(&P::I(IP::new(1, A::B))).unwrap();
        let c = self.map.remove(&P::I(IP::new(1, A::C))).unwrap();
        let d = self.map.remove(&P::I(IP::new(1, A::D))).unwrap();

        self.map.insert(P::I(IP::new(3, A::A)), a);
        self.map.insert(P::I(IP::new(3, A::B)), b);
        self.map.insert(P::I(IP::new(3, A::C)), c);
        self.map.insert(P::I(IP::new(3, A::D)), d);

        self.map.insert(P::I(IP::new(1, A::A)), A::D);
        self.map.insert(P::I(IP::new(1, A::B)), A::C);
        self.map.insert(P::I(IP::new(1, A::C)), A::B);
        self.map.insert(P::I(IP::new(1, A::D)), A::A);
        self.map.insert(P::I(IP::new(2, A::A)), A::D);
        self.map.insert(P::I(IP::new(2, A::B)), A::B);
        self.map.insert(P::I(IP::new(2, A::C)), A::A);
        self.map.insert(P::I(IP::new(2, A::D)), A::C);

        self
    }

    fn get_inside_target(&self, amphipod: Amphipod) -> Option<PosNew> {
        let mut positions = (0..=self.max_depth).rev()
            .map(|i| PosNew::I(IP::new(i, amphipod)))
            .map(|p| (p, self.map.get(&p)));

        // Check first until space: if any type != search, return None
        let empty;
        loop {
            let (pos, amp) = positions.next()?;
            match amp {
                None => {
                    empty = pos;
                    break;
                }
                Some(a) => {
                    if *a != amphipod {
                        return None;
                    }
                }
            }
        }
        // Once get first space, check remaining to see if any are occupied
        if positions.all(|(_, a)| a.is_none()) {
            Some(empty)
        } else {
            None
        }
    }



    fn successors(&self) -> Vec<(Self, usize)> {
        let mut out = Vec::new();
        for (&pos, &amp) in &self.map {
            let mut next_state = self.clone();
            next_state.map.remove(&pos).unwrap();

            match pos {
                PosNew::I(ip) => {
                    let successor_inside = |&pos: &PosNew, _| {
                        pos.neighbors(self.max_depth)
                            .into_iter()
                            // .map(|(p, c)| (p, c * amp.1.energy()))
                            .filter(|(p, _)| next_state.map.get(p).is_none())
                            .filter(|&(p, _)| {
                                match p {
                                    // can move inside if still inside initial
                                    // or color matches
                                    PosNew::I(i) => i.amp == ip.amp || i.amp == amp,
                                    // can always move anywhere outside
                                    PosNew::O(_) => true,
                                }
                            })
                            .collect_vec()
                    };

                    for p in dijkstra_reach(&pos, successor_inside) {
                        // no point moving to another place inside
                        if let PosNew::O(i) = p.node {
                            let mut next_state = next_state.clone();

                            next_state.map.insert(p.node, amp);
                            out.push((next_state, p.total_cost * amp.energy()))
                        }
                    }
                }
                PosNew::O(d) => {
                    let target = self.get_inside_target(amp);

                    if let Some(target) = target {
                        let successor_outside = |&pos: &PosNew| {
                            pos.neighbors(self.max_depth)
                                .into_iter()
                                // .map(|(p, c)| (p, c * amp.1.energy()))
                                .filter(|(p, _)| next_state.map.get(p).is_none())
                                .filter(|&(p, _)| {
                                    match p {
                                        PosNew::I(ip) => ip.amp == amp,
                                        PosNew::O(_) => true,
                                    }
                                })
                                .collect_vec()
                        };

                        let success = |pos: &PosNew| *pos == target;

                        if let Some((_, cost)) = dijkstra(&pos, successor_outside, success) {
                            let mut next_state = next_state.clone();

                            next_state.map.insert(target, amp);
                            out.push((next_state, cost * amp.energy()))
                        }
                    }
                }
            }
        }

        out
    }

    fn heuristic(&self) -> usize {
        self.map.iter()
            .map(|(&p, &a)| p.distance(a.get_room(0)) * a.energy())
            .sum()
    }

    fn success(&self) -> bool {
        use Amphipod as A;
        (0..=self.max_depth)
            .flat_map(|i| A::ALL.map(|a| (i, a)))
            .all(|(i, amp)| self.map.get(&PosNew::I(IP::new(i, amp)))
                .is_some_and(|a| *a == amp))
    }
}



