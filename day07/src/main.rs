fn main() {
    let input = include_str!("../resources/input");
    let numbers = parse_input(input);

    let moves = find_optimal_moves_1(&numbers);
    println!("[1/2] Result: {}", moves);

    let moves = find_optimal_moves_2(&numbers);
    println!("[2/2] Result: {}", moves);
}

fn parse_input(input: &str) -> Vec<i64> {
    input.trim().split(',').map(|number| number.parse::<i64>().unwrap()).collect()
}

fn find_optimal_moves_1(numbers: &Vec<i64>) -> i64 {
    let mut best_moves: i64 = i64::MAX;

    for current in numbers {
        let moves = numbers.iter().map(|number| (number - current).abs()).sum();
        if moves < best_moves {
            best_moves = moves;
        }
    }

    best_moves
    // This can be sped up by using the median:
    // let mut numbers = numbers;
    // numbers.sort();
    // let median = numbers[numbers.len() / 2];
    // let moves = numbers.iter().map(|number| (number - median).abs()).sum();
    // moves
}

fn find_optimal_moves_2(numbers: &Vec<i64>) -> i64 {
    let mut best_moves: i64 = i64::MAX;

    for current in numbers {
        let moves = numbers.iter()
            .map(|number| {
                let steps: i64 = (number - current).abs();
                // (1..steps + 1).sum::<i64>()

                // better (https://math.stackexchange.com/questions/1100897/sum-of-consecutive-numbers)
                (steps * (steps + 1)) / 2
            }
            ).sum();
        if moves < best_moves {
            best_moves = moves;
        }
    }

    best_moves
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DATA: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part1() {
        let numbers = parse_input(TEST_DATA);
        let moves = find_optimal_moves_1(&numbers);

        assert_eq!(37, moves);
    }

    #[test]
    fn test_part2() {
        let numbers = parse_input(TEST_DATA);
        let moves = find_optimal_moves_2(&numbers);
        assert_eq!(170, moves);
    }
}