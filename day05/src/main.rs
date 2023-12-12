use rayon::prelude::*;
use std::{
    fs::{self},
    io,
};

#[derive(Debug)]
struct MapperGroup {
    name: String,
    mappers: Vec<Mapper>,
}

impl MapperGroup {
    fn new() -> Self {
        Self {
            name: String::new(),
            mappers: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Mapper {
    destination: u64,
    source: u64,
    range: u64,
}

impl Mapper {
    fn new(destination: u64, source: u64, range: u64) -> Self {
        Self {
            destination,
            source,
            range,
        }
    }
}

fn parse_seeds(line: &str) -> Vec<u64> {
    let seeds: Vec<u64> = line
        .split_whitespace()
        .filter(|n: &&str| n != &"seeds:")
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    seeds
}

fn parse_almanac(input: &str) -> (Vec<u64>, Vec<MapperGroup>) {
    let mut line_iter = input.lines();

    let seed_line = line_iter.next().unwrap();

    let seeds = parse_seeds(seed_line);

    line_iter.next();

    let mut current_mapper_group: MapperGroup = MapperGroup::new();

    let mut almanac: Vec<MapperGroup> = Vec::new();

    for line in line_iter {
        if line == "" {
            almanac.push(current_mapper_group);
            current_mapper_group = MapperGroup::new();
        } else if line.ends_with("map:") {
            let name = line[..line.len() - 5].to_string();
            current_mapper_group.name = name;
        } else {
            let parts = line
                .split(" ")
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            let current_mapper = Mapper::new(parts[0], parts[1], parts[2]);
            current_mapper_group.mappers.push(current_mapper);
        }
    }

    almanac.push(current_mapper_group);

    (seeds, almanac)
}

fn map_one_step(seed: u64, mapper_group: &MapperGroup) -> u64 {
    let mapper = mapper_group
        .mappers
        .iter()
        .find(|mapper| seed >= mapper.source && seed < mapper.source + mapper.range);

    if let Some(mapper) = mapper {
        return seed - mapper.source + mapper.destination;
    }

    seed
}

fn map_seed_to_location(seed: u64, almanac: &Vec<MapperGroup>) -> u64 {
    let mut current_seed = seed;
    // println!("\nNew seed: {}", seed);

    for mapper_group in almanac {
        let mapped_value = map_one_step(current_seed, mapper_group);
        // println!(
        //     "Mapping {} to {} with {}",
        //     current_seed, mapped_value, mapper_group.name
        // );
        current_seed = mapped_value;
    }

    current_seed
}

fn get_pairs(seeds: &Vec<u64>) -> Vec<(u64, u64)> {
    let mut pairs: Vec<(u64, u64)> = Vec::new();

    for i in (0..seeds.len()).step_by(2) {
        pairs.push((seeds[i], seeds[i + 1]));
    }

    pairs
}

fn map_location_to_seed(location: u64, almanac: &Vec<MapperGroup>) -> u64 {
    let mut current_location = location;

    for mapper_group in almanac.iter().rev() {
        let mapper = mapper_group.mappers.iter().find(|mapper| {
            current_location >= mapper.destination
                && current_location < mapper.destination + mapper.range
        });

        if let Some(mapper) = mapper {
            current_location = current_location - mapper.destination + mapper.source;
        }
    }

    current_location
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    let (seeds, almanac) = parse_almanac(&input);

    let min_seed_location = seeds
        .iter()
        .map(|seed| map_seed_to_location(*seed, &almanac))
        .min()
        .unwrap();

    println!("Answer 1: {}", min_seed_location);

    let min_location = almanac
        .last()
        .unwrap()
        .mappers
        .iter()
        .map(|mapper| mapper.destination)
        .min()
        .unwrap();

    println!("Answer 2: {}", ranges);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let seeds = parse_seeds("seeds: 515785082 87905039 2104518691 503149843 720333403 385234193 1357904101 283386167 93533455 128569683 2844655470 24994629 3934515023 67327818 2655687716 8403417 3120497449 107756881 4055128129 9498708");

        assert_eq!(
            seeds,
            vec![
                515785082, 87905039, 2104518691, 503149843, 720333403, 385234193, 1357904101,
                283386167, 93533455, 128569683, 2844655470, 24994629, 3934515023, 67327818,
                2655687716, 8403417, 3120497449, 107756881, 4055128129, 9498708
            ]
        );
    }

    #[test]
    fn test_map_one_step() {
        let mapper_group = MapperGroup {
            name: "test".to_string(),
            mappers: vec![Mapper::new(50, 98, 2), Mapper::new(52, 50, 48)],
        };

        assert_eq!(map_one_step(98, &mapper_group), 50);
        assert_eq!(map_one_step(99, &mapper_group), 51);
        assert_eq!(map_one_step(10, &mapper_group), 10);
        assert_eq!(map_one_step(79, &mapper_group), 81);
        assert_eq!(map_one_step(14, &mapper_group), 14);
        assert_eq!(map_one_step(55, &mapper_group), 57);
        assert_eq!(map_one_step(13, &mapper_group), 13);
    }

    #[test]
    fn test_map_seed_to_location() {
        let almanac = vec![
            MapperGroup {
                name: "test".to_string(),
                mappers: vec![Mapper::new(50, 98, 2), Mapper::new(52, 50, 48)],
            },
            MapperGroup {
                name: "test".to_string(),
                mappers: vec![
                    Mapper::new(0, 15, 37),
                    Mapper::new(37, 52, 2),
                    Mapper::new(39, 0, 15),
                ],
            },
            MapperGroup {
                name: "test".to_string(),
                mappers: vec![
                    Mapper::new(49, 53, 8),
                    Mapper::new(0, 11, 42),
                    Mapper::new(42, 0, 7),
                    Mapper::new(57, 7, 4),
                ],
            },
            MapperGroup {
                name: "test".to_string(),
                mappers: vec![Mapper::new(88, 18, 7), Mapper::new(18, 25, 70)],
            },
            MapperGroup {
                name: "test".to_string(),
                mappers: vec![
                    Mapper::new(45, 77, 23),
                    Mapper::new(81, 45, 19),
                    Mapper::new(68, 64, 13),
                ],
            },
            MapperGroup {
                name: "test".to_string(),
                mappers: vec![Mapper::new(0, 69, 1), Mapper::new(1, 0, 69)],
            },
            MapperGroup {
                name: "test".to_string(),
                mappers: vec![Mapper::new(60, 56, 37), Mapper::new(56, 93, 4)],
            },
        ];

        assert_eq!(map_seed_to_location(79, &almanac), 82);
        assert_eq!(map_seed_to_location(14, &almanac), 43);
        assert_eq!(map_seed_to_location(55, &almanac), 86);
        assert_eq!(map_seed_to_location(13, &almanac), 35);
    }

    #[test]
    fn test_get_pairs() {
        let seeds = vec![
            515785082, 87905039, 2104518691, 503149843, 720333403, 385234193, 1357904101,
            283386167, 93533455, 128569683, 2844655470, 24994629, 3934515023, 67327818, 2655687716,
            8403417, 3120497449, 107756881, 4055128129, 9498708,
        ];

        let pairs = get_pairs(&seeds);

        assert_eq!(
            pairs,
            vec![
                (515785082, 87905039),
                (2104518691, 503149843),
                (720333403, 385234193),
                (1357904101, 283386167),
                (93533455, 128569683),
                (2844655470, 24994629),
                (3934515023, 67327818),
                (2655687716, 8403417),
                (3120497449, 107756881),
                (4055128129, 9498708),
            ]
        );
    }

    #[test]
    fn test_map_location_to_seed() {
        let almanac = vec![
            MapperGroup {
                name: "test".to_string(),
                mappers: vec![Mapper::new(50, 98, 2), Mapper::new(52, 50, 48)],
            },
            MapperGroup {
                name: "test".to_string(),
                mappers: vec![
                    Mapper::new(0, 15, 37),
                    Mapper::new(37, 52, 2),
                    Mapper::new(39, 0, 15),
                ],
            },
            MapperGroup {
                name: "test".to_string(),
                mappers: vec![
                    Mapper::new(49, 53, 8),
                    Mapper::new(0, 11, 42),
                    Mapper::new(42, 0, 7),
                    Mapper::new(57, 7, 4),
                ],
            },
            MapperGroup {
                name: "test".to_string(),
                mappers: vec![Mapper::new(88, 18, 7), Mapper::new(18, 25, 70)],
            },
            MapperGroup {
                name: "test".to_string(),
                mappers: vec![
                    Mapper::new(45, 77, 23),
                    Mapper::new(81, 45, 19),
                    Mapper::new(68, 64, 13),
                ],
            },
            MapperGroup {
                name: "test".to_string(),
                mappers: vec![Mapper::new(0, 69, 1), Mapper::new(1, 0, 69)],
            },
            MapperGroup {
                name: "test".to_string(),
                mappers: vec![Mapper::new(60, 56, 37), Mapper::new(56, 93, 4)],
            },
        ];

        assert_eq!(map_location_to_seed(82, &almanac), 79);
        assert_eq!(map_location_to_seed(43, &almanac), 14);
        assert_eq!(map_location_to_seed(86, &almanac), 55);
        assert_eq!(map_location_to_seed(35, &almanac), 13);
    }
}
