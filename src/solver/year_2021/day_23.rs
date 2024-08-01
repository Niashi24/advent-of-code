use bimap::{BiBTreeMap, BiMap};
use smallvec::{SmallVec, smallvec};
use utils::extensions::IsNoneOr;

pub struct Day23;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Position {
    O1,O2,O3,O4,O5,O6,O7,
    A1,A2,
    B1,B2,
    C1,C2,
    D1,D2
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

    fn is_inside(&self) -> Option<(AType, bool)> {
        use Position as P;
        use AType as A;
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
enum AType {
    A, B, C, D
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Amphipod(bool, AType);

impl AType {
    fn energy(&self) -> usize {
        match self {
            AType::A => 1,
            AType::B => 10,
            AType::C => 100,
            AType::D => 1000,
        }
    }

    fn is_room(&self, p: Position) -> bool {
        use Position as P;
        use AType as A;
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
        use AType as A;
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


}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    map: BiBTreeMap<Position, Amphipod>,
    last_moved: Option<Amphipod>,
}

impl State {
    fn successors(&self) -> Vec<(Self, usize)> {
        let mut out = Vec::new();

        if let Some(cur) = self.last_moved {
            // stop moving
            out.push((Self {
                map: self.map.clone(),
                last_moved: None,
            }, 0));

            let mut map = self.map.clone();
            let (pos, _) = map.remove_by_right(&cur).unwrap();

            for (pos, cost) in pos.neighbors() {
                // position is already taken
                if map.contains_left(&pos) {
                    continue;
                }
                let cost = cur.1.energy() * cost;

                if let Some((a, inner)) = pos.is_inside() {
                    // not our room
                    if cur.1 != a {
                        continue;
                    }

                    // we can move there only if:
                    //    moving further in
                    //    OR the inner room is not occupied
                    //    OR the inner room is occupied by our roommate
                    if inner || map.get_by_left(&a.get_room(!inner)).is_none_or(|x| x.1 == a) {
                        let mut map = map.clone();
                        map.insert(pos, cur);

                        out.push((Self {
                            map,
                            last_moved: Some(cur),
                        }, cost));
                    }

                } else {
                    let mut map = map.clone();
                    map.insert(pos, cur);

                    out.push((Self {
                        map,
                        last_moved: Some(cur),
                    }, cost));
                }
            }
        } else {
            for (&p, &a) in &self.map {
                if p.is_inside().is_some() {

                }
            }
        }

        out
    }
}



