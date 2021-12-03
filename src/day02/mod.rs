use std::num::ParseIntError;
use std::str::FromStr;


#[derive(Debug)]
enum Direction {
    Up, Down, Forward
}

#[derive(Debug)]
pub struct Command {
    direction: Direction,
    amount: usize,
}

impl FromStr for Command {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split(' ').collect::<Vec<&str>>();
        let direction = match v[0] {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => panic!("Cannot parse '{}' as direction", v[0])
        };

        let amount = v[1].parse::<usize>();

        Ok(Command { direction, amount: amount? })
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Result<Vec<Command>, ParseIntError> {
    input
        .lines()
        .filter(|s| !(*s).is_empty())
        .map(|s| Command::from_str(s))
        .collect::<Result<Vec<Command>, ParseIntError>>()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Command]) -> usize {
    let mut depth = 0;
    let mut distance = 0;

    for command in input {
        match command.direction {
            Direction::Forward => {
                distance += command.amount;
            },
            Direction::Down => {
                depth += command.amount;
            },
            Direction::Up => {
                depth -= command.amount;
            }
        }
    }

    depth * distance
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Command]) -> usize {
    let mut depth = 0;
    let mut distance = 0;
    let mut aim = 0;

    for command in input {
        match command.direction {
            Direction::Forward => {
                distance += command.amount;
                depth += command.amount * aim;
            },
            Direction::Down => {
                aim += command.amount;
            },
            Direction::Up => {
                aim -= command.amount;
            }
        }
    }

    depth * distance
}


#[cfg(test)]
mod test {
    use super::*;

    pub fn sample_str() -> String {
        String::from("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n")
    }

    #[test]
    pub fn test_solve_part1() {
        let input = input_generator(sample_str().as_str()).unwrap();
        let expected_output = 150;

        let output = solve_part1(&input);

        assert_eq!(output, expected_output);
    }


    #[test]
    pub fn test_solve_part2() {
        let input = input_generator(sample_str().as_str()).unwrap();
        let expected_output = 900;

        let output = solve_part2(&input);

        assert_eq!(output, expected_output);
    }
}
