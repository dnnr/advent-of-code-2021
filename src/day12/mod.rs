use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeSet;

#[derive(Debug)]
pub struct Cave {
    is_large: bool,
    adjacents: HashSet<String>,
}

type Map = HashMap<String, Cave>;

fn insert_or_get(map: &mut Map, id: &str) {
    if !map.contains_key(id) {
        let is_large = id.chars().next().unwrap().is_uppercase();
        map.insert(id.to_owned(), Cave { is_large, adjacents: HashSet::new() });
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> HashMap<String, Cave> {
    let mut map: Map = Map::new();
    for line in input.lines() {
        let (left_id, right_id) = line.split_once('-').unwrap();

        insert_or_get(&mut map, left_id);
        insert_or_get(&mut map, right_id);

        map.get_mut(left_id).unwrap().adjacents.insert(right_id.to_owned());
        map.get_mut(right_id).unwrap().adjacents.insert(left_id.to_owned());
    }
    map
}

fn find_paths(map: &Map, cave_allowed_twice: Option<&str>) -> Vec::<Vec::<String>> {
    let mut known_paths: Vec::<Vec::<String>> = Vec::new();
    let mut paths_to_continue = vec![vec!["start".to_owned()]];

    while !paths_to_continue.is_empty() {
        // Take next path
        let path =paths_to_continue.pop().unwrap();

        let current_cave_id = path.last().unwrap();
        if current_cave_id == "end" {
            known_paths.push(path);
        } else {
            // Extend path to all adjacent caves
            for next_cave_id in &map.get(current_cave_id).unwrap().adjacents {
                let next_cave = &map.get(next_cave_id).unwrap();

                let max_previous_appearances =
                    if matches!(&cave_allowed_twice, Some(x) if next_cave_id == x)
                        && next_cave_id != "start" {
                        1
                    } else {
                        0
                    };

                if next_cave.is_large
                   || path.iter().filter(|x| *x == next_cave_id).count() <= max_previous_appearances {
                    let mut new_path = path.clone();
                    new_path.push(next_cave_id.clone());
                    paths_to_continue.push(new_path);
                }
            }
        }

    }

    known_paths
}

#[aoc(day12, part1)]
pub fn solve_part1(map: &Map) -> usize {
    let known_paths = find_paths(map, None);
    known_paths.len()
}


#[aoc(day12, part2)]
pub fn solve_part2(map: &Map) -> usize {
    let mut known_paths = BTreeSet::from_iter(find_paths(map, None));

    // Collect more paths with one of each small cave allowed twice:
    for (id, cave) in map {
        if !cave.is_large {
            let more_paths = find_paths(map, Some(id));
            known_paths.append(&mut BTreeSet::from_iter(more_paths));
        }
    }

    known_paths.len()
}


#[cfg(test)]
mod test {
    use super::*;

    pub fn sample1() -> String {
        String::from("start-A
start-b
A-c
A-b
b-d
A-end
b-end")
    }

    pub fn sample2() -> String {
        String::from("dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc")
    }

    pub fn sample3() -> String {
        String::from("fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW")
    }

    #[test]
    pub fn test_input_generator_sample1() {
        let map = input_generator(&sample1().as_str());

        assert_eq!(map.get("start").unwrap().adjacents,
            HashSet::from_iter(["A", "b"].map(String::from)));
        assert_eq!(map.get("A").unwrap().adjacents,
            HashSet::from_iter(["start", "end", "b", "c"].map(String::from)));
        assert!(map.get("A").unwrap().is_large);
        assert!(!map.get("end").unwrap().is_large);
    }

    #[test]
    pub fn test_input_generator_sample2() {
        let map = input_generator(&sample2().as_str());

        assert_eq!(map.get("start").unwrap().adjacents,
            HashSet::from_iter(["HN", "kj", "dc"].map(String::from)));
        assert_eq!(map.get("HN").unwrap().adjacents,
            HashSet::from_iter(["start", "dc", "end", "kj"].map(String::from)));
        assert!(map.get("LN").unwrap().is_large);
        assert!(!map.get("dc").unwrap().is_large);
    }

    #[test]
    pub fn test_solve_part1_sample1() {
        let map = input_generator(&sample1().as_str());

        let paths = solve_part1(&map);

        assert_eq!(paths, 10);
    }

    #[test]
    pub fn test_solve_part1_sample2() {
        let map = input_generator(&sample2().as_str());

        let paths = solve_part1(&map);

        assert_eq!(paths, 19);
    }

    #[test]
    pub fn test_solve_part1_sample3() {
        let map = input_generator(&sample3().as_str());

        let paths = solve_part1(&map);

        assert_eq!(paths, 226);
    }

    #[test]
    pub fn test_solve_part2_sample1() {
        let map = input_generator(&sample1().as_str());

        let paths = solve_part2(&map);

        assert_eq!(paths, 36);
    }

    #[test]
    pub fn test_solve_part2_sample2() {
        let map = input_generator(&sample2().as_str());

        let paths = solve_part2(&map);

        assert_eq!(paths, 103);
    }

    #[test]
    pub fn test_solve_part2_sample3() {
        let map = input_generator(&sample3().as_str());

        let paths = solve_part2(&map);

        assert_eq!(paths, 3509);
    }
}
