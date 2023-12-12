use std::fs;

#[derive(PartialEq, Debug)]
struct RaceRecord {
    time: u64,
    distance: u64,
}

fn parse_line(line: &str) -> Vec<u64> {
    line.split(" ")
        .filter(|x| x != &"")
        .skip(1)
        .map(|n| n.parse::<u64>().unwrap())
        .collect()
}

fn get_races(input: &str) -> Vec<RaceRecord> {
    let lines: Vec<&str> = input.lines().collect();
    let time = parse_line(lines[0]);
    let distance = parse_line(lines[1]);

    time.iter()
        .zip(distance.iter())
        .map(|(t, d)| RaceRecord {
            time: *t,
            distance: *d,
        })
        .collect()
}

fn compute_distance(available_time: u64, press_time: u64) -> u64 {
    (available_time - press_time) * press_time
}

fn determine_ways_to_beat_record(record: &RaceRecord) -> u64 {
    let mut ways = 0;
    for press_time in 0..record.time {
        let distance = compute_distance(record.time, press_time);
        if distance > record.distance {
            ways += 1;
        }
    }
    ways
}

fn get_answer_1(race_records: &Vec<RaceRecord>) -> u64 {
    race_records
        .iter()
        .map(|record| determine_ways_to_beat_record(record))
        .fold(1, |acc, x| acc * x)
}

fn main() {
    let input = fs::read_to_string("input2.txt").unwrap();

    let races_records = get_races(&input);

    let answer1 = get_answer_1(&races_records);
    println!("Answer 1: {}", answer1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("Time: 1   2 3 4"), vec![1, 2, 3, 4]);
        assert_eq!(parse_line("Time: 1 2        3 4 5"), vec![1, 2, 3, 4, 5]);
        assert_eq!(
            parse_line("Distance:    1    2  3 4 5 6"),
            vec![1, 2, 3, 4, 5, 6]
        );
    }

    #[test]
    fn test_get_races() {
        let input = "Time: 1 2 3 4 5 6\nDistance: 1 2 3 4 5 6";

        assert_eq!(
            get_races(input),
            vec![
                RaceRecord {
                    time: 1,
                    distance: 1
                },
                RaceRecord {
                    time: 2,
                    distance: 2
                },
                RaceRecord {
                    time: 3,
                    distance: 3
                },
                RaceRecord {
                    time: 4,
                    distance: 4
                },
                RaceRecord {
                    time: 5,
                    distance: 5
                },
                RaceRecord {
                    time: 6,
                    distance: 6
                },
            ]
        )
    }

    #[test]
    fn test_compute_distance() {
        assert_eq!(compute_distance(7, 0), 0);
        assert_eq!(compute_distance(7, 1), 6);
        assert_eq!(compute_distance(7, 2), 10);
        assert_eq!(compute_distance(7, 3), 12);
        assert_eq!(compute_distance(7, 4), 12);
        assert_eq!(compute_distance(7, 5), 10);
        assert_eq!(compute_distance(7, 6), 6);
        assert_eq!(compute_distance(7, 7), 0);
    }

    #[test]
    fn test_determine_ways_to_beat_record() {
        assert_eq!(
            determine_ways_to_beat_record(&RaceRecord {
                time: 7,
                distance: 9
            }),
            4
        );
        assert_eq!(
            determine_ways_to_beat_record(&RaceRecord {
                time: 15,
                distance: 40
            }),
            8
        );
        assert_eq!(
            determine_ways_to_beat_record(&RaceRecord {
                time: 30,
                distance: 200
            }),
            9
        );
    }
}
