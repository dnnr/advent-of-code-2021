use std::str::FromStr;
use std::string::ParseError;
use std::collections::BTreeSet;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Digit {
    seg_chars: BTreeSet<char>,
    seg_bools: Vec<bool>,
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    e: bool,
    f: bool,
    g: bool,
}

impl FromStr for Digit {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Digit {
            seg_chars: s.chars().collect(),
            seg_bools: (0..7).map(|d| s.contains((b'a' + d) as char)).collect::<Vec<bool>>(),
            a: s.contains('a'),
            b: s.contains('b'),
            c: s.contains('c'),
            d: s.contains('d'),
            e: s.contains('e'),
            f: s.contains('f'),
            g: s.contains('g'),
        })
    }
}

#[derive(Debug)]
pub struct Entry {
    // uniques: [Digit; 10],
    // output: [Digit; 4],
    uniques: Vec<Digit>,
    output: Vec<Digit>,
}

impl FromStr for Entry {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Entry, Self::Err> {
        let lr = s.split('|')
            .map(|lr| lr.split(' ')
                .filter(|s| !(*s).is_empty())
                .map(|d| Digit::from_str(d).unwrap())
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
            .filter(|d| [2, 3, 4, 7].contains(&d.seg_chars.len()))
            .count())
        .sum()
}

// fn check_if_valid_solution(entry: &Entry, mapping: &[u8; 7]) -> bool {
    // let mut mapped_uniques: Vec<u8> = Vec::new();

    // entry.uniques
        // .iter()
        // .map(|d| d.seg_chars
            // .iter()
            // .map(|c| mapping[*c as usize - 'a'as usize])
            // );
    // false
// }

// fn find_and_take<F: for<'r> std::ops::FnMut<(&'r &std::collections::BTreeSet<char>,)>>(set: BTreeSet<BTreeSet<char>>, condition: F) -> bool {
fn find_and_take<F>(set: &mut BTreeSet<BTreeSet<char>>, condition: F) -> BTreeSet<char> where F: Fn(&&BTreeSet<char>) -> bool {
    /*
    let mut iter = set.iter().filter(condition);
    let found = iter.next().unwrap();
    assert!(iter.next().is_none());
    let found2 = found.clone();
    set.remove(found);
    // found.clone()
    found2
    */

    let mut found: Option<BTreeSet<char>> = None;
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

type Digit2 = BTreeSet<char>;

fn determine_codebook(uniques: &[Digit]) -> HashMap<Digit2, usize> {
    let mut uniques_set = uniques.iter().map(|x| BTreeSet::from(x.seg_chars.clone())).collect::<BTreeSet<_>>();
    let mut num_to_charset: HashMap<usize, BTreeSet<char>> = HashMap::new();

    println!("uniques: {:?}", uniques_set);

    // 1, 4, 7, 8 are easy as per the number of segments
    num_to_charset.insert(1, find_and_take(&mut uniques_set, |x| x.len() == 2));
    num_to_charset.insert(4, find_and_take(&mut uniques_set, |x| x.len() == 4));
    num_to_charset.insert(7, find_and_take(&mut uniques_set, |x| x.len() == 3));
    num_to_charset.insert(8, find_and_take(&mut uniques_set, |x| x.len() == 7));

    // 9 is the only one with 6 segments that also contains 1 and 4
    num_to_charset.insert(9, find_and_take(&mut uniques_set, |x|
            x.len() == 6
            && x.is_superset(&num_to_charset.get(&1).unwrap())
            && x.is_superset(&num_to_charset.get(&4).unwrap())
            ));

    // Now 0 is the only one with 6 segments that contains 1
    num_to_charset.insert(0, find_and_take(&mut uniques_set, |x|
            x.len() == 6
            && x.is_superset(&num_to_charset.get(&1).unwrap())
            ));

    // 3 is the only one with 5 segments that contains 1
    num_to_charset.insert(3, find_and_take(&mut uniques_set, |x|
            x.len() == 5
            && x.is_superset(&num_to_charset.get(&1).unwrap())));

    // 6 is the only remaining one with 6 segments
    num_to_charset.insert(6, find_and_take(&mut uniques_set, |x| x.len() == 6));

    // 5 and 2 both have 5 segments, but only 5 is a subset of 6
    num_to_charset.insert(5, find_and_take(&mut uniques_set, |x|
            x.len() == 5
            && x.is_subset(&num_to_charset.get(&6).unwrap())));

    // 2 is the only one remaining
    num_to_charset.insert(2, find_and_take(&mut uniques_set, |x| x.len() == 5));

    // Sanity checks
    assert_eq!(num_to_charset.len(), 10);
    assert!(uniques_set.is_empty());

    // Reverse the map map to get the desired codebook
    num_to_charset
        .iter()
        .map(|(num, charset)| (charset.clone(), num.clone()))
        .collect::<HashMap<Digit2, usize>>()
}

fn solve_entry(entry: &Entry) -> Vec<usize> {
    let reverse_codebook = determine_codebook(&entry.uniques);

    // println!("codebook: {:?}", codebook);
    // println!("reverse codebook: {:?}", reverse_codebook);

    let output_sets = entry.output.iter().map(|x| BTreeSet::from(x.seg_chars.clone())).collect::<Vec<_>>();
    // println!("output_sets: {:?}", output_sets);

    let output_decoded =output_sets.iter().map(|x| *reverse_codebook.get(&x).expect(&format!("No reverse codebook entry for {:?}", x))).collect::<Vec<usize>>();

    println!("output: {:?}, sum: {}", entry.output, output_decoded.iter().sum::<usize>());
    println!(" ");

    output_decoded
}

fn output_vec_to_number(digits: Vec<usize>) -> usize {
    digits.iter().fold(0, |acc, x| acc * 10 + x)
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

        println!("{:?}", entries[0].output);

        assert_eq!(entries[0].uniques[0].seg_chars, BTreeSet::from_iter(['b', 'e']));
        assert_eq!(entries[0].uniques[0].seg_bools, vec![false, true, false, false, true, false, false]);
        assert!(entries[0].uniques[0].b);
        assert!(entries[0].uniques[0].e);
        assert_eq!(entries[1].output[3].seg_chars, BTreeSet::from_iter(['g', 'c']));
        assert!(entries[1].output[3].g);
        assert!(entries[1].output[3].c);
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
