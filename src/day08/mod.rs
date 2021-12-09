use std::str::FromStr;
use std::string::ParseError;
use std::collections::BTreeSet;
use std::collections::HashMap;

type Digit = BTreeSet<char>;

#[derive(Debug)]
pub struct Entry {
    uniques: Vec<Digit>,
    output: Vec<Digit>,
}

impl FromStr for Entry {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Entry, Self::Err> {
        let lr = s.split('|')
            .map(|lr| lr.split(' ')
                .filter(|s| !(*s).is_empty())
                .map(|d| Digit::from_iter(d.chars()))
                .collect::<Vec<Digit>>()
            )
            .collect::<Vec<Vec<Digit>>>();

        Ok(Entry { uniques: lr[0].to_vec(), output: lr[1].to_vec() } )
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|x| Entry::from_str(x).unwrap())
        .collect::<Vec<Entry>>()
}

#[aoc(day8, part1)]
pub fn solve_part1(entries: &[Entry]) -> usize {
    entries
        .iter()
        .map(|e| e.output
            .iter()
            .filter(|d| [2, 3, 4, 7].contains(&d.len()))
            .count())
        .sum()
}

fn find_and_take<F>(set: &mut BTreeSet<Digit>, condition: F) -> Digit where F: Fn(&&Digit) -> bool {
    let mut found = None;
    set.retain(|k| {
        if condition(&k) {
            found = Some(k.clone());
            false
        } else {
            true
        }
    });

    found.unwrap()
}

fn determine_codebook(uniques: &[Digit]) -> HashMap<Digit, u8> {
    // Organize uniques in a set
    let mut uniques = BTreeSet::from_iter(uniques.iter().cloned());

    // Build mapping from numbers to sets of characters
    let mut num_to_charset: HashMap<u8, BTreeSet<char>> = HashMap::new();

    // 1, 4, 7, 8 are easy as per the number of segments
    num_to_charset.insert(1, find_and_take(&mut uniques, |x| x.len() == 2));
    num_to_charset.insert(4, find_and_take(&mut uniques, |x| x.len() == 4));
    num_to_charset.insert(7, find_and_take(&mut uniques, |x| x.len() == 3));
    num_to_charset.insert(8, find_and_take(&mut uniques, |x| x.len() == 7));

    // 9 is the only one with 6 segments that also contains 1 and 4
    num_to_charset.insert(9, find_and_take(&mut uniques, |x|
            x.len() == 6
            && x.is_superset(num_to_charset.get(&1).unwrap())
            && x.is_superset(num_to_charset.get(&4).unwrap())
            ));

    // Now 0 is the only one with 6 segments that contains 1
    num_to_charset.insert(0, find_and_take(&mut uniques, |x|
            x.len() == 6
            && x.is_superset(num_to_charset.get(&1).unwrap())
            ));

    // 3 is the only one with 5 segments that contains 1
    num_to_charset.insert(3, find_and_take(&mut uniques, |x|
            x.len() == 5
            && x.is_superset(num_to_charset.get(&1).unwrap())));

    // 6 is the only remaining one with 6 segments
    num_to_charset.insert(6, find_and_take(&mut uniques, |x| x.len() == 6));

    // 5 and 2 both have 5 segments, but only 5 is a subset of 6
    num_to_charset.insert(5, find_and_take(&mut uniques, |x|
            x.len() == 5
            && x.is_subset(num_to_charset.get(&6).unwrap())));

    // 2 is the only one remaining
    num_to_charset.insert(2, find_and_take(&mut uniques, |x| x.len() == 5));

    // Sanity checks
    assert_eq!(num_to_charset.len(), 10);
    assert!(uniques.is_empty());

    // Reverse the map map to get the desired codebook
    num_to_charset
        .iter()
        .map(|(num, charset)| (charset.clone(), *num))
        .collect::<HashMap<Digit, u8>>()
}

fn solve_entry(entry: &Entry) -> Vec<u8> {
    let codebook = determine_codebook(&entry.uniques);

    // Apply codebook to output digits
    entry.output
        .iter()
        .map(|x| *codebook.get(x).unwrap_or_else(|| panic!("No reverse codebook entry for {:?}", x)))
        .collect::<Vec<u8>>()
}

fn output_vec_to_number(digits: Vec<u8>) -> usize {
    // Interpret given digits as base 10 number
    digits.iter().fold(0, |acc, x| acc * 10 + *x as usize)
}

#[aoc(day8, part2)]
pub fn solve_part2(entries: &[Entry]) -> usize {
    entries
        .iter()
        .map(solve_entry)
        .map(output_vec_to_number)
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;

    pub fn sample_str() -> String {
        String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce")
    }

    #[test]
    pub fn test_input_generator() {
        let entries = input_generator(&sample_str().as_str());

        assert_eq!(entries[0].uniques[0], BTreeSet::from_iter(['b', 'e']));
        assert_eq!(entries[1].output[3], BTreeSet::from_iter(['g', 'c']));
    }

    #[test]
    pub fn test_solve_part1() {
        let entries = input_generator(&sample_str().as_str());
        let expected_count = 26;

        let count = solve_part1(&entries);

        assert_eq!(count, expected_count);
    }

    #[test]
    pub fn test_solve_entry() {
        let entry = Entry::from_str("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf").unwrap();

        let entry_solution = solve_entry(&entry);

        assert_eq!(entry_solution, vec![5, 3, 5, 3]);
    }

    #[test]
    pub fn test_solve_part2() {
        let entries = input_generator(&sample_str().as_str());
        let expected_sum = 61229;

        let sum = solve_part2(&entries);

        assert_eq!(sum, expected_sum);
    }
}
