pub fn part1(inp: String) {
    let measurements = input_to_measurements(inp);
    let differential = compute_differential(measurements);
    let increases = count_positives(differential);

    println!("Increases: {}", increases);
}

pub fn part2(_inp: String) {
}

pub fn input_to_measurements(inp: String) -> Vec<usize> {
    let lines = inp.split('\n').collect::<Vec<&str>>();

    lines.iter()
        .filter(|x| x != &&"")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

pub fn compute_differential(values: Vec<usize>) -> Vec<isize> {
    let mut ret = Vec::<isize>::new();
    if values.len() <= 1 {
        return ret;
    }
    let mut previous = values[0] as isize;
    for value in values.iter().skip(1).map(|x| x.to_owned() as isize) {
        ret.push(value - previous);
        previous = value;
    }
    ret
}

pub fn count_positives(values: Vec<isize>) -> usize {
    values.iter()
        .filter(|x| x > &&0)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_parse_input() {
        let input = "23\n42\n45";
        let expected_measurements = vec![23, 42, 45];

        let measurements = input_to_measurements(input.to_owned());

        assert_eq!(measurements, expected_measurements);
    }


    #[test]
    pub fn test_compute_differential() {
        let input = vec![23, 42, 45, 44];
        let expected_output = vec![19, 3, -1];

        let output = compute_differential(input);

        assert_eq!(output, expected_output);
    }


    #[test]
    pub fn test_count_positives() {
        let input = vec![23, -42, 45, 44];
        let expected_count = 3;

        let output = count_positives(input);

        assert_eq!(output, expected_count);
    }
}
