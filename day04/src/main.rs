mod board;

use board::Board;

fn main() {
    let board_size = 5;

    let input = include_str!("../resources/input");


    let (bingo_numbers, mut boards) = load_data(input, board_size);
    let (_, score ) = find_winner(&bingo_numbers, &mut boards).unwrap();
    println!("[1/2] Result: {}", score);

    let (bingo_numbers, boards) = load_data(input, board_size);
    let (_, score ) = find_last_winner(&bingo_numbers, boards).unwrap();
    println!("[2/2] Result: {}", score);
}

fn load_data(input: &str, board_size: usize) -> (Vec<u32>, Vec<Board>) {
    let mut lines = input.trim().split('\n').collect::<Vec<&str>>().into_iter();
    let bingo_numbers: Vec<u32> = lines.next().unwrap().split(',').map(|c| c.parse::<u32>().unwrap()).collect();

    let boards: Vec<Board> = 
        lines.as_slice()
            .chunks(board_size + 1)
            .map(|board_lines| board_lines.join(" ").trim().into())
            .map(|line: String| str_to_numbers(&line))
            .enumerate()
            .map(|(i, values)| Board::new(i, board_size, values))
            .collect();


    (bingo_numbers, boards)
}

fn str_to_numbers<T: AsRef<str>>(input: T) -> Vec<u32> {
    input.as_ref().split_whitespace().map(|v|v.parse::<u32>()).collect::<Result<Vec<u32>, _>>().unwrap()
}

fn find_winner(bingo_numbers: &Vec<u32>, boards: &mut Vec<Board>) -> Option<(usize, u32)> {
    for number in bingo_numbers {
        for board in &mut boards.iter_mut() {
            board.update(*number);

            if board.has_won {
                return Some(
                    (board.board_index, number * board.score())
                );
            }
        }
    }

    None
}

fn find_last_winner(bingo_numbers: &Vec<u32>, boards: Vec<Board>) -> Option<(usize, u32)> {

    let mut boards = boards;
    
    for number in bingo_numbers {
        for board in &mut boards.iter_mut() {
            board.update(*number);
        }

        if boards.len() == 1 {
            let last_board = boards.first().unwrap();

            if last_board.has_won {
                return Some((last_board.board_index, last_board.score() * number));
            }
        }

        boards = boards.into_iter().filter(|board| !board.has_won).collect();
    }

    None
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA:&str = "
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
        8  2 23  4 24
        21  9 14 16  7
        6 10  3 18  5
        1 12 20 15 19
        
        3 15  0  2 22
        9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6
        
        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
        2  0 12  3  7
    ";

    #[test]
    fn test_part1() {
        let board_size = 5;

        let (bingo_numbers, mut boards) = load_data(TEST_DATA, board_size);
        let (board_index, score ) = find_winner(&bingo_numbers, &mut boards).unwrap();

        assert_eq!(2, board_index);
        assert_eq!(4512, score);
    }

    #[test]
    fn test_part2() {
        let board_size = 5;

        let (bingo_numbers, boards) = load_data(TEST_DATA, board_size);
        let (board_index, score ) = find_last_winner(&bingo_numbers, boards).unwrap();

        assert_eq!(1, board_index);
        assert_eq!(1924, score);
    }
}