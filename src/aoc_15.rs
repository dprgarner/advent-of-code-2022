use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;
use std::error::Error;

#[derive(Debug, PartialEq)]
struct Sensor {
    location: (i32, i32),
    beacon: (i32, i32),
    distance: i32,
}

fn get_distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn parse_line(input: &str) -> Result<Sensor, Box<dyn Error>> {
    let re = Regex::new("(?:x|y)=([0-9-]+)").unwrap();

    let captures = re.captures_iter(input).collect_vec();
    if captures.len() != 4 {
        return Err("Incorrect number of captures".into());
    }
    let ints = captures
        .iter()
        .map(|x| x.get(1)?.as_str().parse().ok())
        .collect::<Option<Vec<i32>>>()
        .expect("Could not parse capture groups");

    let location = (ints[0], ints[1]);
    let beacon = (ints[2], ints[3]);
    Ok(Sensor {
        location,
        beacon,
        distance: get_distance(location, beacon),
    })
}

fn count_beaconless_in_row(sensor_beacons: &Vec<Sensor>, row: i32) -> usize {
    let mut ranges_in_row = Vec::new();
    for sensor in sensor_beacons.iter() {
        let distance_remaining = sensor.distance - (sensor.location.1 - row).abs();
        if distance_remaining >= 0 {
            ranges_in_row.push((
                sensor.location.0 - distance_remaining,
                sensor.location.0 + distance_remaining + 1,
            ));
        }
    }

    // Brute force solution
    let mut cols_where_beacon_not_present = HashSet::new();
    for (start, end) in ranges_in_row {
        for i in start..end {
            cols_where_beacon_not_present.insert(i);
        }
    }
    for sensor in sensor_beacons.iter() {
        if sensor.beacon.1 == row {
            cols_where_beacon_not_present.remove(&sensor.beacon.0);
        }
    }
    cols_where_beacon_not_present.len()
}

fn find_beaconless_in_row(sensor_beacons: &Vec<Sensor>, row: i32, max: i32) -> Option<i32> {
    let mut ranges_in_row = Vec::new();
    for sensor in sensor_beacons.iter() {
        let distance_remaining = sensor.distance - (sensor.location.1 - row).abs();
        if distance_remaining >= 0 {
            ranges_in_row.push((
                sensor.location.0 - distance_remaining,
                sensor.location.0 + distance_remaining + 1,
            ));
        }
    }
    ranges_in_row.sort();

    let mut end = ranges_in_row[0].1;
    let last_idx = ranges_in_row.len();
    for idx in 1..last_idx {
        let x = ranges_in_row[idx].0;
        if x > end && x >= 0 && x < max {
            return Some(x - 1);
        }
        end = end.max(ranges_in_row[idx].1);
    }
    None
}
fn find_beaconless(sensors: &Vec<Sensor>, max: i32) -> Option<(i32, i32)> {
    for row in 0..max {
        if let Some(col) = find_beaconless_in_row(&sensors, row, max) {
            return Some((col, row));
        }
    }
    None
}

pub fn solve_a(input: impl Iterator<Item = String>) -> Result<usize, Box<dyn Error>> {
    let sensors = (input
        .map(|x| parse_line(&x))
        .collect::<Result<Vec<Sensor>, _>>())?;

    let count = count_beaconless_in_row(&sensors, 2000000);
    Ok(count)
}

// Brute-force solution. Took ~10s to run.
pub fn solve_b(input: impl Iterator<Item = String>) -> Result<i64, Box<dyn Error>> {
    let sensors = (input
        .map(|x| parse_line(&x))
        .collect::<Result<Vec<Sensor>, _>>())?;

    let beaconless = find_beaconless(&sensors, 4000000).expect("Should have found a beacon");
    println!("Found beacon: {:?}", beaconless);

    Ok((beaconless.0 as i64) * 4000000 + (beaconless.1 as i64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_a_line() {
        let input = "Sensor at x=168575, y=491461: closest beacon is at x=1053731, y=-142061";
        let result = parse_line(input).unwrap();
        assert_eq!(
            result,
            (Sensor {
                location: (168575, 491461),
                beacon: (1053731, -142061),
                distance: 1518678,
            })
        );
    }

    #[test]
    fn it_counts_beaconless_squares_in_row_10() {
        let input = [
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
            "Sensor at x=9, y=16: closest beacon is at x=10, y=16",
            "Sensor at x=13, y=2: closest beacon is at x=15, y=3",
            "Sensor at x=12, y=14: closest beacon is at x=10, y=16",
            "Sensor at x=10, y=20: closest beacon is at x=10, y=16",
            "Sensor at x=14, y=17: closest beacon is at x=10, y=16",
            "Sensor at x=8, y=7: closest beacon is at x=2, y=10",
            "Sensor at x=2, y=0: closest beacon is at x=2, y=10",
            "Sensor at x=0, y=11: closest beacon is at x=2, y=10",
            "Sensor at x=20, y=14: closest beacon is at x=25, y=17",
            "Sensor at x=17, y=20: closest beacon is at x=21, y=22",
            "Sensor at x=16, y=7: closest beacon is at x=15, y=3",
            "Sensor at x=14, y=3: closest beacon is at x=15, y=3",
            "Sensor at x=20, y=1: closest beacon is at x=15, y=3",
        ]
        .map(|x| parse_line(&x).unwrap())
        .into_iter()
        .collect_vec();

        let result = count_beaconless_in_row(&input, 10);
        assert_eq!(result, 26);
    }
}
