use std::{cmp::max, collections::HashSet, fs, thread::current};

type Map = Vec<Vec<char>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

type Cell = (i32, i32); // (row, col)

fn get_allowed_directions(character: char) -> Vec<Direction> {
    let direction = match character {
        '|' => vec![Direction::North, Direction::South],
        '-' => vec![Direction::West, Direction::East],
        'L' => vec![Direction::North, Direction::East],
        'J' => vec![Direction::North, Direction::West],
        '7' => vec![Direction::South, Direction::West],
        'F' => vec![Direction::South, Direction::East],
        '.' => vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ],
        'S' => vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ],
        _ => panic!("Invalid direction"),
    };
    direction
}

fn allowed_directions_to_char(direction: &HashSet<Direction>) -> char {
    match *direction {
        direction if direction == HashSet::from([Direction::North, Direction::South]) => '|',
        direction if direction == HashSet::from([Direction::West, Direction::East]) => '-',
        direction if direction == HashSet::from([Direction::North, Direction::East]) => 'L',
        direction if direction == HashSet::from([Direction::North, Direction::West]) => 'J',
        direction if direction == HashSet::from([Direction::South, Direction::West]) => '7',
        direction if direction == HashSet::from([Direction::South, Direction::East]) => 'F',
        _ => panic!("Invalid direction"),
    }
}

fn add_direction((row, col): Cell, direction: Direction) -> Cell {
    match direction {
        Direction::North => (row - 1, col),
        Direction::South => (row + 1, col),
        Direction::East => (row, col + 1),
        Direction::West => (row, col - 1),
    }
}

fn is_bounded(map: &Map, (row, col): Cell) -> bool {
    row >= 0
        && row < map.len() as i32
        && col >= 0
        && col < map[0].len() as i32
        && map[row as usize][col as usize] != '.'
}

fn is_only_bounded(map: &Map, (row, col): Cell) -> bool {
    row >= 0
        && row < map.len() as i32
        && col >= 0
        && col < map[0].len() as i32
        && map[row as usize][col as usize] == '.'
}

fn is_cell_border(map: &Map, (row, col): Cell) -> bool {
    row == 0 || row == map.len() as i32 - 1 || col == 0 || col == map[0].len() as i32 - 1
}

fn can_go(map: &Map, from: Cell, to: Cell) -> bool {
    if (from.0 - to.0).abs() != 1 && (from.1 - to.1).abs() != 1 || !is_bounded(map, to) {
        return false;
    }

    let to_cell = map[to.0 as usize][to.1 as usize];

    let allowed_directions = get_allowed_directions(to_cell);

    for direction in allowed_directions {
        let (row, col) = add_direction(to, direction);

        if (row, col) == from {
            return true;
        }
    }

    false
}

fn dfs(map: &Map, visited: &mut Vec<Vec<bool>>, (row, col): Cell) -> i32 {
    let current_cell = (row, col);

    if !is_bounded(map, (row, col)) || visited[row as usize][col as usize] {
        return 0;
    }

    visited[row as usize][col as usize] = true;

    let allowed_directions: Vec<Direction> =
        get_allowed_directions(map[row as usize][col as usize]);

    let mut steps = 0;

    for direction in allowed_directions {
        let next_cell: (i32, i32) = add_direction((row, col), direction);

        if can_go(map, current_cell, next_cell) {
            steps = max(steps, dfs(map, visited, next_cell));
        }
    }

    steps + 1
}

fn find_starting_cell(map: &Map) -> Cell {
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == 'S' {
                return (row as i32, col as i32);
            }
        }
    }

    panic!("No starting cell found");
}

fn get_farthest_distance(map: &Map) -> (i32, Vec<Vec<bool>>) {
    let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];

    let starting_cell = find_starting_cell(map);

    (dfs(map, &mut visited, starting_cell) / 2, visited)
}

fn substitute_starting_point(map: &mut Map) {
    let starting_cell = find_starting_cell(map);
    let all_directions: Vec<Direction> = vec![
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];

    let allowed_directions: HashSet<Direction> = all_directions
        .iter()
        .filter(|direction| {
            can_go(
                map,
                starting_cell,
                add_direction(starting_cell, **direction),
            )
        })
        .cloned()
        .collect::<HashSet<Direction>>();

    let new_char: char = allowed_directions_to_char(&allowed_directions);

    map[starting_cell.0 as usize][starting_cell.1 as usize] = 'L';
}

fn dfs_enclosed(map: &Map, visited: &mut Vec<Vec<bool>>, (row, col): Cell) -> (i32, bool) {
    let current_cell = (row, col);

    if !is_only_bounded(map, (row, col))
        || visited[row as usize][col as usize]
        || map[row as usize][col as usize] != '.'
    {
        return (0, false);
    }

    visited[row as usize][col as usize] = true;

    let allowed_directions: Vec<Direction> =
        get_allowed_directions(map[row as usize][col as usize]);

    let mut steps = 0;
    let mut is_current_cell_border: bool = is_cell_border(map, current_cell);

    for direction in allowed_directions {
        let next_cell: (i32, i32) = add_direction(current_cell, direction);
        let (added_steps, is_border) = dfs_enclosed(map, visited, next_cell);
        steps += added_steps;
        is_current_cell_border = is_current_cell_border || is_border;
    }
    println!(
        "({},{}) steps: {}, is_current_cell_border={}",
        row,
        col,
        steps + 1,
        is_current_cell_border
    );

    (steps + 1, is_current_cell_border)
}

fn find_enclosed_tiles(map: &Map, loop_cells: &mut Vec<Vec<bool>>) -> i32 {
    let mut enclosed_tiles = 0;
    for row in 0..map.len() as i32 {
        let mut is_inside: bool = false;

        for col in 0..map[0].len() as i32 {
            let cell = (row, col);
            let cell_char = map[row as usize][col as usize];

            if loop_cells[row as usize][col as usize] {
                if cell_char = 'S' {}

                if cell_char == 'L' {
                    is_inside = !is_inside
                } else if cell_char == '|' {
                    is_inside = !is_inside;
                } else if cell_char == 'J' {
                    is_inside = !is_inside;
                } else if cell_char == 'L' {
                    is_inside = !is_inside;
                }
            }

            if is_inside && cell_char == '.' {
                enclosed_tiles += 1;
            }
        }
    }

    enclosed_tiles
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let (answer1, mut visited) = get_farthest_distance(&map);

    let answer2 = find_enclosed_tiles(&map, &mut visited);

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_connected() {
        let map: Map = vec![
            vec!['.', '|', '.', '.'],
            vec!['|', 'S', 'F', '.'],
            vec!['.', 'J', '.', '.'],
            vec!['.', '.', '.', '.'],
        ];

        assert_eq!(can_go(&map, (1, 1), (1, 2)), false);
        assert_eq!(can_go(&map, (1, 1), (2, 1)), true);
        assert_eq!(can_go(&map, (1, 1), (0, 1)), true);
        assert_eq!(can_go(&map, (1, 1), (1, 0)), false);
        assert_eq!(can_go(&map, (1, 1), (0, 0)), false);
    }

    #[test]
    fn test_find_starting_cell() {
        let map: Map = vec![
            vec!['.', '|', '.', '.'],
            vec!['|', 'S', 'F', '.'],
            vec!['.', 'J', '.', '.'],
            vec!['.', '.', '.', '.'],
        ];

        assert_eq!(find_starting_cell(&map), (1, 1));
    }

    #[test]
    fn test_get_farthest_distance_1() {
        let map: Map = vec![
            vec!['.', '.', 'F', '7', '.'],
            vec!['.', 'F', 'J', '|', '.'],
            vec!['S', 'J', '.', 'L', '7'],
            vec!['|', 'F', '-', '-', 'J'],
            vec!['L', 'J', '.', '.', '.'],
        ];

        assert_eq!(get_farthest_distance(&map).0, 8);
    }

    #[test]
    fn test_get_farthest_distance_2() {
        let map = vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['.', 'S', '-', '7', '.'],
            vec!['.', '|', '.', '|', '.'],
            vec!['.', 'L', '-', 'J', '.'],
            vec!['.', '.', '.', '.', '.'],
        ];

        assert_eq!(get_farthest_distance(&map).0, 4);
    }

    #[test]
    fn test_find_enclosed_tiles() {
        // ...........
        // .S-------7.
        // .|F-----7|.
        // .||.....||.
        // .||.....||.
        // .|L-7.F-J|.
        // .|..|.|..|.
        // .L--J.L--J.
        // ...........

        let map = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', 'S', '-', '-', '-', '-', '-', '-', '-', '7', '.'],
            vec!['.', '|', 'F', '-', '-', '-', '-', '-', '7', '|', '.'],
            vec!['.', '|', '|', '.', '.', '.', '.', '.', '|', '|', '.'],
            vec!['.', '|', '|', '.', '.', '.', '.', '.', '|', '|', '.'],
            vec!['.', '|', 'L', '-', '7', '.', 'F', '-', 'J', '|', '.'],
            vec!['.', '|', '.', '.', '|', '.', '|', '.', '.', '|', '.'],
            vec!['.', 'L', '-', '-', 'J', '.', 'L', '-', '-', 'J', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];

        let (_, mut visited) = get_farthest_distance(&map);

        assert_eq!(find_enclosed_tiles(&map, &mut visited), 4);
    }
}
