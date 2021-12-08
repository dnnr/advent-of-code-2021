use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug, Clone)]
pub struct Digit {
    segments: Vec<char>,
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
            segments: s.chars().collect(),
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

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|line| line.split('|')
            .map(|lr| lr.split(' ')
                .filter(|s| !(*s).is_empty())
                .map(|d| Digit::from_str(d).unwrap())
                .collect::<Vec<Digit>>()
            )
            .collect::<Vec<Vec<Digit>>>()
        )
        .map(|lr| Entry { uniques: lr[0].to_vec(), output: lr[1].to_vec() } )
        .collect::<Vec<Entry>>()
}

#[aoc(day8, part1)]
pub fn solve_part1(entries: &[Entry]) -> usize {
    entries
        .iter()
        .map(|e| e.output
            .iter()
            .filter(|d| [2, 4, 3, 7].contains(&d.segments.len()))
            .count())
        .sum()
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
        let entries = input_generator(sample_str().as_str());

        println!("{:?}", entries[0].output);

        assert_eq!(entries[0].uniques[0].segments, vec!['b', 'e']);
        assert!(entries[0].uniques[0].b);
        assert!(entries[0].uniques[0].e);
        assert_eq!(entries[1].output[3].segments, vec!['g', 'c']);
        assert!(entries[1].output[3].g);
        assert!(entries[1].output[3].c);
    }

    #[test]
    pub fn test_solve_part1() {
        let entries = input_generator(sample_str().as_str());
        let expected_count = 26;

        let count = solve_part1(&entries);

        assert_eq!(count, expected_count);
    }
}
