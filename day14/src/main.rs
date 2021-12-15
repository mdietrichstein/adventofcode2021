use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../resources/input");

    let score = calculate_score(input, 10);
    println!("[1/2] Result: {}", score);

    let score = calculate_score(input, 40);
    println!("[2/2] Result: {}", score);
}

fn parse_instructions(input: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let (start, rules) = input.trim().split_once("\n\n").unwrap();

    let start_word = start.chars().collect::<Vec<char>>();

    let rules = rules
        .split('\n')
        .map(|line| line.trim().split_once(" -> ").unwrap())
        .map(
            |(from, to)| {
                let from = from.trim().chars().collect::<Vec<char>>();
                (
                    (from.get(0).unwrap().clone(), from.get(1).unwrap().clone()),
                    to.trim().chars().next().unwrap()
                )
            }
        ).collect::<HashMap<_, _>>();

    (start_word, rules)
}

fn apply_rules_step(
    rules: &HashMap<(char, char), char>,
    pair_counts: HashMap<(char, char), usize>,
    char_counts: &mut HashMap<char, usize>,
) -> HashMap<(char, char), usize> {
    let mut result: HashMap<(char, char), usize> = HashMap::new();

    for (from, to) in rules {
        let (l, r) = from;

        if let Some(count) = pair_counts.get(from) {
            let new_pair_a = (*l, *to);
            let new_pair_b = (*to, *r);

            *char_counts.entry(*to).or_insert(0) += count;

            *result.entry(new_pair_a).or_insert(0) += *count;
            *result.entry(new_pair_b).or_insert(0) += *count;
        }
    }

    result
}

fn apply_rules(rules: &HashMap<(char, char), char>, start_word: &Vec<char>, num_steps: usize) -> HashMap<char, usize> {
    let mut char_counts = start_word.iter()
        .fold(
            HashMap::new(),
            |mut s, c| {
                *s.entry(*c).or_insert(0) += 1;
                s
            },
        );

    let mut result = start_word.windows(2)
        .map(|words| (words[0], words[1]))
        .fold(HashMap::new(), |mut s, chars| {
            *s.entry(chars).or_insert(0) += 1;
            s
        });

    for step in 0..num_steps {
        result = apply_rules_step(&rules, result, &mut char_counts);
    }

    char_counts
}

fn calculate_score(input: &str, num_steps: usize) -> usize {
    let (start_word, rules) = parse_instructions(input);
    let char_counts = apply_rules(&rules, &start_word, num_steps);

    let char_count_values = char_counts.iter().map(|(k, v)| *v).collect::<Vec<_>>();

    char_count_values.iter().max().unwrap() - char_count_values.iter().min().unwrap()
}


fn to_string(word: &Vec<char>) -> String {
    word.into_iter().map(|c| c.to_string()).collect::<Vec<String>>().join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C
    ";

    #[test]
    fn test_part1() {
        let (start_word, rules) = parse_instructions(TEST_DATA);
        let score = calculate_score(TEST_DATA, 10);
        assert_eq!(1588, score);
    }

    #[test]
    fn test_part2() {
        let (start_word, rules) = parse_instructions(TEST_DATA);
        let score = calculate_score(TEST_DATA, 40);
        assert_eq!(2188189693529, score);
    }
}