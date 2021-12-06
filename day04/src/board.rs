#[derive(Debug)]
pub struct Board {
    pub board_index: usize,
    pub board_size: usize,
    pub values: Vec<u32>,
    matches: Vec<usize>,
    pub has_won: bool,
}

impl Board {
    pub fn new(board_index: usize, board_size: usize, values: Vec<u32>) -> Self {
        if values.len() != board_size * board_size {
            panic!("Invalid number of values for board {} (expected: {}, actual: {})", board_index, board_size * board_size, values.len());
        }
        

        Board {
            board_index,
            board_size,
            values,
            matches: vec![],
            has_won: false,
        }
    }

    pub fn update(&mut self, number: u32) {
        if let Some(position) = self.values.iter().position(|value| *value == number) {
            self.matches.push(position);
            
        } else {
            return;
        }

        if self.has_won {
            return;
        }

        if self.matches.len() < self.board_size {
            return;
        }

        
        let (rows, columns): (Vec<_>, Vec<_>) = self.matches.clone()
            .into_iter()
            .map(|position| (
                position / self.board_size,
                position % self.board_size
            ))
            .unzip();
        

        for i in 0..self.board_size {
            let is_row_full = rows.iter().filter(|row_index| **row_index == i).count() >= self.board_size;

            if is_row_full {
                self.has_won = true;
                break;
            }

            let is_column_full = columns.iter().filter(|column_index| **column_index == i).count() >= self.board_size;

            if is_column_full {
                self.has_won = true;
                break;
            }
        }
    }

    pub fn score(&self) -> u32 {
        self.values.iter()
            .enumerate()
            .filter(|(i, value)| !self.matches.contains(i))
            .map(|(i, value)| value)
            .sum()
    }
}