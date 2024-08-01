use std::fmt::{Display, Formatter};
use std::ops::Range;

use itertools::Itertools;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct RangeD<const N: usize> {
    pub start: [i64; N],
    pub end: [i64; N]
}

impl<'a, const N: usize> IntoIterator for &'a RangeD<N> {
    type Item = [i64; N];
    type IntoIter = RangeDIterator<'a, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<const N: usize> Display for RangeD<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.start.iter().zip(self.end.iter()).map(|(s,e)| {
            format!("{s}..{e}")
        }).join(", "))
    }
}

impl<const N: usize> RangeD<N> {
    pub fn from_range_1d(ranges: [Range<i64>; N]) -> Self {
        Self {
            start: ranges.clone().map(|x| x.start),
            end: ranges.clone().map(|x| x.end),
        }
    }
    
    pub fn offset(&mut self, offset: i64) {
        self.start.iter_mut().for_each(|i| *i += offset);
        self.end.iter_mut().for_each(|i| *i += offset);
    }
    
    // pub fn offset_neg(&mut self, offset: usize) {
    //     self.start.iter_mut().for_each(|i| *i -= offset);
    //     self.end.iter_mut().for_each(|i| *i -= offset);
    // }
    
    pub fn offset_component(&mut self, i: usize, offset: i64) {
        self.start[i] += offset;
        self.end[i] += offset;
    }
    //
    // pub fn offset_component_neg(&mut self, i: usize, offset: usize) {
    //     self.start[i] -= offset;
    //     self.end[i] -= offset;
    // }
    
    pub fn volume(&self) -> i64 {
        self.start.iter().zip(self.end.iter()).map(|(s, e)| e - s).product()
    }
    
    pub fn intersect(&self, other: &Self) -> Option<Self> {
        // if !self.intersects(other) { return None; }
        
        let mut start = [0; N];
        let mut end = [0; N];
        for i in 0..N {
            start[i] = self.start[i].max(other.start[i]);
            end[i] = self.end[i].min(other.end[i]);
        }
        
        if start.iter().zip(end.iter()).all(|(s, e)| s < e) {
            Some(Self {
                start,
                end
            })
        } else {
            None
        }
    }

    pub fn difference(&self, other: &Self) -> Vec<Self> {
        let mut result = Vec::new();

        // Check if there is no intersection.
        if self.start.iter().zip(&other.end).any(|(s, e)| s >= e) || self.end.iter().zip(&other.start).any(|(e, s)| e <= s) {
            result.push(self.clone());
            return result;
        }

        let mut rest = self.clone();

        for i in 0..N {
            if other.start[i] > rest.start[i] {
                let mut fragment = rest.clone();
                fragment.end[i] = other.start[i];
                rest.start[i] = other.start[i];
                result.push(fragment);
            }

            if other.end[i] < rest.end[i] {
                let mut fragment = rest.clone();
                fragment.start[i] = other.end[i];
                rest.end[i] = other.end[i];
                result.push(fragment);
            }
        }

        result
    }
    
    pub fn intersects(&self, other: &Self) -> bool {
        fn contains_simple<const N: usize>(a: &RangeD<N>, b: &RangeD<N>) -> bool {
            (0..N).all(|i| {
                a.start[i] <= b.start[i] && b.start[i] < a.end[i]
            })
        }
        
        contains_simple::<N>(self, other) || contains_simple::<N>(other, self)
    }
    
    pub fn len_d(&self, i: usize) -> i64 {
        self.end[i] - self.start[i]
    }
    
    pub fn iter(&self) -> RangeDIterator<N> {
        RangeDIterator::new(self)
    }
}

#[test]
fn difference_1d() {
    // left side
    let r_1 = RangeD::from_range_1d([0..4]);
    let r_2 = RangeD::from_range_1d([-1..3]);
    let expected = RangeD::from_range_1d([3..4]);
    assert_eq!(vec![expected], r_1.difference(&r_2));
    // right side
    let r_1 = RangeD::from_range_1d([-1..3]);
    let r_2 = RangeD::from_range_1d([0..4]);
    let expected = RangeD::from_range_1d([-1..0]);
    assert_eq!(vec![expected], r_1.difference(&r_2));
    // inside
    let r_1 = RangeD::from_range_1d([0..4]);
    let r_2 = RangeD::from_range_1d([1..3]);
    let expected = vec![RangeD::from_range_1d([0..1]), RangeD::from_range_1d([3..4])];
    assert_eq!(expected, r_1.difference(&r_2));
    // outside
    let r_1 = RangeD::from_range_1d([1..3]);
    let r_2 = RangeD::from_range_1d([0..4]);
    let expected: Vec<RangeD<1>> = vec![];
    assert_eq!(expected, r_1.difference(&r_2));
}

pub struct RangeDIterator<'a, const N: usize> {
    ranges: &'a RangeD<N>,
    values: [i64; N]
}

impl<'a, const N: usize> RangeDIterator<'a, N> {
    pub fn new(range: &'a RangeD<N>) -> Self {
        Self {
            ranges: range,
            values: range.start
        }
    }
}

impl<'a, const N: usize> Iterator for RangeDIterator<'a, N> {
    type Item = [i64; N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.values[0] == self.ranges.end[0] {
            return None;
        }
        let item = self.values;
        
        let mut i = N - 1;
        self.values[i] += 1;
        while i != 0 && self.values[i] == self.ranges.end[i] {
            self.values[i] = self.ranges.start[i];
            self.values[i - 1] += 1;
            i -= 1;
        }

        Some(item)
    }
}

