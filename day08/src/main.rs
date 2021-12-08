fn main() {
    let input = include_str!("../resources/input");
    let mappings = parse_string_mapping(input);
    let num_unique_outputs = count_unique_outputs(&mappings);
    println!("[1/2] Result: {}", num_unique_outputs);

}

type StringMapping = (Vec<&'static str>, Vec<&'static str>);

fn parse_string_mapping(input: &'static str) -> Vec<StringMapping> {
    let entries=
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
            2 | 4 | 3 | 7 => true,
            _ => false
        }
    }).count()
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

    }
}