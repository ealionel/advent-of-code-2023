use std::fs;

fn are_all_zero(line: &Vec<i32>) -> bool {
    line.iter().all(|n| *n == 0)
}

fn get_diff_vec(line: &Vec<i32>) -> Vec<i32> {
    let mut diff_vec = Vec::new();

    for i in 0..line.len() - 1 {
        let diff = line[i + 1] - line[i];

        diff_vec.push(diff);
    }

    diff_vec
}

fn find_next_value(line: &Vec<i32>) -> i32 {
    if are_all_zero(line) {
        return 0;
    }

    let diff_vec = get_diff_vec(line);

    let last_val: &i32 = line.last().unwrap();

    last_val + find_next_value(&diff_vec)
}

fn find_previous_value(line: &Vec<i32>) -> i32 {
    if are_all_zero(line) {
        return 0;
    }

    let diff_vec: Vec<i32> = get_diff_vec(line);

    let last_val: &i32 = line.first().unwrap();

    last_val - find_previous_value(&diff_vec)
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split(" ")
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let parsed_input: Vec<Vec<i32>> = input.lines().map(|line| parse_line(line)).collect();

    let answer1: i32 = parsed_input.iter().map(|line| find_next_value(line)).sum();
    let answer2: i32 = parsed_input
        .iter()
        .map(|line| find_previous_value(line))
        .sum();

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "5 1 9 5";
        let expected = vec![5, 1, 9, 5];

        assert_eq!(parse_line(line), expected);
    }

    #[test]
    fn test_get_diff_vec() {
        let line = vec![5, 1, 9, 5];
        let expected = vec![-4, 8, -4];

        assert_eq!(get_diff_vec(&line), expected);
    }

    #[test]
    fn test_find_next_value() {
        let line = vec![10, 13, 16, 21, 30, 45];
        let expected = 68;

        assert_eq!(find_next_value(&line), expected);
    }

    #[test]
    fn test_find_previous_value() {
        let line = vec![10, 13, 16, 21, 30, 45];
        let expected = 5;

        assert_eq!(find_previous_value(&line), expected);
    }
}
