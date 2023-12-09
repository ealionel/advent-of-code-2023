use std::{
    collections::{HashMap, HashSet},
    fs,
};

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

fn is_star(grid: &Grid, col: i32, row: i32) -> bool {
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;

    if col >= width || row >= height || col < 0 || row < 0 {
        // println!("({},{}) oob", col, row,);

        return false;
    }

    let character = grid[row as usize].chars().nth(col as usize).unwrap();

    character == '*'
}

fn serialize_cell(col: i32, row: i32) -> String {
    format!("{}:{}", col, row)
}

fn is_special_cell_store(
    grid: &Grid,
    col: i32,
    row: i32,
    store: &mut HashMap<String, Vec<i32>>,
    from: i32,
) -> bool {
    let r = is_special_cell(grid, col, row);

    let entry = store.entry(serialize_cell(col, row)).or_insert(Vec::new());

    entry.push(from);

    r
}

fn visit_line(grid: &Grid, line_index: u32, store: &mut HashMap<String, Vec<i32>>) -> i32 {
    let line = &grid[line_index as usize];

    let mut sum: i32 = 0;

    let mut current_number: String = String::from("");
    let mut is_current_part_number = false;

    let line_iter: std::str::Chars<'_> = line.chars();

    let mut special_cells_buffer: HashSet<(i32, i32)> = HashSet::new();

    for (i, c) in line_iter.clone().enumerate() {
        let previous_col: i32 = i as i32 - 1;
        let next_col: i32 = i as i32 + 1;
        let next_row: i32 = line_index as i32 + 1;
        let previous_row: i32 = line_index as i32 - 1;

        let is_next_col_number = match line_iter.clone().nth(i + 1) {
            Some(val) => val.is_digit(10),
            None => false,
        };

        // println!("({}) is_next_col_number: {}", c, is_next_col_number);

        if c.is_digit(10) {
            current_number.push(c);

            let cells_to_check: Vec<(i32, i32)> = vec![
                (previous_col, line_index as i32),
                (i as i32, previous_row),
                (i as i32, next_row),
                (previous_col, next_row),
                (previous_col, previous_row),
                (next_col, line_index as i32),
                (next_col, next_row),
                (next_col, previous_row),
            ];

            // println!("Checking cells: {:?}", cells_to_check);

            for (col, row) in cells_to_check {
                let is_cell_special = is_special_cell(grid, col, row) && is_star(grid, col, row);

                if is_cell_special {
                    // println!("Found special cell ({},{})", col, row);
                    special_cells_buffer.insert((col, row));
                }

                is_current_part_number = is_current_part_number || is_cell_special;
            }
            // println!("is_current_part_number {}", is_current_part_number);
        } else if c != '*' {
            is_current_part_number = false;
            current_number.clear();
        } else {
            // special_cells_buffer.push((i as i32, line_index as i32));
            is_current_part_number = true;
        }

        if !is_next_col_number && is_current_part_number {
            // println!("Line #{}: {} += {}", line_index, sum, current_number);
            if let Ok(parsed_current_number) = current_number.parse::<i32>() {
                println!(
                    "Adding {} to {:?}",
                    parsed_current_number, special_cells_buffer
                );
                for (col, row) in special_cells_buffer.clone() {
                    store
                        .entry(serialize_cell(col, row))
                        .or_insert(Vec::new())
                        .push(parsed_current_number);
                }
                sum += parsed_current_number;
            }

            special_cells_buffer.clear();
            current_number.clear();
            is_current_part_number = false;
        }
    }

    return sum;
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let grid: Vec<String> = input.lines().map(|line: &str| line.to_string()).collect();

    let store = &mut HashMap::new();

    let answer1 = input.lines().enumerate().fold(0, |sum, (line_index, _)| {
        let line_value: i32 = visit_line(&grid, line_index as u32, store);
        // println!("Line #{}\tvalue = {}\tsum={}", line_index, line_value, sum);
        sum + line_value
    });

    let mut answer2 = 0;

    for (cell, numbers) in store {
        if numbers.len() == 2 {
            answer2 += numbers[0] * numbers[1];
        }
    }

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);
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
