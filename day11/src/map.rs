use std::fmt;
use std::fmt::Formatter;
use std::ops::{Index, IndexMut};

const NEIGHBORHOOD_SIZE: i64 = 3;

pub trait Center {
    fn center(&self) -> Option<u64>;
}

pub struct Map {
    num_rows: usize,
    num_columns: usize,
    data: Vec<u64>,
}

impl Map {
    pub fn new(input: &str) -> Self {
        let lines = input.trim().split('\n').collect::<Vec<&str>>();
        let num_rows = lines.len();
        let num_columns = lines.first().unwrap().len();

        let data =
            lines.into_iter()
                .flat_map(|line| line.trim().chars().into_iter()
                    .map(|char| char.to_digit(10).unwrap() as u64)).collect::<Vec<_>>();

        assert_eq!(data.len(), num_rows * num_columns);

        Map {
            num_rows,
            num_columns,
            data,
        }
    }

    pub fn neighborhood_iter(&self) -> NeighborhoodIterator {
        NeighborhoodIterator {
            map: self,
            current_index: 0,
        }
    }

    pub fn neighborhood(&self, index: usize) -> Option<[Option<u64>; 9]> {
        let (current_row, current_column) = self.position_for_index(index)?;
        let current_row = current_row as i64;
        let current_column = current_column as i64;

        let mut window = [None; 9];

        for row in 0..NEIGHBORHOOD_SIZE {
            for column in 0..NEIGHBORHOOD_SIZE {
                let window_index = (NEIGHBORHOOD_SIZE * row + column) as usize;
                window[window_index] = match (current_row + row - 1, current_column + column - 1) {
                    (r, c) if r < 0 || c < 0 || r as usize >= self.num_rows || c as usize >= self.num_columns => None,
                    (r, c) => Some(self[(r as usize, c as usize)])
                };
            }
        }

        Some(window)
    }

    pub fn neighborhood_positions(&self, index: usize) -> Option<[Option<usize>; 9]> {
        let (current_row, current_column) = self.position_for_index(index)?;

        let current_row = current_row as i64;
        let current_column = current_column as i64;

        let mut indices = [None; 9];

        for row in 0..NEIGHBORHOOD_SIZE {
            for column in 0..NEIGHBORHOOD_SIZE {
                let neighborhoods_index = (NEIGHBORHOOD_SIZE * row + column) as usize;
                let map_row = current_row + row as i64 - 1;
                let map_column = (current_column + column as i64 - 1);

                if map_column < 0 || map_row < 0 || map_column >= self.num_columns as i64 || map_row >= self.num_rows as i64 {
                    indices[neighborhoods_index] = None;
                    continue;
                }
                let map_index: i64 = self.num_columns as i64 * map_row + map_column;

                indices[neighborhoods_index] = if map_index < 0 {
                    None
                } else if map_index < self.data.len() as i64 {
                    Some(map_index as usize)
                } else {
                    None
                };
            }
        }

        Some(indices)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    fn position_for_index(&self, index: usize) -> Option<(usize, usize)> {
        if index > self.data.len() - 1 {
            return None;
        }

        let row = index / self.num_columns;
        let column = index % self.num_columns;

        Some((row, column))
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let lines = self.data.clone().iter()
            .map(|v| if *v > 9 { "X".to_string() } else { format!("{}", v) })
            .collect::<Vec<_>>()
            .chunks(self.num_columns)
            .map(|chunk| chunk.join(""))
            .collect::<Vec<String>>();

        write!(
            f, "{}",
            lines.join("\n")
        )
    }
}

impl Index<(usize, usize)> for Map {
    type Output = u64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (y, x) = index;
        &self.data[y * self.num_columns + x]
    }
}

impl Index<usize> for Map {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<(usize, usize)> for Map {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (y, x) = index;
        &mut self.data[y * self.num_columns + x]
    }
}

impl IndexMut<usize> for Map {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

pub struct NeighborhoodIterator<'a> {
    map: &'a Map,
    current_index: usize,
}

impl<'a> NeighborhoodIterator<'a> {
    #[inline]
    fn current_position(&self) -> Option<(usize, usize)> {
        self.map.position_for_index(self.current_index)
    }
}

impl Center for [Option<u64>; 9] {
    fn center(&self) -> Option<u64> {
        self[4]
    }
}

impl<'a> Iterator for NeighborhoodIterator<'a> {
    type Item = [Option<u64>; 9];

    fn next(&mut self) -> Option<Self::Item> {
        let window = self.map.neighborhood(self.current_index)?;

        self.current_index += 1;

        Some(window)
    }
}