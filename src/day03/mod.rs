#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .filter(|s| !(*s).is_empty())
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[String]) -> usize {
    let most_common = get_value_balance_per_bit(input);

    let mut gamma_rate: usize = 0;
    for value in most_common {
        gamma_rate <<= 1;
        gamma_rate += if value > 0 { 1 } else { 0 };
    }

    let bitcount = input.iter().map(|x| x.len()).max().unwrap();
    let epsilon_rate = usize::pow(2, bitcount as u32) - gamma_rate - 1;

    gamma_rate * epsilon_rate
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[String]) -> usize {
    get_oxygen_generator_rating(input) * get_co2_scrubber_rating(input)

}

pub fn get_value_balance_per_bit(input: &[String]) -> Vec<i32> {
    let bitcount = input.iter().map(|x| x.len()).max().unwrap();
    let mut balance_per_bit = vec![0; bitcount];

    for value in input {
        for (bit_pos, bit_value) in value.chars().enumerate() {
            if bit_value == '1' {
                balance_per_bit[bit_pos] += 1;
            } else {
                balance_per_bit[bit_pos] -= 1;
            }
        }
    }

    balance_per_bit
}

pub fn filter_by_bit_value(input: &[String], bit_pos: usize, bit_value: usize) -> Vec<String> {
    let mut result = Vec::new();
    for value in input {
        if value.as_bytes()[bit_pos] as char == char::from_digit(bit_value as u32, 10).unwrap() {
            result.push(value.to_owned());
        }
    }

    result
}

pub fn get_oxygen_generator_rating(input: &[String]) -> usize {
    let bitcount = input.iter().map(|x| x.len()).max().unwrap();

    let mut remaining = input.to_owned();
    for bit_pos in 0..bitcount {
        let balance_per_bit = get_value_balance_per_bit(&remaining);
        let most_common = if balance_per_bit[bit_pos] >= 0 { 1 } else { 0 };

        remaining = filter_by_bit_value(&remaining, bit_pos, most_common);

        if remaining.len() == 1 {
            return usize::from_str_radix(&remaining[0], 2).unwrap();
        }
    }

    panic!("No oxygen generator rating found (remaining input: {:?})", remaining);
}

pub fn get_co2_scrubber_rating(input: &[String]) -> usize {
    let bitcount = input.iter().map(|x| x.len()).max().unwrap();

    let mut remaining = input.to_owned();
    for bit_pos in 0..bitcount {
        let balance_per_bit = get_value_balance_per_bit(&remaining);
        let least_common = if balance_per_bit[bit_pos] < 0 { 1 } else { 0 };

        remaining = filter_by_bit_value(&remaining, bit_pos, least_common);

        if remaining.len() == 1 {
            return usize::from_str_radix(&remaining[0], 2).unwrap();
        }
    }

    panic!("No oxygen generator rating found (remaining input: {:?})", remaining);
}

#[cfg(test)]
mod test {
    use super::*;

    pub fn sample_str() -> String {
        String::from("00100
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
")
    }

    #[test]
    pub fn test_solve_part1() {
        let input = input_generator(sample_str().as_str());
        let expected_output = 198;

        let output = solve_part1(&input);

        assert_eq!(output, expected_output);
    }

    #[test]
    pub fn test_get_oxygen_generator_rating() {
        let input = input_generator(sample_str().as_str());
        let expected_output = 23;

        let output = get_oxygen_generator_rating(&input);

        assert_eq!(output, expected_output);
    }

    #[test]
    pub fn test_get_co2_scrubber_rating() {
        let input = input_generator(sample_str().as_str());
        let expected_output = 10;

        let output = get_co2_scrubber_rating(&input);

        assert_eq!(output, expected_output);
    }

    #[test]
    pub fn test_solve_part2() {
        let input = input_generator(sample_str().as_str());
        let expected_output = 230;

        let output = solve_part2(&input);

        assert_eq!(output, expected_output);
    }
}
