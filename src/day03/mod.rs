#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .filter(|s| *s != "")
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Vec<String>) -> usize {
    let bitcount = input.iter().map(|x| x.len()).max().unwrap();
    let mut ones_per_pos = vec![0; bitcount];

    for value in input {
        for (bit_pos, bit_value) in value.chars().enumerate() {
            if bit_value == '1' {
                ones_per_pos[bit_pos] += 1;
            }
        }
    }

    let mut gamma_rate: usize = 0;
    for ones in ones_per_pos {
        gamma_rate <<= 1;
        if (ones * 2) >= input.len() {
            gamma_rate += 1;
        }
    }

    let epsilon_rate = usize::pow(2, bitcount as u32) - gamma_rate - 1;

    gamma_rate * epsilon_rate
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
}
