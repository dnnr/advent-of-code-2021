use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug)]
struct Outcome {
    winning_round: usize,
    score: u32,
}

#[derive(Debug)]
struct Board {
    rows: Vec<Vec<u32>>,
}

impl Board {
    pub fn find_value(&self, value: &u32) -> Option<(usize, usize)> {
        for (row_idx, row) in self.rows.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                if col == value {
                    return Some((row_idx, col_idx));
                }
            }
        }
        None
    }

    pub fn play(&self, draw: &[u32]) -> Option<Outcome> {
        let mut hits_per_row = [0; 5];
        let mut hits_per_column = [0; 5];
        let mut sum_of_hits = 0;

        for (round, current_draw) in draw.iter().enumerate() {
            if let Some((row_idx, col_idx)) = self.find_value(current_draw) {
                    sum_of_hits += *current_draw;
                    hits_per_row[row_idx] += 1;
                    hits_per_column[col_idx] += 1;

                    if hits_per_row.contains(&5) || hits_per_column.contains(&5) {
                        let score = current_draw * (self.rows.iter().map(|row| row.iter().sum::<u32>()).sum::<u32>() - sum_of_hits);
                        return Some(Outcome {
                            winning_round: round,
                            score
                        });
                    }

            }
        }
        None
    }
    }

impl FromStr for Board {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s
            .lines()
            .map(|row| row
                .split(' ')
                .filter(|x| !(*x).is_empty())
                .map(|x| x.parse::<u32>().unwrap())
                .collect()
                )
            .collect();
        Ok(Board { rows } )
    }
}

#[derive(Debug)]
pub struct Game {
    draw: Vec<u32>,
    boards: Vec<Board>,
}

impl Game {
    fn play_boards(&self) -> Vec<Outcome> {
        let mut outcomes = self.boards.iter()
            .map(|board| board.play(&self.draw))
            .flatten()
            .collect::<Vec<Outcome>>();

            outcomes.sort_by(|a, b| a.winning_round.partial_cmp(&b.winning_round).unwrap());

            outcomes
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Game {
    let groups = input.split("\n\n").collect::<Vec<&str>>();

    let draw = groups[0]
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let boards = groups[1..]
        .iter()
        .map(|x| Board::from_str(x).unwrap())
        .collect::<Vec<Board>>();

    Game { draw, boards }
}

#[aoc(day4, part1)]
pub fn solve_part1(game: &Game) -> u32 {
    let outcomes = game.play_boards();
    let winner = &outcomes[0];
    winner.score
}

#[aoc(day4, part2)]
pub fn solve_part2(game: &Game) -> u32 {
    let outcomes = game.play_boards();
    let loser = &outcomes.last().unwrap();
    loser.score
}

#[cfg(test)]
mod test {
    use super::*;

    pub fn sample_str() -> String {
        String::from("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
")
    }

    #[test]
    pub fn test_input_generator() {
        let game = input_generator(sample_str().as_str());

        assert_eq!(game.draw[2], 9);
        assert_eq!(game.boards[1].rows[1][2], 13);
    }

    #[test]
    pub fn test_solve_part1() {
        let game = input_generator(sample_str().as_str());
        let expected_score = 4512;

        let score = solve_part1(&game);

        assert_eq!(score, expected_score);
    }

    #[test]
    pub fn test_solve_part2() {
        let game = input_generator(sample_str().as_str());
        let expected_score = 1924;

        let score = solve_part2(&game);

        assert_eq!(score, expected_score);
    }
}
