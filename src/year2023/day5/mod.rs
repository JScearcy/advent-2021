use std::{collections::HashMap, str::Split};

use crate::handler::{AdventSolution, DayHandler, SolveError};

#[derive(Debug)]
pub enum Day5Error {}

impl Into<SolveError> for Day5Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day5Error: {:?}", self))
    }
}

#[derive(Clone, Debug)]
struct RowMap {
    destination_start: usize,
    source_start: usize,
    range: usize,
}

impl RowMap {
    fn from_row(row: &str) -> Self {
        let vals: Vec<usize> = row
            .split(" ")
            .map(|val| usize::from_str_radix(val, 10).unwrap())
            .collect();
        Self {
            destination_start: vals[0],
            source_start: vals[1],
            range: vals[2],
        }
    }

    fn map_val(&self, seed_val: usize) -> Option<usize> {
        // println!("seed_val: {}, destination_start: {}, source_start: {}, range: {}", seed_val, self.destination_start, self.source_start, self.range);
        if seed_val >= self.source_start && seed_val <= self.source_start + self.range {
            let out_diff = seed_val - self.source_start;
            // println!("seed val out: {}", self.destination_start + out_diff);
            Some(self.destination_start + out_diff)
        } else {
            None
        }
    }

    fn map_range(&self, seed_ranges: &Vec<(usize, usize)>) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
        let mut in_range_seeds: Vec<(usize, usize)> = vec![];
        let mut out_range_seeds: Vec<(usize, usize)> = vec![];
        for seed_range in seed_ranges {
            let source_end = self.source_start + self.range;
            let (seed_start, seed_end) = *seed_range;
            let before_range = (seed_start, seed_end.min(self.source_start));
            let in_range = (seed_start.max(self.source_start), seed_end.min(source_end));
            let after_range = (seed_start.max(source_end), seed_end);

            if before_range.1 > before_range.0 {
                out_range_seeds.push(before_range);
            }
            if in_range.1 > in_range.0 {
                let start = in_range.0 - self.source_start + self.destination_start;
                let end = in_range.1 - self.source_start + self.destination_start;
                in_range_seeds.push((start, end));
            }
            if after_range.1 > after_range.0 {
                out_range_seeds.push(after_range);
            }
        }

        (in_range_seeds, out_range_seeds)
    }
}

static MAP_ORDER: [&str; 7] = [
    "seed-to-soil",
    "soil-to-fertilizer",
    "fertilizer-to-water",
    "water-to-light",
    "light-to-temperature",
    "temperature-to-humidity",
    "humidity-to-location",
];

fn parse(input_lines: Split<&str>) -> (Vec<usize>, HashMap<String, Vec<RowMap>>) {
    let mut seeds: Vec<usize> = vec![];
    let mut map_map: HashMap<String, Vec<RowMap>> = HashMap::new();
    let mut map_key: Option<String> = None;
    for line in input_lines {
        if line.len() == 0 {
            map_key = None;
            continue;
        } else if line.contains("seeds") {
            let seed_nums_str: String = line.split(": ").skip(1).collect();
            let mut seed_nums: Vec<usize> = seed_nums_str
                .split(" ")
                .map(|val| usize::from_str_radix(val, 10).unwrap())
                .collect();
            seeds.append(&mut seed_nums);
        } else if line.contains("map") {
            let key: String = line.split(" ").take(1).collect();
            map_map.insert(key.clone(), vec![]);
            map_key = Some(key);
        } else if map_key.is_some() {
            let key = map_key.clone().unwrap();
            let row_map = RowMap::from_row(line);
            let mut val = map_map.get(&key).unwrap().clone();
            val.push(row_map);
            map_map.insert(key, val);
        }
    }

    return (seeds, map_map);
}

fn get_lowest(seeds: &Vec<usize>, map_map: &HashMap<String, Vec<RowMap>>) -> usize {
    let mut seed_to_locations: Vec<usize> = vec![];
    for seed in seeds {
        let mut running_value: Option<usize> = Some(*seed);
        for key in MAP_ORDER {
            let maps = map_map.get(key).unwrap();
            let map_val: Option<usize> = maps.iter().fold(None, |running, curr| {
                if running.is_none() {
                    curr.map_val(running_value.unwrap())
                } else {
                    running
                }
            });
            running_value = map_val.or(running_value);
        }
        seed_to_locations.push(running_value.unwrap());
    }

    seed_to_locations
        .iter()
        .fold(seed_to_locations[0], |running, val| running.min(*val))
}

pub struct Day5Handler {}
impl<'a> Day5Handler {
    pub fn new() -> DayHandler<'a, &'a str> {
        DayHandler::new(Day5Handler {})
    }
    pub fn solve_1(&self, input_lines: Split<&str>) -> Result<String, Day5Error> {
        let (seeds, map_map) = parse(input_lines);
        let min_val = get_lowest(&seeds, &map_map);
        Ok(min_val.to_string())
    }

    pub fn solve_2(&self, input_lines: Split<&str>) -> Result<String, Day5Error> {
        let (seed_pairs, map_map) = parse(input_lines);
        let mut min_value: Option<usize> = None;
        for seed_pair in seed_pairs.chunks(2) {
            let mut seed_pair_list = vec![(seed_pair[0], seed_pair[0]+seed_pair[1])];
            for key in MAP_ORDER {
                let mut in_range_seeds: Vec<(usize, usize)> = vec![];
                let mut out_range_seeds: Vec<(usize, usize)> = seed_pair_list.clone();
                let maps = map_map.get(key).unwrap();
                for map in maps {
                    let (mut in_range_seed_result, out_range_seed_result) = map.map_range(&seed_pair_list);
                    out_range_seeds = out_range_seed_result;
                    in_range_seeds.append(&mut in_range_seed_result);
                }
                in_range_seeds.append(&mut out_range_seeds);
                seed_pair_list = in_range_seeds.iter().filter(|val| val.0 != 0).map(|val| *val).collect();
            }
            min_value = seed_pair_list.iter().fold(min_value, |new_val, curr_val| {
                if new_val.is_none() {
                    Some(curr_val.0)
                } else {
                    let curr_min = new_val.unwrap();
                    Some(curr_val.0.min(curr_min))
                }
            });
        }

        Ok(min_value.unwrap().to_string())
    }
}

impl<'a> AdventSolution<&str> for Day5Handler {
    fn get_day(&self) -> String {
        String::from("5")
    }
    fn solve(&self, problem: &str, input: &str) -> Result<String, SolveError> {
        let input_lines = input.split("\n");
        let result = if problem == "1" {
            self.solve_1(input_lines)
        } else {
            self.solve_2(input_lines)
        };

        result.map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::Day5Handler;
    use crate::handler::AdventSolution;

    fn get_input<'a>() -> &'a str {
        "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
    }

    #[test]
    fn get_day() {
        let handler = Day5Handler::new();
        assert!(&handler.get_day() == "5");
    }

    async fn solution(sol: &str) -> String {
        let handler = Day5Handler::new();
        handler.solve(sol, get_input()).unwrap()
    }

    #[tokio::test]
    async fn solution_1() {
        let solution = solution("1").await;
        assert!(solution == String::from("35"));
    }

    #[tokio::test]
    async fn solution_2() {
        let solution = solution("2").await;
        assert!(solution == String::from("46"));
    }
}
