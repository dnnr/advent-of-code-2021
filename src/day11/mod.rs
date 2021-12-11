#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>()
        )
        .collect::<Vec<Vec<u8>>>()
}

fn increment(matrix: &mut Vec<Vec<u8>>) {
    for row in matrix {
        for col in row {
            *col += 1;
        }
    }
}

fn increment_neighbors(matrix: &mut Vec<Vec<u8>>, row_idx: &usize, col_idx: &usize) {
    let neighbors = vec![
        ( 0,  1),  (1, 1),  (1,  0),
        ( 1, -1),           (0, -1),
        (-1, -1), (-1, 0), (-1,  1)
    ].iter()
        .map(|(r, c)|
            (*row_idx as i32 + r, *col_idx as i32 + c))
        .collect::<Vec<(i32, i32)>>();

    let rows_count = matrix.len();
    let cols_count = matrix[0].len();
    neighbors
        .iter()
        .filter(|(r, c)|
            r >= &0
            && c >= &0
            && r < &(rows_count as i32)
            && c < &(cols_count as i32))
        .for_each(|(r, c)|
            matrix[*r as usize][*c as usize] += 1
            );
}

fn find_over_nine(matrix: &[Vec<u8>]) -> Option<(usize, usize)> {
    for (row_idx, row) in matrix.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if *col > 9 {
                return Some((row_idx, col_idx));
            }
        }
    }

    None
}

fn step(matrix: &mut Vec<Vec<u8>>) -> usize {
    let mut flashed = [[false; 10]; 10];

    increment(matrix);

    while let Some(over_nine) = find_over_nine(matrix) {
        let (row_idx, col_idx) = over_nine;

        // Don't flash twice
        if !flashed[row_idx][col_idx] {
            increment_neighbors(matrix, &row_idx, &col_idx);
            flashed[row_idx][col_idx] = true;
        }

        matrix[row_idx][col_idx] = 0;
    }

    for (row_idx, row) in flashed.iter().enumerate() {
        for (col_idx, _) in row.iter().enumerate() {
            if flashed[row_idx][col_idx] {
                matrix[row_idx][col_idx] = 0;
            }
        }
    }

    flashed
        .iter()
        .map(|row| row
            .iter()
            .filter(|x| **x)
            .count()
        ).sum()
}

#[aoc(day11, part1)]
pub fn solve_part1(matrix_ro: &[Vec<u8>]) -> usize {
    let mut matrix = matrix_ro.to_owned();

    (0..100)
        .map(|_| step(&mut matrix))
        .sum()
}

#[aoc(day11, part2)]
pub fn solve_part2(matrix_ro: &[Vec<u8>]) -> usize {
    let mut matrix = matrix_ro.to_owned();

    for n in 1.. {
        if step(&mut matrix) == 100 {
            return n;
        }
    }

    // fail
    0
}

#[cfg(test)]
mod test {
    use super::*;

    pub fn sample_str() -> String {
        String::from("5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526")
    }

    #[test]
    pub fn test_input_generator() {
        let matrix = input_generator(&sample_str().as_str());

        assert_eq!(matrix[0][0], 5);
        assert_eq!(matrix[1][0], 2);
        assert_eq!(matrix[0][1], 4);
    }

    #[test]
    pub fn test_increment() {
        let mut matrix = input_generator("123\n456\n789");

        increment(&mut matrix);

        assert_eq!(matrix, vec![vec![2,3,4], vec![5,6,7], vec![8, 9, 10]]);
    }

    #[test]
    pub fn test_step() {
        let mut matrix = input_generator("11111
19991
19191
19991
11111");
        let expected_after_step1 = input_generator("34543
40004
50005
40004
34543");

        let expected_after_step2 = input_generator("45654
51115
61116
51115
45654");

        assert_eq!(step(&mut matrix), 9);

        assert_eq!(matrix, expected_after_step1);

        assert_eq!(step(&mut matrix), 0);

        assert_eq!(matrix, expected_after_step2);
    }

    #[test]
    pub fn test_solve_part1() {
        let matrix = input_generator(&sample_str().as_str());

        let flashcount = solve_part1(&matrix);

        assert_eq!(flashcount, 1656);
    }

    #[test]
    pub fn test_solve_part2() {
        let matrix = input_generator(&sample_str().as_str());

        let flashcount = solve_part2(&matrix);

        assert_eq!(flashcount, 195);
    }
}
