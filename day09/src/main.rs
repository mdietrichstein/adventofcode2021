mod map;

use map::Map;
use map::Center;

fn main() {
    let input = include_str!("../resources/input");

    let map = Map::new(input);
    let result = calculate_risk_level(&map);
    println!("[1/2] Result: {}", result);
}

fn calculate_risk_level(map: &Map) -> u64 {
    let result: u64 = map.window_iter().filter(
        |window| {
            let current = window.center().unwrap();
            let smaller = window.into_iter()
                .find(
                    |value| if let Some(v) = value {
                        v < &current
                    } else {
                        false
                    }
                );

            smaller.is_none()
        }
    ).map(|window| window.center().unwrap() + 1).sum();

    result
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DATA: &str = "
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    ";

    #[test]
    fn test_part1() {
        let map = Map::new(TEST_DATA);
        let result = calculate_risk_level(&map);
        assert_eq!(15, result);
    }

    #[test]
    fn test_part2() {}
}