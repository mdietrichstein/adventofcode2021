use std::ops::Index;

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

    pub fn window_iter(&self) -> WindowIterator {
        WindowIterator {
            map: self,
            current_index: 0,
        }
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

impl Index<(usize, usize)> for Map {
    type Output = u64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (y, x) = index;
        &self.data[y * self.num_columns + x]
    }
}

pub struct WindowIterator<'a> {
    map: &'a Map,
    current_index: usize,
}

impl<'a> WindowIterator<'a> {
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

impl<'a> Iterator for WindowIterator<'a> {
    type Item = [Option<u64>; 9];

    fn next(&mut self) -> Option<Self::Item> {
        let (current_row, current_column) = self.current_position()?;
        let current_row = current_row as i64;
        let current_column = current_column as i64;

        let mut window = [None; 9];

        for row in 0..3 {
            for column in 0..3 {
                window[3 * row as usize + column as usize] = match (current_row + row - 1, current_column + column - 1) {
                    (r, c) if r < 0 || c < 0 || r as usize >= self.map.num_rows || c as usize >= self.map.num_columns => None,
                    (r, c) => Some(self.map[(r as usize, c as usize)])
                };
            }
        }

        self.current_index += 1;

        Some(window)
    }
}