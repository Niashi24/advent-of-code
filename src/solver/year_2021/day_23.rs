use std::collections::BTreeMap;
use std::io::BufRead;
use itertools::Itertools;
use pathfinding::prelude::{astar, dijkstra, dijkstra_reach};
use smallvec::{SmallVec, smallvec};
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

        #[allow(unused_variables)]
        let state = State {
            map,
            max_depth: 1,
        };

        return Ok(("".to_string(), "".to_string()));
        #[allow(unreachable_code)]
        let (_, part_1) = astar(&state, State::successors, State::heuristic, State::success).unwrap();

        let (_, part_2) = astar(&state.part_2(), State::successors, State::heuristic, State::success).unwrap();


        Ok((part_1.to_string(), part_2.to_string()))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum PosNew {
    I(IP),
    O(OU),
}

impl PosNew {
    fn distance(self, other: Self) -> usize {
        use PosNew as P;
        use Amphipod as A;
        match (self, other) {
            (P::O(OU(1)), P::O(OU(2))) => 1,
            (P::O(OU(1)), P::O(OU(3))) => 3,
            (P::O(OU(1)), P::O(OU(4))) => 5,
            (P::O(OU(1)), P::O(OU(5))) => 7,
            (P::O(OU(1)), P::O(OU(6))) => 9,
            (P::O(OU(1)), P::O(OU(7))) => 10,
            (P::O(OU(1)), P::I(IP { depth, amp: A::A })) => 3 + depth as usize,
            (P::O(OU(1)), P::I(IP { depth, amp: A::B })) => 5 + depth as usize,
            (P::O(OU(1)), P::I(IP { depth, amp: A::C })) => 7 + depth as usize,
            (P::O(OU(1)), P::I(IP { depth, amp: A::D })) => 9 + depth as usize,

            (P::O(OU(2)), P::O(OU(3))) => 2,
            (P::O(OU(2)), P::O(OU(4))) => 4,
            (P::O(OU(2)), P::O(OU(5))) => 6,
            (P::O(OU(2)), P::O(OU(6))) => 8,
            (P::O(OU(2)), P::O(OU(7))) => 9,
            (P::O(OU(2)), P::I(IP { depth, amp: A::A })) => 2 + depth as usize,
            (P::O(OU(2)), P::I(IP { depth, amp: A::B })) => 4 + depth as usize,
            (P::O(OU(2)), P::I(IP { depth, amp: A::C })) => 6 + depth as usize,
            (P::O(OU(2)), P::I(IP { depth, amp: A::D })) => 8 + depth as usize,

            (P::O(OU(3)), P::O(OU(4))) => 2,
            (P::O(OU(3)), P::O(OU(5))) => 4,
            (P::O(OU(3)), P::O(OU(6))) => 6,
            (P::O(OU(3)), P::O(OU(7))) => 7,
            (P::O(OU(3)), P::I(IP { depth, amp: A::A })) => 2 + depth as usize,
            (P::O(OU(3)), P::I(IP { depth, amp: A::B })) => 2 + depth as usize,
            (P::O(OU(3)), P::I(IP { depth, amp: A::C })) => 4 + depth as usize,
            (P::O(OU(3)), P::I(IP { depth, amp: A::D })) => 6 + depth as usize,

            (P::O(OU(4)), P::O(OU(5))) => 2,
            (P::O(OU(4)), P::O(OU(6))) => 4,
            (P::O(OU(4)), P::O(OU(7))) => 5,
            (P::O(OU(4)), P::I(IP { depth, amp: A::A })) => 4 + depth as usize,
            (P::O(OU(4)), P::I(IP { depth, amp: A::B })) => 2 + depth as usize,
            (P::O(OU(4)), P::I(IP { depth, amp: A::C })) => 2 + depth as usize,
            (P::O(OU(4)), P::I(IP { depth, amp: A::D })) => 4 + depth as usize,

            (P::O(OU(5)), P::O(OU(6))) => 2,
            (P::O(OU(5)), P::O(OU(7))) => 3,
            (P::O(OU(5)), P::I(IP { depth, amp: A::A })) => 6 + depth as usize,
            (P::O(OU(5)), P::I(IP { depth, amp: A::B })) => 4 + depth as usize,
            (P::O(OU(5)), P::I(IP { depth, amp: A::C })) => 2 + depth as usize,
            (P::O(OU(5)), P::I(IP { depth, amp: A::D })) => 2 + depth as usize,

            (P::O(OU(6)), P::O(OU(7))) => 1,
            (P::O(OU(6)), P::I(IP { depth, amp: A::A })) => 8 + depth as usize,
            (P::O(OU(6)), P::I(IP { depth, amp: A::B })) => 6 + depth as usize,
            (P::O(OU(6)), P::I(IP { depth, amp: A::C })) => 4 + depth as usize,
            (P::O(OU(6)), P::I(IP { depth, amp: A::D })) => 2 + depth as usize,

            (P::O(OU(7)), P::I(IP { depth, amp: A::A })) => 9 + depth as usize,
            (P::O(OU(7)), P::I(IP { depth, amp: A::B })) => 7 + depth as usize,
            (P::O(OU(7)), P::I(IP { depth, amp: A::C })) => 5 + depth as usize,
            (P::O(OU(7)), P::I(IP { depth, amp: A::D })) => 3 + depth as usize,

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
                    OU(1) => smallvec![(P::O(OU(2)), 1)],
                    OU(2) => smallvec![(P::O(OU(1)), 1), (P::O(OU(3)), 2), (P::I(IP::new(0, A::A)), 2)],
                    OU(3) => smallvec![(P::O(OU(2)), 2), (P::O(OU(4)), 2), (P::I(IP::new(0, A::A)), 2), (P::I(IP::new(0, A::B)), 2)],
                    OU(4) => smallvec![(P::O(OU(3)), 2), (P::O(OU(5)), 2), (P::I(IP::new(0, A::B)), 2), (P::I(IP::new(0, A::C)), 2)],
                    OU(5) => smallvec![(P::O(OU(4)), 2), (P::O(OU(6)), 2), (P::I(IP::new(0, A::C)), 2), (P::I(IP::new(0, A::D)), 2)],
                    OU(6) => smallvec![(P::O(OU(7)), 1), (P::O(OU(5)), 2), (P::I(IP::new(0, A::D)), 2)],
                    OU(7) => smallvec![(P::O(OU(6)), 1)],

                    _ => panic!("{}", i.0),
                }
            }
            P::I(IP { depth: 0, amp }) => {
                match amp {
                    Amphipod::A => smallvec![(P::I(IP::new(1, amp)), 1), (P::O(OU(2)), 2), (P::O(OU(3)), 2)],
                    Amphipod::B => smallvec![(P::I(IP::new(1, amp)), 1), (P::O(OU(3)), 2), (P::O(OU(4)), 2)],
                    Amphipod::C => smallvec![(P::I(IP::new(1, amp)), 1), (P::O(OU(4)), 2), (P::O(OU(5)), 2)],
                    Amphipod::D => smallvec![(P::I(IP::new(1, amp)), 1), (P::O(OU(5)), 2), (P::O(OU(6)), 2)],
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

    // fn path(self, other: Self) -> SmallVec<[Self; 8]> {
    //     use PosNew as P;
    //     match (self, other) {
    //         (a, b) if a == b => smallvec![],
    //         (P::O(OU(a)), P::O(OU(b))) => {
    //             (a.min(b)..a.max(b))
    //                 .map(|i| P::O(OU(i)))
    //                 .collect()
    //         },
    //         (P::O(OU(a)), P::I(b)) | (P::I(b), P::O(OU(a))) => {
    //             todo!()
    //         }
    //         _ => other.path(self)
    //     }
    // 
    // }
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
struct OU(u8);

// impl OU {
//     pub fn path(self, other: Self) -> SmallVec<[Self; 7]> {
//         match (self, other) {
//             (a, b) if a == b => smallvec![],
//
//             (OU(1), OU(2)) => smallvec![OU(2)],
//             (OU(1), OU(3)) => smallvec![OU(2), OU(3)],
//             (OU(1), OU(4)) => smallvec![OU(2)],
//             (OU(1), OU(5)) => smallvec![OU(2)],
//             (OU(1), OU(6)) => smallvec![OU(2)],
//             (OU(1), OU(7)) => smallvec![OU(2)],
//             _ => other.path(self)
//         }
//     }
// }

// #############
// #12.3.4.5.67#
// ###2#2#2#2###
//   #1#1#1#1#
//   #########



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

    // fn to_char(self) -> char {
    //     match self {
    //         Amphipod::A => 'A',
    //         Amphipod::B => 'B',
    //         Amphipod::C => 'C',
    //         Amphipod::D => 'D',
    //     }
    // }
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
                                    PosNew::I(i) => (i.amp == ip.amp && i.depth > ip.depth) || i.amp == amp,
                                    // can always move anywhere outside
                                    PosNew::O(_) => true,
                                }
                            })
                            .collect_vec()
                    };
                    
                    

                    for p in dijkstra_reach(&pos, successor_inside) {
                        // no point moving to another place inside
                        if let PosNew::O(_) = p.node {
                            let mut next_state = next_state.clone();

                            next_state.map.insert(p.node, amp);
                            out.push((next_state, p.total_cost * amp.energy()))
                        }
                    }
                }
                PosNew::O(_) => {
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



