use std::num::ParseIntError;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input
        .lines()
        .filter(|s| *s != "")
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<_>, ParseIntError>>()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<u32>) -> Result<usize, ParseIntError> {
    Ok(input
        .windows(2)
        .filter(|v| v[1] > v[0])
        .count())
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Vec<u32>) -> Result<usize, ParseIntError> {
    Ok(input
        .windows(4)
        .filter(|v| v[3] > v[0])
        .count())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_solve_part1() {
        let input = vec![ 199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let expected_output = 7;

        let output = solve_part1(&input).unwrap();

        assert_eq!(output, expected_output);
    }

    #[test]
    pub fn test_solve_part2() {
        let input = vec![ 199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let expected_output = 5;

        let output = solve_part2(&input).unwrap();

        assert_eq!(output, expected_output);
    }
}
