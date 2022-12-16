use super::File;
use std::collections::HashSet;

pub fn star1(file: File) -> Result<usize, anyhow::Error> {
    const ROW: i64 = 2_000_000;
    let sensors: Vec<Sensor> = Sensor::expect_from_lines(&file.contents).collect();
    let n = count_unavailable_in_row(&sensors, ROW);
    Ok(n)
}

pub fn star2(file: File) -> Result<usize, anyhow::Error> {
    todo!()
}

struct Sensor {
    x: i64,
    y: i64,
    beacon_x: i64,
    beacon_y: i64
}

fn taken_ranges_in_row(sensors: &[Sensor], row: i64) -> impl Iterator<Item=(i64,i64)> + '_ {
    sensors.iter().filter_map(move |sensor| {
        let sensor_distance = (sensor.y - row).abs();
        let sensor_radius = sensor.radius();
        if sensor_distance > sensor_radius {
            None
        } else {
            let radius_at_y = sensor_radius - sensor_distance;
            let range = (sensor.x - radius_at_y, sensor.x + radius_at_y);
            Some(range)
        }
    })
}

// fn is_overlap(a: &(i64,i64), b: &(i64,i64)) -> bool {
//     if a.0 >= b.0 && a.0 <= b.1 {
//         true // a.0 is within b
//     } else if a.1 >= b.0 && a.1 <= b.1 {
//         true // a.1 is within b
//     } else if a.0 < b.0 && a.1 > b.1 {
//         true // a.0 and a.1 not in b, but do surround it
//     } else {
//         false
//     }
// }

fn count_unavailable_in_row(sensors: &[Sensor], row: i64) -> usize {
    let taken_ranges = taken_ranges_in_row(sensors, row);

    // The less lazy way would be to merge any overlapping ranges and then count
    // the sizes of remaining disjoint ones.
    let mut taken_locations: HashSet<i64> = taken_ranges.flat_map(|r| r.0..=r.1).collect();

    // dont count any beacons in the locations that are covered:
    for beacon_in_row in sensors.iter().filter(|s| s.beacon_y == row) {
        taken_locations.remove(&beacon_in_row.beacon_y);
    }

    taken_locations.len()
}

impl Sensor {
    fn radius(&self) -> i64 {
        (self.x - self.beacon_x).abs() + (self.y - self.beacon_y).abs()
    }
    fn expect_from_lines(input: &str) -> impl Iterator<Item=Sensor> + '_ {
        input.trim().lines().map(Sensor::expect_from_line)
    }
    fn expect_from_line(l: &str) -> Sensor {
        let re = regex!("x=(-?[0-9]+), y=(-?[0-9]+)");
        let mut caps = re.captures_iter(l);

        let sensor = caps.next().unwrap();
        let beacon = caps.next().unwrap();

        let get = |cap: &regex::Captures, n| cap
            .get(n).unwrap()
            .as_str().parse().unwrap();

        Sensor {
            x: get(&sensor, 1),
            y: get(&sensor, 2),
            beacon_x: get(&beacon, 1),
            beacon_y: get(&beacon, 2)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let example = "
            Sensor at x=2, y=18: closest beacon is at x=-2, y=15
            Sensor at x=9, y=16: closest beacon is at x=10, y=16
            Sensor at x=13, y=2: closest beacon is at x=15, y=3
            Sensor at x=12, y=14: closest beacon is at x=10, y=16
            Sensor at x=10, y=20: closest beacon is at x=10, y=16
            Sensor at x=14, y=17: closest beacon is at x=10, y=16
            Sensor at x=8, y=7: closest beacon is at x=2, y=10
            Sensor at x=2, y=0: closest beacon is at x=2, y=10
            Sensor at x=0, y=11: closest beacon is at x=2, y=10
            Sensor at x=20, y=14: closest beacon is at x=25, y=17
            Sensor at x=17, y=20: closest beacon is at x=21, y=22
            Sensor at x=16, y=7: closest beacon is at x=15, y=3
            Sensor at x=14, y=3: closest beacon is at x=15, y=3
            Sensor at x=20, y=1: closest beacon is at x=15, y=3
        ";

        let sensors: Vec<Sensor> = Sensor::expect_from_lines(example).collect();
        let n = count_unavailable_in_row(&sensors, 10);
        assert_eq!(n, 26);
    }
}