fn main() {
    let input = include_str!("../resources/input");

    let corruption_score = calculate_corruption_score(input);
    println!("[1/2] Result: {}", corruption_score);

    let completion_score = calculate_completion_score(input);
    println!("[2/2] Result: {}", completion_score);
}

const OPEN_BRACES: [char; 4] = ['(', '[', '{', '<'];
const CLOSING_BRACES: [char; 4] = [')', ']', '}', '>'];

fn corrupted_line_char(line: &str) -> Option<char> {
    let mut visited_braces = vec![];

    for c in line.trim().chars() {
        if OPEN_BRACES.contains(&c) {
            visited_braces.push(c);
        } else if let Some(index) = CLOSING_BRACES.iter().position(|n| *n == c) {
            let matching_open_brace = OPEN_BRACES[index];

            if let Some(last_open_brace) = visited_braces.last() {
                if *last_open_brace != matching_open_brace {
                    return Some(c);
                }
                visited_braces.pop();
            } else {
                return Some(c);
            }
        } else {
            panic!("Invalid character '{:?}'", c);
        }
    }

    None
}

fn complete_line(line: &str) -> Option<Vec<char>> {
    let mut visited_braces = vec![];

    for c in line.trim().chars() {
        if OPEN_BRACES.contains(&c) {
            visited_braces.push(c);
        } else if let Some(index) = CLOSING_BRACES.iter().position(|n| *n == c) {
            let matching_open_brace = OPEN_BRACES[index];

            if let Some(last_open_brace) = visited_braces.last() {
                if *last_open_brace != matching_open_brace {
                    return None;
                }
                visited_braces.pop();
            }
        } else {
            panic!("Invalid character '{:?}'", c);
        }
    }

    Some(
        visited_braces.into_iter().map(|c| {
            let index = OPEN_BRACES.iter().position(|n| *n == c);
            if let Some(index) = index {
                CLOSING_BRACES[index]
            } else {
                panic!("Invalid character '{:?}'", c);
            }
        }).rev().collect::<Vec<char>>()
    )
}

fn corruption_score_for_char(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Invalid character '{:?}'", c)
    }
}

fn calculate_corruption_score(input: &str) -> usize {
    input.trim()
        .split('\n')
        .map(|line| corrupted_line_char(line))
        .map(|c| {
            match c {
                Some(c) => corruption_score_for_char(c),
                _ => 0
            }
        }).sum()
}

fn completion_score_for_char(c: char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Invalid character '{:?}'", c)
    }
}

fn completion_score_for_line(line: &str) -> usize {
    complete_line(line).unwrap().into_iter()
        .map(completion_score_for_char)
        .fold(0, |sum, char_score| (sum * 5) + char_score)
}

fn calculate_completion_score(input: &str) -> usize {
    let mut scores = input.trim()
        .split('\n')
        .filter(|line| corrupted_line_char(line).is_none())
        .map(|line| completion_score_for_line(line))
        .collect::<Vec<usize>>();

    scores.sort();

    scores[scores.len() / 2]
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DATA: &str = "
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
    ";


    #[test]
    fn test_part1() {
        let score = calculate_corruption_score(TEST_DATA);
        assert_eq!(26397, score);
    }

    #[test]
    fn test_part2() {
        let score = calculate_completion_score(TEST_DATA);
        assert_eq!(288957, score);
    }
}