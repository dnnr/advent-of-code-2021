use std::collections::HashMap;
use itertools::Itertools;

pub struct Input {
    start: String,
    rules: HashMap<(char, char), char>
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Input {
    let (start, rules_str) = input.split_once("\n\n").unwrap();

    let rules_map: HashMap<_, _> = rules_str
        .lines()
        .map(|line| line
            .split_once(" -> ")
            .unwrap()
            )
        .map(|(l, r)|
            (l.chars().next_tuple::<(_, _)>().unwrap(),
            r.chars().next().unwrap()))
        .collect();

    Input {
        start: start.to_string(),
        rules: rules_map,
    }
}

fn apply_rules(s: &str, rules: &HashMap<(char, char), char>, iterations: usize) -> u64 {
    // Turn input into map of counts per pair
    let mut pair_counts = s
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .map(|w| w.iter().cloned().collect_tuple::<(char, char)>().unwrap())
        .fold(HashMap::new(), |mut hist, pair| {
            *hist.entry(pair).or_insert(0u64) += 1;
            hist
        });

    for _ in 0..iterations {
        for (pair, count) in pair_counts.clone().iter() {
            if let Some(insert) = rules.get(pair) {
                // Replace pair with two new ones
                *pair_counts.get_mut(pair).unwrap() -= count;
                let (l, r) = pair;
                *pair_counts.entry((*l, *insert)).or_insert(0u64) += count;
                *pair_counts.entry((*insert, *r)).or_insert(0u64) += count;
            }
        }
    }

    // Count first char of every pair
    let mut char_counts = pair_counts
        .iter()
        .fold(HashMap::new(), |mut hist, ((l, _r), count)| {
            *hist.entry(l).or_insert(0u64) += count;
            hist
        });

    // Explicitly add last char to count
    let last_char = s.chars().last().unwrap();
    *char_counts.entry(&last_char).or_insert(0u64) += 1;

    let min = char_counts.values().min().unwrap();
    let max = char_counts.values().max().unwrap();

    max - min
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &Input) -> u64 {
    apply_rules(&input.start, &input.rules, 10)
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &Input) -> u64 {
    apply_rules(&input.start, &input.rules, 40)
}

#[cfg(test)]
mod test {
    use super::*;

    pub fn sample1() -> String {
        String::from("NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C")
    }

    #[test]
    pub fn test_input_generator_sample1() {
        let input = input_generator(&sample1().as_str());

        assert_eq!(input.start, "NNCB");
        assert_eq!(input.rules.get(&('C', 'B')).unwrap(), &'H');
    }

    #[test]
    pub fn test_apply_rules_once() {
        let input = input_generator(&sample1().as_str());

        let new_word = apply_rules(&"NNCB".to_string(), &input.rules, 1);

        // NCNBCHB
        assert_eq!(new_word, 1);
    }

    #[test]
    pub fn test_apply_rules_twice() {
        let input = input_generator(&sample1().as_str());

        let new_word = apply_rules(&"NNCB".to_string(), &input.rules, 2);

        // NBCCNBBBCBHCB
        assert_eq!(new_word, 5);
    }

    #[test]
    pub fn test_apply_rules_three_times() {
        let input = input_generator(&sample1().as_str());

        let new_word = apply_rules(&"NNCB".to_string(), &input.rules, 3);

        // NBBBCNCCNBBNBNBBCHBHHBCHB
        assert_eq!(new_word, 7);
    }

    #[test]
    pub fn test_solve_part1() {
        let input = input_generator(&sample1().as_str());

        let result = solve_part1(&input);

        assert_eq!(result, 1588);
    }

    #[test]
    pub fn test_solve_part2() {
        let input = input_generator(&sample1().as_str());

        let result = solve_part2(&input);

        assert_eq!(result, 2188189693529);
    }
}
