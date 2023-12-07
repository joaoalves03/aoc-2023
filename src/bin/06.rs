advent_of_code::solution!(6);

struct Race {
    time: u64,
    record: u64
}

fn parse_races(input: &str, join: bool) -> Vec<Race> {
    let mut races: Vec<Race> = Vec::new();

    let lines: Vec<&str> = input.lines().collect();
    let time_fields: Vec<&str> = lines
        .first()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect();

    let record_fields: Vec<&str> = lines
        .last()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect();

    if join {
        races.push(Race {
            time: time_fields.join("").parse::<u64>().unwrap(),
            record: record_fields.join("").parse::<u64>().unwrap(),
        });

        return races
    }

    for i in 0..time_fields.len() {
        races.push(Race {
            time: time_fields.get(i).unwrap().parse::<u64>().unwrap(),
            record: record_fields.get(i).unwrap().parse::<u64>().unwrap(),
        })
    }

    races
}

pub fn part_one(input: &str) -> Option<u32> {
    let races = parse_races(input, false);
    let mut total: u32 = 0;

    for race in races {
        let mut n_ways_to_win: u32 = 0;

        for i in 1..race.time-1 {
            if i*(race.time-i) > race.record {
                n_ways_to_win += 1;
            }
        }

        if total == 0 { total = n_ways_to_win }
        else { total *= n_ways_to_win }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let races = parse_races(input,true);
    let race = races.first().unwrap();

    let mut margin = 0;

    for i in 1..race.time-1 {
        let x = i*(race.time-i);

        if x > race.record {
            margin = i;
            break;
        }
    }

    Some((race.time - margin * 2 + 1) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
