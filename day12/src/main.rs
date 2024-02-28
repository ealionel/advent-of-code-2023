use std::{
    collections::{HashMap, HashSet},
    fs,
};

type Record<'a> = (&'a str, Vec<i32>);

fn count_contiguous_damaged_springs(springs: &str) -> Vec<i32> {
    let mut groups = Vec::new();

    let mut is_in_group = false;
    let mut count = 0;

    for spring in springs.chars() {
        if spring == '.' && is_in_group {
            groups.push(count);
            count = 0;
            is_in_group = false;
        }

        if spring == '#' {
            is_in_group = true;
            count += 1;
        }
    }

    if is_in_group {
        groups.push(count);
    }

    groups
}

fn parse_input(input: &str) -> Vec<(&str, Vec<i32>)> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(" ").collect();

            let hints = parts[1]
                .split(",")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            (parts[0], hints)
        })
        .collect()
}

fn serialize_memo_key(spring_condition: &String, position: usize) -> String {
    format!("{}-{}", spring_condition, position)
}

fn helper(
    spring_condition: String,
    hints: &Vec<i32>,
    position: usize,
    memo: &mut HashMap<String, i32>,
) -> i32 {
    let cache_key = serialize_memo_key(&spring_condition, position);

    if let Some(count) = memo.get(&cache_key) {
        return *count;
    }

    if position == spring_condition.len() {
        return if is_valid(&spring_condition, hints) {
            1
        } else {
            0
        };
    }

    if spring_condition.chars().nth(position).unwrap() == '?' {
        let mut damaged = spring_condition.clone();
        damaged.replace_range(position..position + 1, "#");

        let mut undamaged = spring_condition.clone();
        undamaged.replace_range(position..position + 1, ".");

        let count = helper(damaged, hints, position + 1, memo)
            + helper(undamaged, hints, position + 1, memo);

        memo.insert(cache_key, count);

        return count;
    }

    let count = helper(spring_condition, hints, position + 1, memo);

    memo.insert(cache_key, count);

    count
}

fn is_valid(spring_condition: &String, hints: &Vec<i32>) -> bool {
    count_contiguous_damaged_springs(&spring_condition) == *hints
}

fn solve(input: &Vec<Record>) -> i32 {
    let mut count: i32 = 0;

    let mut memo: HashMap<String, i32> = HashMap::new();

    for record in input {
        let (spring_condition, hints) = record;

        count += helper(spring_condition.to_string(), hints, 0, &mut memo);
    }

    count
}

fn unfold_record(record: &Record) -> (String, Vec<i32>) {
    let (spring_condition, hints) = record;

    let unfolded_spring_condition = vec![*spring_condition; 5].join("?");
    let unfolded_hints: Vec<i32> = hints.repeat(5);

    (unfolded_spring_condition, unfolded_hints)
}

fn solve_unfolded(input: &Vec<Record>) -> i32 {
    let mut count: i32 = 0;

    let mut memo: HashMap<String, i32> = HashMap::new();

    for record in input {
        let (spring_condition, hints) = unfold_record(record);

        count += helper(spring_condition, &hints, 0, &mut memo);
    }

    count
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let parsed_input = parse_input(&input);

    let answer1 = solve(&parsed_input);
    println!("Answer 1: {}", answer1);
    let answer2 = solve_unfolded(&parsed_input);

    println!("Answer 2: {}", answer2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "???.### 1,1,3\n.??..??...?##. 1,1,3";

        let parsed = parse_input(input);

        assert_eq!(parsed[0].0, "???.###");
        assert_eq!(parsed[0].1, vec![1, 1, 3]);
        assert_eq!(parsed[1].0, ".??..??...?##.");
        assert_eq!(parsed[1].1, vec![1, 1, 3]);
    }

    #[test]
    fn test_count_contiguous_damaged_springs() {
        let input = "#.##...###.#";

        assert_eq!(count_contiguous_damaged_springs(input), vec![1, 2, 3, 1]);
    }

    #[test]
    fn test_solve_1() {
        let input: Vec<(&str, Vec<i32>)> = vec![
            ("???.###", vec![1, 1, 3]),
            (".??..??...?##.", vec![1, 1, 3]),
            ("?#?#?#?#?#?#?#?", vec![1, 3, 1, 6]),
            ("????.#...#...", vec![4, 1, 1]),
            ("????.######..#####.", vec![1, 6, 5]),
            ("?###????????", vec![3, 2, 1]),
        ];

        assert_eq!(solve(&input), 21);
    }

    #[test]
    fn test_unfold_record() {
        let input: Vec<(&str, Vec<i32>)> = vec![(".#", vec![1])];

        let unfolded = unfold_record(&input[0]);

        assert_eq!(unfolded.0, ".#?.#?.#?.#?.#");
        assert_eq!(unfolded.1, vec![1, 1, 1, 1, 1]);
    }
}
