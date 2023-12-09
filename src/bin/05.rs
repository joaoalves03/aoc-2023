use std::fs;
advent_of_code::solution!(5);

struct RangeSet {
    dest_start: u64,
    src_start: u64,
    length: u64
}

static MAP_NAMES: [&str; 7] = [
    "seed-to-soil map:",
    "soil-to-fertilizer map:",
    "fertilizer-to-water map:",
    "water-to-light map:",
    "light-to-temperature map:",
    "temperature-to-humidity map:",
    "humidity-to-location map:"
];

fn parse_seeds(line: &str) -> Vec<u64> {
    let fields = line.split_whitespace();
    let mut seeds: Vec<u64> = Vec::new();

    for (i, field) in fields.enumerate() {
        if i == 0 { continue }

        seeds.push(field.parse::<u64>().unwrap())
    }

    seeds
}

fn parse_seed_ranges(line: &str) -> Vec<(u64,u64)> {
    let fields: Vec<&str> = line.split_whitespace().skip(1).collect();

    let mut seed_ranges: Vec<(u64,u64)> = Vec::new();

    for chunk in fields.chunks(2) {
        let start = chunk.first().unwrap().parse::<u64>().unwrap();
        let range = chunk.last().unwrap().parse::<u64>().unwrap();

        seed_ranges.push((
            start,
            start+range-1
        ))
    }

    seed_ranges
}

fn parse_map(lines: &Vec<&str>, filter: &str) -> Vec<RangeSet> {
    let mut lines_iter = lines.iter();
    let mut ranges: Vec<RangeSet> = Vec::new();

    let filter_index = lines_iter.position(|&x| x == filter).unwrap();
    let mut usable_lines: Vec<&str> = Vec::new();

    for i in filter_index+1..lines.len() {
        let line = lines.get(i).unwrap();

        if line.is_empty() {
            break;
        }

        usable_lines.push(line);
    }

    for line in usable_lines {
        let fields: Vec<&str> = line.split_whitespace().collect();
        let range_set = RangeSet {
            dest_start: fields.get(0).unwrap().parse:: <u64>().unwrap(),
            src_start: fields.get(1).unwrap().parse:: <u64>().unwrap(),
            length: fields.get(2).unwrap().parse:: <u64>().unwrap()
        };

        ranges.push(range_set)
    }

    ranges
}

fn map_value(mut value: u64, range_maps: &Vec<Vec<RangeSet>>) -> u64 {
    for range_map in range_maps {
        for range_set in range_map {
            if value >= range_set.src_start && value <= range_set.src_start + range_set.length {
                value = range_set.dest_start + (value - range_set.src_start);
                break;
            }
        }
    }

    value
}

fn map_seed_range(seed_range: (u64,u64), range_maps: &Vec<Vec<RangeSet>>) -> Vec<(u64, u64)>{
    let mut seed_ranges: Vec<(u64,u64)> = vec![seed_range];

    for range_map in range_maps {
        let mut new_seed_ranges: Vec<(u64,u64)> = Vec::new();

        for mut seed_range in seed_ranges {
            for range_set in range_map {
                let (mut start, mut end) = seed_range;

                if start < range_set.src_start {
                    if end < range_set.src_start {
                        continue;
                    }

                    //TODO: -1?
                    new_seed_ranges.push((start, range_set.src_start-1));
                    start = range_set.src_start;
                    seed_range = (start,end);
                }

                // TODO: am i supposed to do -1 here
                if end > range_set.src_start + range_set.length - 1 {
                    if start > range_set.src_start + range_set.length - 1 {
                        continue;
                    }

                    new_seed_ranges.push((range_set.src_start + range_set.length + 1, end));
                    end = range_set.src_start + range_set.length - 1;
                    seed_range = (start,end);
                }

                new_seed_ranges.push((
                    range_set.dest_start + (start - range_set.src_start),
                    range_set.dest_start + (end - range_set.src_start)
                ));

                print!("");
            }
        }

        seed_ranges = new_seed_ranges.clone();
    }


    seed_ranges
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let seeds: Vec<u64> = parse_seeds(lines.first().unwrap());
    let mut range_maps: Vec<Vec<RangeSet>> = Vec::new();
    let mut lowest_location_value: u64 = 0;

    for name in MAP_NAMES {
        range_maps.push(parse_map(&lines, name))
    }

    for seed in seeds {
        let mut mapped_value = map_value(seed, &range_maps);

        if lowest_location_value == 0 { lowest_location_value = mapped_value }

        if mapped_value < lowest_location_value {
            lowest_location_value = mapped_value;
        }
    }

    Some(lowest_location_value as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    //TODO
    let raw = fs::read_to_string("data/examples/05.txt").unwrap();

    let lines: Vec<&str> = raw.lines().collect();

    let seed_ranges = parse_seed_ranges(lines.first().unwrap());
    let mut range_maps: Vec<Vec<RangeSet>> = Vec::new();
    let mut lowest_location_value: u64 = 0;

    for name in MAP_NAMES {
        range_maps.push(parse_map(&lines, name))
    }

    for seed_range in seed_ranges {
        let mut mapped_seed_range = map_seed_range(seed_range, &range_maps);

        print!("");

        //println!("{} - {}", map_value(start, &range_maps), map_value(end, &range_maps));
    }

    Some(lowest_location_value as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
