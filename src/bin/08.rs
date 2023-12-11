use std::collections::HashMap;
use num_integer::lcm;
advent_of_code::solution!(8);

struct Node {
    left: String,
    right: String
}

fn parse_node(str: &str) -> (String, Node) {
    let mut fields = str.split(" = ");
    let name = fields.next().unwrap();

    let mut directions = fields.next().unwrap().split(", ");
    let left = directions.next().unwrap().trim_start_matches("(");
    let right = directions.next().unwrap().trim_end_matches(")");

    (name.to_string(), Node { left: left.to_string(), right: right.to_string() })
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map: HashMap<String, Node> = HashMap::new();

    let mut lines = input.lines();
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    let mut raw_nodes = lines.skip(1);

    for raw_node in raw_nodes {
        let (name, directions) = parse_node(raw_node);
        map.insert(name, directions);
    }

    let mut current_node = "AAA";
    let mut steps = 0;
    let mut index = 0;
    loop {
        let instruction = *instructions.get(index).unwrap();
        let node = map.get(current_node).unwrap();
        current_node = if instruction == 'L' { node.left.as_str() } else { node.right.as_str() };

        if index == instructions.len()-1 { index = 0 } else { index += 1 }
        steps += 1;
        if current_node.eq("ZZZ") { break; }
    }

    Some(steps)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map: HashMap<String, Node> = HashMap::new();

    let mut lines = input.lines();
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    let mut raw_nodes = lines.skip(1);

    for raw_node in raw_nodes {
        let (name, directions) = parse_node(raw_node);
        map.insert(name, directions);
    }

    let mut current_nodes: Vec<String> = map
        .keys().cloned().collect::<Vec<String>>()
        .into_iter().filter(|x| x.ends_with("A")).collect();

    let mut steps: Vec<u32> = Vec::new();
    let mut answer = 0;
    let mut index = 0;

    for node in current_nodes {
        let mut _steps = 0;
        let mut current_node = node;

        loop {
            let instruction = *instructions.get(index).unwrap();
            let _node = map.get(&current_node).unwrap().clone();

            current_node = if instruction == 'L' { _node.left.to_string() } else { _node.right.to_string() };

            if index == instructions.len() - 1 { index = 0 } else { index += 1 }
            _steps += 1;

            if current_node.ends_with("Z") {
                print!("");
                break;
            }
        }

        steps.push(_steps);
    }

    answer = steps.iter().cloned().fold(steps[0] as u64, |acc, x| lcm(acc, x as u64));

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
