use std::collections::HashSet;

fn main() {
    let input = include_str!("../resources/input");
    let mappings = parse_string_mapping(input);
    let num_unique_outputs = count_unique_outputs(&mappings);
    println!("[1/2] Result: {}", num_unique_outputs);

    let mappings = parse_string_mapping(input);
    let result: usize = mappings.iter().map(solve_mapping).sum();
    println!("[2/2] Result: {}", result);
}

type StringMapping = (Vec<&'static str>, Vec<&'static str>);

fn parse_string_mapping(input: &'static str) -> Vec<StringMapping> {
    let entries =
        input.trim()
            .split('\n').map(|line| {
            let mut parts = line.split('|');
            (
                parts.next().unwrap().trim().split_ascii_whitespace().collect(),
                parts.next().unwrap().trim().split_ascii_whitespace().collect()
            )
        }).collect::<Vec<StringMapping>>();

    entries
}

fn count_unique_outputs(mappings: &Vec<StringMapping>) -> usize {
    mappings.into_iter().flat_map(|(_, output)| output).filter(|output| {
        match output.len()
        {
            // digits = 1, 4, 7, 8
            2 | 4 | 3 | 7 => true,
            _ => false
        }
    }).count()
}

fn find_with_num_segments(values: &Vec<&str>, num_segments: usize) -> Vec<HashSet<char>> {
    let result = values.into_iter().filter(|value| value.len() == num_segments);
    let c = result.map(|value| value.chars().collect::<HashSet<char>>()).collect::<Vec<_>>();
    c
}

fn find_with_num_segments_single(values: &Vec<&str>, num_segments: usize) -> HashSet<char> {
    let matches = find_with_num_segments(values, num_segments);
    assert_eq!(1, matches.len());

    let m = matches.first().unwrap();
    m.clone()
}

fn solve_mapping(mapping: &StringMapping) -> usize{
    let (alphabet, output) = mapping;

    let one = find_with_num_segments_single(&alphabet, 2);
    let four = find_with_num_segments_single(&alphabet, 4);
    let seven = find_with_num_segments_single(&alphabet, 3);
    let eight = find_with_num_segments_single(&alphabet, 7);

    let two_three_five = find_with_num_segments(&alphabet, 5);
    assert_eq!(3, two_three_five.len());

    let three = two_three_five.iter().find(|chars| {
        one.is_subset(chars)
    }).unwrap().clone();

    let two_five = two_three_five.into_iter().filter(|chars| !three.eq(chars)).collect::<Vec<HashSet<_>>>();
    assert_eq!(2, two_five.len());

    let part_of_four: HashSet<char> = four.difference(&one).map(|v| *v).collect::<HashSet<_>>();
    assert_eq!(2, part_of_four.len());

    let five: HashSet<char> = two_five.iter().find(|chars| part_of_four.is_subset(chars)).unwrap().clone();
    let two = two_five.into_iter().find(|chars| !five.eq(chars)).unwrap();

    let zero_six_nine = find_with_num_segments(&alphabet, 6);
    assert_eq!(3, zero_six_nine.len());

    let six = zero_six_nine.iter().find(|chars| {
        !one.is_subset(chars)
    }).unwrap().clone();

    let zero_nine = zero_six_nine.into_iter().filter(|chars| !six.eq(chars)).collect::<Vec<HashSet<_>>>();
    assert_eq!(2, zero_nine.len());

    let zero = zero_nine.iter().find(|chars| {
        !four.is_subset(chars)
    }).unwrap().clone();

    let nine = zero_nine.iter().find(|chars| {
        four.is_subset(chars)
    }).unwrap().clone();

    let solution = [
        &zero, &one, &two, &three, &four, &five, &six, &seven, &eight, &nine
    ];

    let result: usize = output.into_iter()
        .map(|string| string.chars().collect::<HashSet<char>>())
        .map(
                |chars|
                    solution.iter().position(|entry| chars.eq(entry)).unwrap()
        )
        .enumerate()
        .map(|(i, value)|
            value * 10_usize.pow(3 - i as u32)
        ).sum();


    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    ";

    #[test]
    fn test_part1() {
        let mappings = parse_string_mapping(TEST_DATA);
        let num_unique_outputs = count_unique_outputs(&mappings);
        assert_eq!(26, num_unique_outputs);
    }

    #[test]
    fn test_part2() {
        let mappings = parse_string_mapping(TEST_DATA);
        let result: usize = mappings.iter().map(solve_mapping).sum();
        assert_eq!(61229, result);
    }
}