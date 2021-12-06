const MAX_CYCLE: usize = 9;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn convert_gens_to_fishes_per_age(input: &[u64]) -> [u64; MAX_CYCLE] {
    let mut fishes_per_age: [u64; MAX_CYCLE] = [0; MAX_CYCLE];

    for gen in input {
        if (*gen as usize) < fishes_per_age.len() {
            fishes_per_age[*gen as usize] += 1;
        }
    }

    fishes_per_age
}

fn iterate(fishes_per_age: &mut [u64]) {
    let spawns = fishes_per_age[0];

    for age in 1..fishes_per_age.len() {
        fishes_per_age[age - 1] = fishes_per_age[age];
    }

    fishes_per_age[6] += spawns;
    fishes_per_age[8] = spawns;

}

#[aoc(day6, part1)]
pub fn solve_part1(gens: &[u64]) -> u64 {
    let mut fishes_per_age = convert_gens_to_fishes_per_age(gens);

    for _ in 0..80 {
        iterate(&mut fishes_per_age);
    }

    fishes_per_age.iter().sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(gens: &[u64]) -> u64 {
    let mut fishes_per_age = convert_gens_to_fishes_per_age(gens);

    for _ in 0..256 {
        iterate(&mut fishes_per_age);
    }

    fishes_per_age.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    pub fn sample_str() -> String {
        String::from("3,4,3,1,2")
    }

    #[test]
    pub fn test_input_generator() {
        let fishes = input_generator(sample_str().as_str());

        assert_eq!(fishes, vec![3, 4, 3, 1, 2]);
    }


    #[test]
    pub fn test_convert_gens_to_fishes_per_age() {
        let gens = input_generator(sample_str().as_str());
        let expected_output = [0, 1, 1, 2, 1, 0, 0, 0 ,0];

        let output = convert_gens_to_fishes_per_age(&gens);

        assert_eq!(output, expected_output);
    }

    #[test]
    pub fn test_iterate_no_spawn() {
        let mut input =         [0, 1, 1, 2, 1, 0, 0, 0 ,0];
        let expected_output =   [1, 1, 2, 1, 0, 0, 0, 0, 0];

        iterate(&mut input);

        assert_eq!(input, expected_output);
    }

    #[test]
    pub fn test_iterate_with_spawn() {
        let mut input = [1, 1, 2, 1, 0, 0, 0, 0, 0];
        let expected_output = [1, 2, 1, 0, 0, 0, 1, 0, 1];

        iterate(&mut input);

        assert_eq!(input, expected_output);
    }

    #[test]
    pub fn test_solve_part1() {
        let input = input_generator(sample_str().as_str());
        let expected_output = 5934;

        let output = solve_part1(&input);

        assert_eq!(output, expected_output);
    }


    #[test]
    pub fn test_solve_part2() {
        let input = input_generator(sample_str().as_str());
        let expected_output = 26984457539;

        let output = solve_part2(&input);

        assert_eq!(output, expected_output);
    }
}
