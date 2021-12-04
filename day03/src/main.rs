use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>
{
    let input = include_str!("../resources/input");
    let values = parse_inputs(input).ok_or("Unable to parse inputs")?;
    let (gamma, epsilon) = calculate_gamma_epsilon(&values);

    println!("[1/2] Result: {}", gamma * epsilon);

    let oxygen_rating =
        bits_to_value(
            calculate_criteria(&values, true).unwrap()
        );

    let co2_scrubber_rating =
        bits_to_value(
            calculate_criteria(&values, false).unwrap()
        );

    println!("[2/2] Result: {}", oxygen_rating * co2_scrubber_rating);

    Ok(())
}

fn parse_inputs(input: &str) -> Option<Vec<Vec<u32>>> {
    input.trim().split('\n').map(|line|
        line.chars().map(|c| c.to_digit(10)).collect::<Option<Vec<_>>>()
    ).collect::<Option<Vec<Vec<_>>>>()
}

fn calculate_gamma_epsilon(values: &Vec<Vec<u32>>) -> (u32, u32) {
    let num_bits: u32 = values.first().unwrap().len() as u32;
    let one_threshold: u32 = (values.len() / 2) as u32;

    let mut bits: Vec<u32> = vec![];

    for i in 0..num_bits {
        let num_ones = values.iter().map(|value| value[i as usize]).sum::<u32>();
        let has_more_ones = num_ones >= one_threshold;

        if has_more_ones {
            bits.push(1);
        } else {
            bits.push(0);
        }
    }

    let gamma = bits_to_value(bits);
    let mask = 2_u32.pow(num_bits) - 1;
    let epsilon = !gamma & mask;
    (gamma, epsilon)
}

fn calculate_criteria(values: &Vec<Vec<u32>>, most_common: bool) -> Option<Vec<u32>> {
    let num_bits: u32 = values.first().unwrap().len() as u32;
    let mut result = values.clone();

    for i in 0..num_bits {
        let bit_index = i as usize;
        let num_ones = result.iter().map(|value| value[bit_index]).sum::<u32>();

        let target_value = if num_ones >= (result.len() as u32 - num_ones) {
            if most_common { 1 } else { 0 }
        } else {
            if most_common { 0 } else { 1 }
        };

        result = result
            .into_iter()
            .filter(|value| {
                value[bit_index] == target_value
        }).collect();

        if result.len() == 1 {
            return result.first().cloned();
        }
    }

    None
}

fn bits_to_value(bits: Vec<u32>) -> u32 {
    let num_bits = bits.len();

    let mut value: u32 = 0;

    for i in 0..num_bits {
        if bits[i] == 1 {
            value = value + 2_u32.pow((num_bits - 1 - i) as u32);
        }
    }

    value
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const TEST_DATA: &str = indoc! {"
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
    "};

    #[test]
    fn test_part1() {
        let values = parse_inputs(TEST_DATA).unwrap();
        let (gamma, epsilon) = calculate_gamma_epsilon(&values);

        assert_eq!(198, gamma * epsilon);
    }

    #[test]
    fn test_part2() {
        let values = parse_inputs(TEST_DATA).unwrap();
        let oxygen_rating =
            bits_to_value(
                calculate_criteria(&values, true).unwrap()
            );

        let co2_scrubber_rating =
            bits_to_value(
                calculate_criteria(&values, false).unwrap()
            );

        assert_eq!(230, oxygen_rating * co2_scrubber_rating);
    }
}