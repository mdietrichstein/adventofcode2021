use std::error::Error;
use std::num::ParseIntError;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../resources/input");

    let measurements = parse_measurements(input)?;
    println!("[1/2] Result: {}", count_increases(&measurements, 1));
    println!("[2/2] Result: {}", count_increases(&measurements, 3));

    //
    // We do not need to sum over the chunks, since each sum contains (chunk_size - 1) identical
    // values: a + b + c < b + c + d = a < d
    // let chunk_size = 3;
    // let grouped_measurements = measurements.windows(chunk_size)
    //     .map(
    //         |window|
    //             (0..chunk_size).map(|i| window[i]).sum()
    //     ).collect();

    Ok(())
}

fn parse_measurements(input: &str) -> Result<Vec<i64>, Box<dyn Error>> {
    input.trim().split('\n')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>().map_err(|e: ParseIntError| e.into())
}

fn count_increases(measurements: &Vec<i64>, step_size: usize) -> usize {
    measurements.windows(step_size + 1).filter(|w| w.first() < w.last()).count()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const TEST_INPUT: &str = indoc! {"
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
    "};

    #[test]
    fn test_part1() {
        let measurements: Vec<i64> = parse_measurements(test_input).unwrap();
        let result = count_increases(&measurements, 1);

        assert_eq!(result, 7);
    }

    #[test]
    fn test_part2() {
        let measurements: Vec<i64> = parse_measurements(test_input).unwrap();
        let result = count_increases(&measurements, 5);

        assert_eq!(result, 5);
    }
}