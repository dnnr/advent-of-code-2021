use std::collections::HashSet;
use priority_queue::DoublePriorityQueue;

pub struct Input {
    map: Vec<Vec<u8>>,
}

impl Input {
    fn get_risk_to(&self, index: usize) -> u8 {
        let (x, y) = self.index_to_xy(index);
        self.map[y][x]
    }

    fn index_to_xy(&self, index: usize) -> (usize, usize) {
        let edge_len = self.map.len();
        let y = (index - index % edge_len) / edge_len;
        let x = index % edge_len;
        (x, y)
    }

    fn xy_to_index(&self, x: usize, y: usize) -> usize {
        let edge_len = self.map.len();
        y * edge_len + x
    }

    fn neighbors(&self, index: usize) -> HashSet<usize> {
        let (x, y) = self.index_to_xy(index);
        let edge_len = self.map.len();

        let mut neighbors = HashSet::new();
        if y > 0 {
            neighbors.insert(self.xy_to_index(x, y - 1));
        }
        if x > 0 {
            neighbors.insert(self.xy_to_index(x - 1, y));
        }
        if y < edge_len {
            neighbors.insert(self.xy_to_index(x, y + 1));
        }
        if x < edge_len {
            neighbors.insert(self.xy_to_index(x + 1, y));
        }

        neighbors
    }
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Input {
    let map = input
        .lines()
        .map(|line| line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>()
            )
        .collect::<Vec<Vec<u8>>>();

    Input { map }
}

fn quintuple_input(input: &Input) -> Input {
    let tile_edge_len = input.map.len();
    let new_edge_len = tile_edge_len * 5;
    let mut new_map = vec![vec![0u8; new_edge_len]; new_edge_len];

    for tile_y in 0..5 {
        for tile_x in 0..5 {
            for (y, line) in input.map.iter().enumerate() {
                for (x, value) in line.iter().enumerate() {
                    let tiled_x = x + tile_x * tile_edge_len;
                    let tiled_y = y + tile_y * tile_edge_len;
                    let new_value = ((*value as usize + tile_y + tile_x - 1) % 9) as u8 + 1;
                    new_map[tiled_y][tiled_x] = new_value;
                }
            }
        }
    }

    Input { map: new_map }
}


fn dijkstra(input: &Input) -> usize {
    let map_edge_len = input.map.len();
    let num_nodes = map_edge_len.pow(2);
    let mut dist = vec![usize::MAX; num_nodes];
    let mut prev: Vec<Option<usize>> = vec![None; num_nodes];

    let start = 0;
    let target = num_nodes - 1;
    dist[start] = 0;

    let mut queue = DoublePriorityQueue::<usize, usize>::from_iter(
        (0..map_edge_len.pow(2)).map(|v| (v, dist[v])));

    while !queue.is_empty() {
        let u = *queue.peek_min().unwrap().0;
        queue.remove(&u);

        if u == target {
            return dist[u];
        }

        for v in input.neighbors(u) {
            if queue.get(&v).is_some() {
                let alt = dist[u] + input.get_risk_to(v) as usize;
                if alt < dist[v] {
                    dist[v] = alt;
                    prev[v] = Some(u);
                    queue.change_priority(&v, alt);
                }
            }
        }
    }

    usize::MAX
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &Input) -> usize {
    dijkstra(input)
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &Input) -> usize {
    dijkstra(&quintuple_input(input))
}

#[cfg(test)]
mod test {
    use super::*;

    pub fn sample1() -> String {
        String::from("1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581")
    }

    #[test]
    pub fn test_input_generator_sample1() {
        let input = input_generator(&sample1().as_str());

        assert_eq!(input.map[1][2], 8);
    }

    #[test]
    pub fn test_calc_neighbors() {
        let input = input_generator(&sample1().as_str());

        assert_eq!(input.neighbors(0), HashSet::from_iter([1, 10]));
        assert_eq!(input.neighbors(11), HashSet::from_iter([10, 12, 1, 21]));
    }


    #[test]
    pub fn test_quintuple_input() {
        let input = input_generator("12\n34");

        let quintupled_input = quintuple_input(&input);

        assert_eq!(quintupled_input.map[0], vec![1, 2, 2, 3, 3, 4, 4, 5, 5, 6]);
        assert_eq!(quintupled_input.map[1], vec![3, 4, 4, 5, 5, 6, 6, 7, 7, 8]);
    }

    #[test]
    pub fn test_solve_part1() {
        let input = input_generator(&sample1().as_str());

        let total_cost = solve_part1(&input);

        assert_eq!(total_cost, 40);
    }

    #[test]
    pub fn test_solve_part2() {
        let input = input_generator(&sample1().as_str());

        let total_cost = solve_part2(&input);

        assert_eq!(total_cost, 315);
    }

}

