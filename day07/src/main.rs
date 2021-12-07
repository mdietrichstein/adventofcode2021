fn main() {

}

fn parse_input(input: &str) -> Vec<u32> {
    input.trim().split(',').map(|number| number.parse::<u32>().unwrap()).collect()
}

#[cfg(test)]
mod test {
    use crate::parse_input;

    const TEST_DATA: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part1() {
        let numbers = parse_input(TEST_DATA);
    }

    #[test]
    fn test_part2() {

    }
}