use std::collections::{BTreeMap, HashMap};
use std::fmt::{Display, Formatter};
use std::io::BufRead;
use bimap::BiBTreeMap;
use itertools::Itertools;
use pathfinding::prelude::{dijkstra, dijkstra_reach};
use smallvec::{SmallVec, smallvec};
use utils::extensions::IsNoneOr;
use crate::day::CombinedSolver;

pub struct Day23;

impl CombinedSolver for Day23 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let mut lines = input.lines().map(Result::unwrap);
        let mut map = BTreeMap::new();
        use Position as P;

        let line = lines.nth(2).unwrap();
        let mut chars = line.chars().filter(|c| *c != '#');
        map.insert(P::A2, chars.next().unwrap().try_into().unwrap());
        map.insert(P::B2, chars.next().unwrap().try_into().unwrap());
        map.insert(P::C2, chars.next().unwrap().try_into().unwrap());
        map.insert(P::D2, chars.next().unwrap().try_into().unwrap());
        let line = lines.next().unwrap();
        let mut chars = line.chars().filter(|c| *c != '#' && *c != ' ');
        map.insert(P::A1, chars.next().unwrap().try_into().unwrap());
        map.insert(P::B1, chars.next().unwrap().try_into().unwrap());
        map.insert(P::C1, chars.next().unwrap().try_into().unwrap());
        map.insert(P::D1, chars.next().unwrap().try_into().unwrap());

        let state = State {
            map
        };

        let (_, part_1) = dijkstra(&state, State::successors, State::success).unwrap();


        Ok((part_1.to_string(), "".to_string()))
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

    fn get_room(&self, inner: bool) -> Position {
        use Position as P;
        use Amphipod as A;
        match (*self, inner) {
            (A::A, true) => P::A1,
            (A::A, false) => P::A2,
            (A::B, true) => P::B1,
            (A::B, false) => P::B2,
            (A::C, true) => P::C1,
            (A::C, false) => P::C2,
            (A::D, true) => P::D1,
            (A::D, false) => P::D2,
        }
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
    map: BTreeMap<Position, Amphipod>,
    // last_moved: Option<Amphipod>,
}

impl State {
    fn successors(&self) -> Vec<(Self, usize)> {
        let mut out = Vec::new();

        for (&pos, &amp) in &self.map {
            let mut next_state = self.clone();
            next_state.map.remove(&pos).unwrap();

            if let Some((a_i, _)) = pos.is_inside() {
                let successor_inside = |&pos: &Position, _| {
                    pos.neighbors()
                        .into_iter()
                        // .map(|(p, c)| (p, c * amp.1.energy()))
                        .filter(|(p, _)| next_state.map.get(p).is_none())
                        .filter(|&(p, _)| p.is_inside().is_none_or(|(a, inner)| {
                            a_i == a ||
                                (a == amp && next_state.map.get(&a.get_room(!inner)).is_none_or(|a| {
                                    *a == amp
                                }))
                        }))
                        .collect_vec()
                };

                for p in dijkstra_reach(&pos, successor_inside).skip(1) {
                    let mut next_state = next_state.clone();

                    next_state.map.insert(p.node, amp);
                    out.push((next_state, p.total_cost * amp.energy()))
                }
            } else {
                let inner_room = amp.get_room(true);
                let outer_room = amp.get_room(false);
                let inner = next_state.map.get(&inner_room);
                let outer = next_state.map.get(&outer_room);
                
                let target = match (inner, outer) {
                    (None, None) => Some(amp.get_room(true)),
                    (_, Some(_)) => None,
                    (Some(x), None) if x.get_room(true) == inner_room => Some(outer_room),
                    (Some(_), None) => None,
                };
                
                if let Some(target) = target {
                    let successor_outside = |&pos: &Position| {
                        pos.neighbors()
                            .into_iter()
                            // .map(|(p, c)| (p, c * amp.1.energy()))
                            .filter(|(p, _)| next_state.map.get(p).is_none())
                            .filter(|&(p, _)| p.is_inside().is_none_or(|(a, inner)| {
                                a == amp
                            }))
                            .collect_vec()
                    };
                    
                    let success = |pos: &Position| *pos == target;
                    
                    if let Some((_, cost)) = dijkstra(&pos, successor_outside, success) {
                        let mut next_state = next_state.clone();

                        next_state.map.insert(target, amp);
                        out.push((next_state, cost * amp.energy()))
                    }

                    // for p in dijkstra(&pos, successor_outside, ).skip(1) {
                    //     let mut next_state = next_state.clone();
                    // 
                    //     next_state.map.insert(p.node, amp);
                    //     out.push((next_state, p.total_cost * amp.energy()))
                    // }
                }
            }
        }

        // println!("=====================");
        // println!("{self}");
        // 
        // for (p, c) in &out {
        //     println!("{c}");
        //     println!("{p}");
        // }
        // dbg!(out.len());

        out
    }

    fn success(&self) -> bool {
        self.map.get(&Position::A1) == Some(&Amphipod::A) &&
            self.map.get(&Position::A2) == Some(&Amphipod::A) &&
            self.map.get(&Position::B1) == Some(&Amphipod::B) &&
            self.map.get(&Position::B2) == Some(&Amphipod::B) &&
            self.map.get(&Position::C1) == Some(&Amphipod::C) &&
            self.map.get(&Position::C2) == Some(&Amphipod::C) &&
            self.map.get(&Position::D1) == Some(&Amphipod::D) &&
            self.map.get(&Position::D2) == Some(&Amphipod::D)
    }
}

#[test]
fn test_successors() {
    use Position as P;
    use Amphipod as A;
    let state = State {
        map: BTreeMap::from([
            (P::B1, A::A),
            (P::B2, A::B),
        ]),
    };

    dbg!(state.successors());
}

#[test]
fn should_go_to_goal() {
    use Position as P;
    use Amphipod as A;
    let state = State {
        map: BTreeMap::from([
            (P::O7, A::A)
        ])
    };

    let expected = State {
        map: BTreeMap::from([
            (P::A1, A::A)
        ])
    };

    assert_eq!(vec![(expected, 10)], state.successors());
}

impl Display for Amphipod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let get = |pos| {
            self.map.get(&pos)
                .map(|a| a.to_char())
                .unwrap_or('.')
        };
        use Position as P;
        writeln!(f, "#############")?;

        write!(f, "#")?;
        write!(f, "{}", get(P::O1))?;
        write!(f, "{}", get(P::O2))?;
        write!(f, ".")?;
        write!(f, "{}", get(P::O3))?;
        write!(f, ".")?;
        write!(f, "{}", get(P::O4))?;
        write!(f, ".")?;
        write!(f, "{}", get(P::O5))?;
        write!(f, ".")?;
        write!(f, "{}", get(P::O6))?;
        write!(f, "{}", get(P::O7))?;
        writeln!(f, "#")?;

        write!(f, "###")?;
        write!(f, "{}", get(P::A2))?;
        write!(f, "#")?;
        write!(f, "{}", get(P::B2))?;
        write!(f, "#")?;
        write!(f, "{}", get(P::C2))?;
        write!(f, "#")?;
        write!(f, "{}", get(P::D2))?;
        writeln!(f, "###")?;

        write!(f, "  #")?;
        write!(f, "{}", get(P::A1))?;
        write!(f, "#")?;
        write!(f, "{}", get(P::B1))?;
        write!(f, "#")?;
        write!(f, "{}", get(P::C1))?;
        write!(f, "#")?;
        write!(f, "{}", get(P::D1))?;
        writeln!(f, "#  ")?;

        write!(f, "  #########")?;

        Ok(())
    }
}



