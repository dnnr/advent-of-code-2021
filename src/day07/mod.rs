#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

#[aoc(day7, part1)]
pub fn solve_part1(crabs: &[u32]) -> u32 {
    let crabs_median = median(crabs);
    compute_fuel(crabs, &crabs_median)
}

#[aoc(day7, part2)]
pub fn solve_part2(crabs: &[u32]) -> u32 {
    (*crabs.iter().min().unwrap()..(crabs.iter().max().unwrap() + 1))
        .map(|x| compute_fuel_part2(crabs, &x))
        .min()
        .unwrap()
}

fn compute_fuel(crabs: &[u32], target: &u32) -> u32 {
    crabs
        .iter()
        .map(|x| if x > target { x - target } else { target -x })
        .sum()
}

fn compute_fuel_part2(crabs: &[u32], target: &u32) -> u32 {
    crabs
        .iter()
        .map(|x| if x > target { x - target } else { target -x })
        .map(|x| (x.pow(2) + x) / 2)
        .sum()
}

fn median(numbers: &[u32]) -> u32 {
    let mut vec = numbers.to_vec();
    vec.sort_unstable();

    let mid = vec.len() / 2;
    if numbers.len() % 2 == 0 {
        vec[mid - 1]
    } else {
        vec[mid]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    pub fn sample_str() -> String {
        String::from("16,1,2,0,4,2,7,1,2,14")
    }

    #[test]
    pub fn test_input_generator() {
        let crabs = input_generator(sample_str().as_str());

        assert_eq!(crabs, vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
    }

    #[test]
    pub fn test_solve_part1() {
        let crabs = input_generator(sample_str().as_str());
        let expected_fuel = 37;

        let fuel = solve_part1(&crabs);

        assert_eq!(fuel, expected_fuel);
    }

    #[test]
    pub fn test_solve_part2() {
        let crabs = input_generator(sample_str().as_str());
        let expected_fuel = 168;

        let fuel = solve_part2(&crabs);

        assert_eq!(fuel, expected_fuel);
    }
}
