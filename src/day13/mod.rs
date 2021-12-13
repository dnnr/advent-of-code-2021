use std::str::FromStr;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone)]
pub enum Fold {
    X(usize),
    Y(usize),
}

impl FromStr for Fold {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (statement, value_str) = s.split_once('=').unwrap();
        let value = value_str.parse::<usize>().unwrap();
        match statement {
            "fold along x" => Ok(Fold::X(value)),
            "fold along y" => Ok(Fold::Y(value)),
            _ => panic!("Unknown fold instruction"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Coords {
    x: usize,
    y: usize,
}

impl FromStr for Coords {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .split(',')
            .map(|x| (x.parse::<usize>().unwrap()))
            .collect::<Vec<usize>>();

        Ok(Coords { x: nums[0], y: nums[1] })
    }
}

pub struct Input {
    dots: Vec<Coords>,
    folds: Vec<Fold>,
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Input {
    let (coords_str, folds_str) = input.split_once("\n\n").unwrap();

    let dots = coords_str
        .lines()
        .map(|x| Coords::from_str(x).unwrap())
        .collect::<Vec<Coords>>();

    let folds = folds_str
        .lines()
        .map(|x| Fold::from_str(x).unwrap())
        .collect::<Vec<Fold>>();

    Input { dots, folds }
}

pub fn fold(dots: &[Coords], folds: &[Fold]) -> HashSet::<Coords> {
    let mut folded_dots = Vec::from(dots);

    for fold in folds {
        match fold {
            Fold::X(fold_here) => {
                for coords in &mut folded_dots {
                    if coords.x > *fold_here {
                        coords.x -= 2 * (coords.x - fold_here);
                    }
                }
            },
            Fold::Y(fold_here) => {
                for coords in &mut folded_dots {
                    if coords.y > *fold_here {
                        coords.y -= 2 * (coords.y - fold_here);
                    }
                }
            },
        }
    }

    HashSet::<Coords>::from_iter(folded_dots)
}

pub fn format_dots(coords: &HashSet<Coords>) -> String {
    let max_x: usize = coords.iter().map(|c| c.x).max().unwrap();
    let max_y: usize = coords.iter().map(|c| c.y).max().unwrap();

    let mut out = String::new();
    for y in 0..max_y+1 {
        for x in 0..max_x+1 {
            if coords.contains(&Coords { x, y }) {
                out.push('#');
            } else {
                out.push('.');
            }
        }
        out.push('\n');
    }

    out
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let folded_dots = fold(&input.dots, &input.folds[..1]);
    folded_dots.len()
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &Input) -> String {
    let folded_dots = fold(&input.dots, &input.folds);
    format_dots(&folded_dots)
}

#[cfg(test)]
mod test {
    use super::*;

    pub fn sample1() -> String {
        String::from("6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5")
    }

    #[test]
    pub fn test_input_generator_sample1() {
        let input = input_generator(&sample1().as_str());

        assert_eq!(input.dots[0].x, 6);
        assert_eq!(input.dots[0].y, 10);

        assert_eq!(input.folds[0], Fold::Y(7));
        assert_eq!(input.folds[1], Fold::X(5));
    }

    #[test]
    pub fn test_format_coords() {
        let input = input_generator(&sample1().as_str());
        let dots = HashSet::from_iter(input.dots.clone());

        let printed = format_dots(&dots);

        assert_eq!(printed, "...#..#..#.
....#......
...........
#..........
...#....#.#
...........
...........
...........
...........
...........
.#....#.##.
....#......
......#...#
#..........
#.#........
");
    }

    #[test]
    pub fn test_solve_part1() {
        let input = input_generator(&sample1().as_str());

        let dots = solve_part1(&input);

        assert_eq!(dots, 17);
    }

    #[test]
    pub fn test_solve_part2() {
        let input = input_generator(&sample1().as_str());

        let printed_dots = solve_part2(&input);

        println!("{}", printed_dots);
        assert_eq!(printed_dots, "#####
#...#
#...#
#...#
#####
");
    }
}
