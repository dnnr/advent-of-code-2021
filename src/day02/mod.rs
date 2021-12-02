use std::num::ParseIntError;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(String, Result<usize, ParseIntError>)> {
    input
        .lines()
        .filter(|s| *s != "")
        .map(|s| s.split(" ").map(|x| x.to_owned()).collect::<Vec<String>>())
        .map(|v| (v[0].clone(), v[1].parse::<usize>()))
        .collect::<Vec<(String, Result<usize, ParseIntError>)>>()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<(String, Result<usize, ParseIntError>)>) -> Result<usize, ParseIntError> {
    let mut depth = 0;
    let mut distance = 0;

    for (direction, amount) in input {
        match direction.as_str() {
            "forward" => {
                // TODO: avoid unwrap?
                // TODO: avoid as_ref?
                distance += amount.as_ref().unwrap();
            },
            "down" => {
                depth += amount.as_ref().unwrap();
            },
            "up" => {
                depth -= amount.as_ref().unwrap();
            },
            _ => {
                panic!("Wut");
            }
        }
    }

    Ok(depth * distance)
}

#[cfg(test)]
mod test {
    use super::*;

    pub fn sample() -> Vec<(String, Result<usize, ParseIntError>)> {
        vec![
            (String::from("forward"), Ok(5)),
            (String::from("down"), Ok(5)),
            (String::from("forward"), Ok(8)),
            (String::from("up"), Ok(3)),
            (String::from("down"), Ok(8)),
            (String::from("forward"), Ok(2)),
        ]
    }

    #[test]
    pub fn test_solve_part1() {
        let input = sample();
        let expected_output = 150;

        let output = solve_part1(&input).unwrap();

        assert_eq!(output, expected_output);
    }
}
