use std::str::FromStr;

fn main() -> Result<(), ()> {
    let input = include_str!("../resources/input");
    let directions = directions(input)?;

    let mut position = Position::new();
    for direction in &directions {
        position.navigate_part1(&direction);
    }

    println!("[1/2] Result: {}", position.x * position.z);

    let mut position = Position::new();
    for direction in &directions {
        position.navigate_part2(&direction);
    }

    println!("[2/2] Result: {}", position.x * position.z);

    Ok(())
}

#[derive(Debug)]
struct Position {
    x: i64,
    z: i64,
    aim: i64,
}

#[derive(Debug)]
enum Direction {
    Forward(i64),
    Down(i64),
    Up(i64),
}

impl Position {
    fn new() -> Self {
        Position {
            x: 0,
            z: 0,
            aim: 0,
        }
    }

    fn navigate_part1(&mut self, direction: &Direction) {
        match direction {
            Direction::Forward(amount) => self.x += amount,
            Direction::Up(amount) => self.z -= amount,
            Direction::Down(amount) => self.z += amount,
        }
    }

    fn navigate_part2(&mut self, direction: &Direction) {
        match direction {
            Direction::Forward(amount) => {
                self.x += amount;
                self.z += self.aim * amount;
            }
            Direction::Up(amount) => self.aim -= amount,
            Direction::Down(amount) => self.aim += amount,
        }
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        let (direction, amount) = (parts[0], parts[1].parse::<i64>().expect("could not parse amount into number"));

        match direction {
            "forward" => Ok(Direction::Forward(amount)),
            "up" => Ok(Direction::Up(amount)),
            "down" => Ok(Direction::Down(amount)),
            _ => Err(())
        }
    }
}

fn directions(input: &str) -> Result<Vec<Direction>, ()> {
    input.trim().split('\n')
        .map(|line| Direction::from_str(line))
        .collect::<Result<Vec<_>, _>>()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const TEST_INPUT: &str = indoc! {"
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    "};

    #[test]
    fn test_part1() {
        let directions: Vec<Direction> = directions(TEST_INPUT).unwrap();
        let mut position = Position::new();

        for direction in &directions {
            position.navigate_part1(&direction);
        }

        assert_eq!(150, position.x * position.z);
    }

    #[test]
    fn test_part2() {
        let directions: Vec<Direction> = directions(TEST_INPUT).unwrap();
        let mut position = Position::new();

        for direction in &directions {
            position.navigate_part2(&direction);
        }

        assert_eq!(900, position.x * position.z);
    }
}