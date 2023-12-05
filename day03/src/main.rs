use std::fs;

type Grid = Vec<String>;

fn is_special_cell(grid: &Grid, col: i32, row: i32) -> bool {
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;

    if col >= width || row >= height || col < 0 || row < 0 {
        // println!("({},{}) oob", col, row,);

        return false;
    }

    let character = grid[row as usize].chars().nth(col as usize).unwrap();

    // println!(
    //     "({},{}) {} {} {}",
    //     col,
    //     row,
    //     character,
    //     !character.is_digit(10),
    //     character != '.'
    // );

    !character.is_digit(10) && character != '.'
}

fn visit_line(grid: &Grid, line_index: u32) -> i32 {
    let line = &grid[line_index as usize];

    let mut sum: i32 = 0;

    let mut current_number: String = String::from("");
    let mut is_current_part_number = false;

    let line_iter: std::str::Chars<'_> = line.chars();

    for (i, c) in line_iter.clone().enumerate() {
        let previous_col: i32 = i as i32 - 1;
        let next_col: i32 = i as i32 + 1;

        let is_next_col_number = match line_iter.clone().nth(i + 1) {
            Some(val) => val.is_digit(10),
            None => false,
        };

        // println!("({}) is_next_col_number: {}", c, is_next_col_number);

        if c.is_digit(10) {
            current_number.push(c);

            is_current_part_number = is_current_part_number
                || is_special_cell(grid, previous_col, line_index as i32)
                || is_special_cell(grid, i as i32, line_index as i32 - 1)
                || is_special_cell(grid, i as i32, line_index as i32 + 1)
                || is_special_cell(grid, previous_col, line_index as i32 + 1)
                || is_special_cell(grid, previous_col, line_index as i32 - 1)
                || is_special_cell(grid, next_col, line_index as i32)
                || is_special_cell(grid, next_col, line_index as i32 + 1)
                || is_special_cell(grid, next_col, line_index as i32 - 1)
            // println!("is_current_part_number {}", is_current_part_number);
        } else if c == '.' {
            is_current_part_number = false;
            current_number.clear();
        } else {
            is_current_part_number = true;
        }

        if !is_next_col_number && is_current_part_number {
            println!("Line #{}: {} += {}", line_index, sum, current_number);
            sum += current_number.parse::<i32>().unwrap_or(0);
            current_number.clear();
            is_current_part_number = false;
        }
    }
    println!();

    return sum;
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let grid: Vec<String> = input.lines().map(|line: &str| line.to_string()).collect();

    let answer1 = input.lines().enumerate().fold(0, |sum, (line_index, _)| {
        let line_value = visit_line(&grid, line_index as u32);
        // println!("Line #{}\tvalue = {}\tsum={}", line_index, line_value, sum);
        sum + line_value
    });

    println!("Answer 1: {}", answer1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_cell() {
        let grid = Vec::from([String::from("12.."), String::from("#=.2")]);

        assert_eq!(is_special_cell(&grid, 0, 0), false);
        assert_eq!(is_special_cell(&grid, 0, 2), false);
        assert_eq!(is_special_cell(&grid, 0, 1), true);
        assert_eq!(is_special_cell(&grid, 1, 1), true);
    }

    #[test]
    fn test_visit_line() {
        let grid = Vec::from([
            String::from("3.12.."),
            String::from("..#..2"),
            String::from("21...!"),
            String::from("10..4!"),
            String::from("10#3.."),
        ]);

        assert_eq!(visit_line(&grid, 0), 12);
        assert_eq!(visit_line(&grid, 1), 2);
        assert_eq!(visit_line(&grid, 2), 21);
        assert_eq!(visit_line(&grid, 3), 14);
        assert_eq!(visit_line(&grid, 4), 13);
    }
}
