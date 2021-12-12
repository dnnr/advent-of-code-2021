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
        let is_large = id.chars().nth(0).unwrap().is_uppercase();
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

#[aoc(day12, part1)]
pub fn solve_part1(map: &Map) -> usize {
    // Work queues do not really need to be sets, but they are...
    let mut known_paths: BTreeSet<Vec::<String>> = BTreeSet::new();
    let mut paths_to_continue: BTreeSet<Vec::<String>> = BTreeSet::new();

    paths_to_continue.insert(vec!["start".to_owned()]);

    while paths_to_continue.len() > 0 {
        // Take next path
        let path = paths_to_continue.iter().nth(0).unwrap().clone();
        paths_to_continue.take(&path);

        let current_cave_id = path.last().unwrap();
        if current_cave_id == "end" {
            let is_really_new = known_paths.insert(path);
            assert!(is_really_new);
        } else {
            // Extend path to all adjacent caves
            for next_cave_id in &map.get(current_cave_id).unwrap().adjacents {
                let next_cave = &map.get(next_cave_id).unwrap();

                // Make sure not to visit a small cave twice
                if next_cave.is_large || !path.contains(next_cave_id) {
                    let mut new_path = path.clone();
                    new_path.push(next_cave_id.clone());
                    let is_really_new = paths_to_continue.insert(new_path);
                    assert!(is_really_new);
                }
            }
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
}
