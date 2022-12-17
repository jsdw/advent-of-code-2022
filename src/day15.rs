use super::File;
use std::collections::HashSet;

pub fn star1(file: File) -> Result<usize, anyhow::Error> {
    let sensors: Vec<Sensor> = Sensor::expect_from_lines(&file.contents).collect();

    const ROW: i64 = 2_000_000;
    let taken_ranges = taken_ranges_in_row(&sensors, ROW);

    // The less lazy way would be to merge any overlapping ranges and then count
    // the sizes of remaining disjoint ones.
    let mut taken_locations: HashSet<i64> = taken_ranges.flat_map(|r| r.0..=r.1).collect();

    // dont count any beacons in the locations that are covered:
    for beacon_in_row in sensors.iter().filter(|s| s.beacon_y == ROW) {
        taken_locations.remove(&beacon_in_row.beacon_y);
    }

    Ok(taken_locations.len())
}

pub fn star2(file: File) -> Result<i64, anyhow::Error> {
    let sensors: Vec<Sensor> = Sensor::expect_from_lines(&file.contents).collect();

    const MAX: i64 = 4_000_000;
    for y in 0 ..= MAX {
        if let Some(x) = find_free_spot_in_row(&sensors, y, MAX) {
            return Ok(x * MAX + y)
        }
    }

    anyhow::bail!("Could not find any free location for the beacon");
}

fn find_free_spot_in_row(sensors: &[Sensor], row: i64, max: i64) -> Option<i64> {
    let mut taken_ranges: Vec<(i64,i64)> = taken_ranges_in_row(sensors, row).collect();
    taken_ranges.sort_by_key(|(start,_end)| *start);

    // Assuming max 1 free spot in range, we just work through our claimed sensor spots
    // and see whether any of the possible X values before `max` are free, skipping over
    // any values that sensors can see.
    let mut x = 0;
    for (start, end) in taken_ranges {
        if start > x {
            return Some(x)
        }
        if x < end {
            x = end + 1;
        }
        if x > max {
            break
        }
    }
    None
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

struct Sensor {
    x: i64,
    y: i64,
    beacon_x: i64,
    beacon_y: i64
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
