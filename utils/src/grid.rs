use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Grid<T> {
    pub grid: Vec<Vec<T>>,
    pub w: usize,
    pub h: usize,
}

impl<T> Grid<T> {
    pub fn new(grid: Vec<Vec<T>>) -> Self {
        let h = grid.len();
        let w = grid.first().map(Vec::len).unwrap_or(0);
        Self { grid, w, h }
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.grid.get(y).and_then(|y| y.get(x))
    }

    #[inline]
    #[allow(dead_code)]
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.grid.get_mut(y).and_then(|y| y.get_mut(x))
    }

    #[allow(dead_code)]
    pub fn get_i(&self, x: i64, y: i64) -> Option<&T> {
        self.get(usize::try_from(x).ok()?, usize::try_from(y).ok()?)
    }
    
    #[inline]
    pub fn get_i32(&self, x: i32, y: i32) -> Option<&T> {
        self.get(usize::try_from(x).ok()?, usize::try_from(y).ok()?)
    }

    #[allow(dead_code)]
    pub fn get_i_mut(&mut self, x: i64, y: i64) -> Option<&mut T> {
        self.get_mut(usize::try_from(x).ok()?, usize::try_from(y).ok()?)
    }

    #[allow(dead_code)]
    pub fn get_cycle(&self, mut x: i64, mut y: i64) -> Option<&T> {
        x = x.rem_euclid(self.w as i64);
        y = y.rem_euclid(self.h as i64);
        self.get(x as usize, y as usize)
    }

    #[allow(dead_code)]
    pub fn positions<FN: Fn(&T) -> bool>(&self, predicate: FN) -> Vec<(usize, usize)> {
        let mut pos = vec![];
        for (y, row) in self.grid.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                if predicate(item) {
                    pos.push((x, y));
                }
            }
        }
        pos
    }

    #[allow(dead_code)]
    pub fn iter(&self) -> GridIter<T> {
        GridIter {
            grid: self,
            x: 0,
            y: 0,
        }
    }

    #[allow(dead_code)]
    pub fn map<X, FN: Fn(T) -> X>(self, func: FN) -> Grid<X> {
        self.grid
            .into_iter()
            .map(|x| x.into_iter().map(&func))
            .collect()
    }

    #[allow(dead_code)]
    pub fn try_from_iter<E, IT, TIT>(iter: TIT) -> Result<Self, (usize, usize, E)>
    where
        IT: IntoIterator<Item = Result<T, E>>,
        TIT: IntoIterator<Item = IT>,
    {
        iter.into_iter()
            .enumerate()
            .map(move |(y, inner)| {
                inner
                    .into_iter()
                    .enumerate()
                    .map(|(x, item)| item.map_err(|e| (x, y, e)))
                    .collect::<Result<_, _>>()
            })
            .collect::<Result<_, _>>()
            .map(Self::new)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = ((usize, usize), &mut T)> {
        self.grid.iter_mut().enumerate().flat_map(|(y, row)| {
            row.iter_mut()
                .enumerate()
                .map(move |(x, elem)| ((x, y), elem))
        })
    }
}

#[derive(Copy, Clone)]
pub struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.grid.get(self.x, self.y).map(|x| ((self.x, self.y), x));

        self.x += 1;
        if self.x == self.grid.w {
            self.y += 1;
            self.x = 0;
        }

        item
    }
}

pub struct GridIntoIter<T> {
    grid: VecDeque<VecDeque<T>>,
    x: usize,
    y: usize,
}

impl<T> Iterator for GridIntoIter<T> {
    type Item = ((usize, usize), T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.grid.is_empty() {
            return None;
        }

        if self.y >= self.grid.len() {
            return None;
        }

        if self.grid[self.y].is_empty() {
            return None;
        }

        let item = self.grid[self.y]
            .pop_front()
            .map(|value| ((self.x, self.y), value));

        self.x += 1;
        if self.grid[self.y].is_empty() {
            self.x = 0;
            self.y += 1;
        }

        item
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = ((usize, usize), T);
    type IntoIter = GridIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            grid: self
                .grid
                .into_iter()
                .map(VecDeque::from)
                .collect(),
            x: 0,
            y: 0,
        }
    }
}

impl<T, IT> FromIterator<IT> for Grid<T>
where
    IT: IntoIterator<Item = T>,
{
    fn from_iter<TIT: IntoIterator<Item = IT>>(iter: TIT) -> Self {
        Self::new(iter.into_iter().map(|y| y.into_iter().collect()).collect())
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            for col in row.iter() {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[test]
fn test_iterator() {
    let mut grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6]]).into_iter();

    assert_eq!(grid.next(), Some(((0, 0), 1)));
    assert_eq!(grid.next(), Some(((1, 0), 2)));
    assert_eq!(grid.next(), Some(((2, 0), 3)));
    assert_eq!(grid.next(), Some(((0, 1), 4)));
    assert_eq!(grid.next(), Some(((1, 1), 5)));
    assert_eq!(grid.next(), Some(((2, 1), 6)));
    assert_eq!(grid.next(), None);
    assert_eq!(grid.next(), None);
}
