use std::{fs, io::empty};

use itertools::Itertools;

type Image = Vec<Vec<char>>;

fn transpose(v: &Image) -> Image {
    let mut transposed = vec![vec!['0'; v.len()]; v[0].len()];
    for (i, row) in v.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            transposed[j][i] = *value;
        }
    }
    transposed
}

fn parse_input(input: &str) -> Image {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn find_empty_rows_and_cols(image: &Image) -> (Vec<i64>, Vec<i64>) {
    let empty_rows = image
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|pixel| *pixel == '.'))
        .map(|(i, _)| i as i64)
        .collect::<Vec<i64>>();

    let empty_cols = transpose(image)
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|pixel| *pixel == '.'))
        .map(|(i, _)| i as i64)
        .collect::<Vec<i64>>();

    (empty_rows, empty_cols)
}

fn find_galaxies_positions(image: &Image) -> Vec<(i64, i64)> {
    let mut positions = Vec::new();

    for (i, row) in image.iter().enumerate() {
        for (j, pixel) in row.iter().enumerate() {
            if *pixel == '#' {
                positions.push((i as i64, j as i64));
            }
        }
    }

    positions
}

fn get_distance(a: &(i64, i64), b: &(i64, i64)) -> i64 {
    let (x1, y1) = a;
    let (x2, y2) = b;

    let dx: i64 = x2 - x1;
    let dy: i64 = y2 - y1;

    dx.abs() + dy.abs()
}

fn expand_image(image: &Image) -> Image {
    let (empty_rows, empty_cols) = find_empty_rows_and_cols(image);

    let new_width = image[0].len() + empty_cols.len();

    let mut expanded: Vec<Vec<char>> = image.clone();

    for row in expanded.iter_mut() {
        let mut added = 0;
        for empty_col in empty_cols.iter() {
            row.insert(*empty_col as usize + added, '.');
            added += 1;
        }
    }

    let mut added = 0;
    for row in empty_rows {
        expanded.insert(row as usize + added, vec!['.'; new_width]);
        added += 1;
    }

    expanded
}

fn get_length_of_all_shortest_paths(image: &Image) -> i64 {
    let positions = find_galaxies_positions(image);
    let combinations: itertools::Unique<itertools::Combinations<std::slice::Iter<'_, (i64, i64)>>> =
        positions.iter().combinations(2).unique();

    let mut sum: i64 = 0;

    for combination in combinations {
        let distance = get_distance(&combination[0], &combination[1]);
        sum += distance;
    }

    sum
}

fn count_elements_within_range(range: &(i64, i64), elements: &Vec<i64>) -> i64 {
    let (mut min, mut max) = range;

    if min > max {
        (max, min) = (min, max);
    }

    elements
        .iter()
        .filter(|element| **element > min && **element < max)
        .count() as i64
}

fn get_length_of_all_shortest_paths_expanded(image: &Image, expansion_factor: i64) -> i64 {
    let (empty_rows, empty_cols) = find_empty_rows_and_cols(image);
    let positions = find_galaxies_positions(image);
    let combinations = positions.iter().combinations(2).unique();

    let mut sum: i64 = 0;

    for combination in combinations {
        let a = combination[0];
        let b = combination[1];

        let added_vertically = count_elements_within_range(&(a.0, b.0), &empty_rows);
        let added_horizontally = count_elements_within_range(&(a.1, b.1), &empty_cols);

        println!(
            "a {:?}, b {:?}, added_vertically {:?}, added_horizontally {:?}",
            a, b, added_vertically, added_horizontally
        );

        let distance = get_distance(&combination[0], &combination[1]);
        sum += distance
            + added_vertically * (expansion_factor - 1)
            + added_horizontally * (expansion_factor - 1)
    }

    sum
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let image = parse_input(input.as_str());

    let answer1 = get_length_of_all_shortest_paths_expanded(&image, 2);
    let answer2 = get_length_of_all_shortest_paths_expanded(&image, 1000000);

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Image {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        parse_input(input)
    }

    #[test]
    fn test_parse_input() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let expected: Image = vec![
            vec!['.', '.', '.', '#', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
            vec!['.', '#', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
            vec!['#', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
        ];

        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_find_empty_rows_and_cols() {
        let input = get_input();

        let (empty_rows, empty_cols) = find_empty_rows_and_cols(&input);

        assert_eq!(empty_rows, vec![3, 7]);
        assert_eq!(empty_cols, vec![2, 5, 8]);
    }

    #[test]
    fn test_expand_image() {
        let input = get_input();

        let expected = parse_input(
            "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......",
        );

        assert_eq!(expand_image(&input), expected);

        // println!("{:?}", find_galaxies_positions(&expand_image(&input)));
    }

    #[test]
    fn test_find_galaxies_positions() {
        let input = get_input();

        let expected: Vec<(i64, i64)> = vec![
            (0, 3),
            (1, 7),
            (2, 0),
            (4, 6),
            (5, 1),
            (6, 9),
            (8, 7),
            (9, 0),
            (9, 4),
        ];

        assert_eq!(find_galaxies_positions(&input), expected);
    }

    #[test]
    fn test_get_distance() {
        let points: Vec<(i64, i64)> = vec![
            (0, 4),
            (1, 9),
            (2, 0),
            (5, 8),
            (6, 1),
            (7, 12),
            (10, 9),
            (11, 0),
            (11, 5),
        ];

        assert_eq!(get_distance(&points[4], &points[8]), 9);
        assert_eq!(get_distance(&points[0], &points[6]), 15);
        assert_eq!(get_distance(&points[2], &points[5]), 17);
        assert_eq!(get_distance(&points[7], &points[8]), 5);
    }

    #[test]
    fn test_get_all_shortest_paths_expanded() {
        let input = get_input();

        let expected = 374;

        assert_eq!(
            get_length_of_all_shortest_paths_expanded(&input, 2),
            expected
        );
    }

    #[test]
    fn test_get_all_shortest_paths_expanded_2() {
        let input = get_input();

        assert_eq!(get_length_of_all_shortest_paths_expanded(&input, 10), 1030);
        assert_eq!(get_length_of_all_shortest_paths_expanded(&input, 100), 8410);
    }
}
