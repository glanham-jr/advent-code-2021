
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::hash_map::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
struct Point {
    x: u32,
    y: u32,
}


fn get_line_diagnol(v1: Point, v2: Point) -> Option<Vec<Point>> {
    fn generate_points_x(p1: Point, p2: Point) -> Vec<Point> {
        let mut acc: Vec<Point> = Vec::with_capacity((p1.x as i32 - p2.x as i32).abs() as usize);
        let mut iter = p1;
        while iter.x < p2.x {
            acc.push(iter);
            iter.x += 1;
        }
        acc.push(p2);
        return acc;
    }

    fn generate_points_y(p1: Point, p2: Point) -> Vec<Point> {
        let mut acc: Vec<Point> = Vec::with_capacity((p1.x as i32 - p2.x as i32).abs() as usize);
        let mut iter = p1;
        while iter.y < p2.y {
            acc.push(iter);
            iter.y += 1;
        }
        acc.push(p2);
        return acc;
    }

    fn generate_points_xy(p1: Point,
                          p2: Point,
                          dx: &dyn Fn(u32) -> u32,
                          dy: &dyn Fn(u32) -> u32) -> Vec<Point> {
        let mut acc: Vec<Point> = Vec::with_capacity((p1.x as i32 - p2.x as i32).abs() as usize);
        let mut iter = p1;
        while iter.y != p2.y {
            acc.push(iter);
            iter.x = dx(iter.x);
            iter.y = dy(iter.y);
        }
        acc.push(p2);
        return acc;
    }

    if v1.x == v2.x {
        if v1.y < v2.y {
            return Some(generate_points_y(v1, v2));
        }
        return Some(generate_points_y(v2, v1));
    }
    else if v1.y == v2.y {
        if v1.x < v2.x {
            return Some(generate_points_x(v1, v2));
        }
        return Some(generate_points_x(v2, v1));
    }
    else { // if (v1.y as i32 - v2.y as i32).abs() == (v1.x as i32 - v2.x as i32) {
        let dx = if v1.x > v2.x { |x| x - 1 } else { |x| x + 1 };
        let dy = if v1.y > v2.y { |y| y - 1 } else { |y| y + 1 };
        return Some(generate_points_xy(v1, v2, &dx, &dy));
    }
}

fn get_line(v1: Point, v2: Point) -> Option<Vec<Point>> {
    fn generate_points_x(p1: Point, p2: Point) -> Vec<Point> {
        let mut acc: Vec<Point> = Vec::with_capacity((p1.x as i32 - p2.x as i32).abs() as usize);
        let mut iter = p1;
        while iter.x < p2.x {
            acc.push(iter);
            iter.x += 1;
        }
        acc.push(p2);
        return acc;
    }

    fn generate_points_y(p1: Point, p2: Point) -> Vec<Point> {
        let mut acc: Vec<Point> = Vec::with_capacity((p1.x as i32 - p2.x as i32).abs() as usize);
        let mut iter = p1;
        while iter.y < p2.y {
            acc.push(iter);
            iter.y += 1;
        }
        acc.push(iter);
        return acc;
    }

    if v1.x == v2.x {
        if v1.y < v2.y {
            return Some(generate_points_y(v1, v2));
        }
        return Some(generate_points_y(v2, v1));
    }
    else if v1.y == v2.y {
        if v1.x < v2.x {
            return Some(generate_points_x(v1, v2));
        }
        return Some(generate_points_x(v2, v1));
    }
    return None;
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vector {
    v1: Point,
    v2: Point,
	line: Vec<Point>,
}
impl Vector {
    pub fn new_line_only(v1: Point, v2: Point) -> Option<Self> {
        return get_line(v1, v2).map(|line| {
            Vector {
                v1,
                v2,
                line,
            }
        });
    }

    pub fn new_line_diagnol(v1: Point, v2: Point) -> Option<Self> {
        return get_line_diagnol(v1, v2).map(|line| {
            Vector {
                v1,
                v2,
                line,
            }
        });
    }
}

fn parse_line(line: Result<String, std::io::Error>, line_only: bool) -> Option<Vector> {
    fn inner(line: String, line_only: bool) -> Option<Vector> {
        // Split by space, ignore arrow, split points by comma
        // 629,581 -> 123,75
        // [629,581] [->] [123,75]
        let split_line = line.split(" ").into_iter().collect::<Vec<&str>>();

        let v1_line = split_line[0]
            .split(",")
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let v2_line = split_line[2]
            .split(",")
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let v1 = Point {
            x: v1_line[0],
            y: v1_line[1],
        };
        let v2 = Point {
            x: v2_line[0],
            y: v2_line[1],
        };

        if line_only {
            return Vector::new_line_only(v1, v2);
        }
        return Vector::new_line_diagnol(v1, v2);
    }

    return match line {
        Ok(l) => inner(l, line_only),
        Err(e) => panic!("Failed to read line: {:?}", e),
    };
}

fn read_file() -> BufReader<File> {
    let file_path = "data/day5_hydrothermal_vents.txt";
    let file =  match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("Failed to open file: {:?}", e),
    };
    return BufReader::new(file);
}

fn get_hydrothermal_vents(line_only: bool) -> Vec<Vector> {
    return read_file()
        .lines()
        .map(|l| parse_line(l, line_only))
        .filter(|v| v.is_some())
        .map(|v| v.unwrap())
        .collect();
}

fn _hydrothermal_vents(vents: Vec<Vector>) -> usize {
    let mut hash = HashMap::new();
    let mut max = 0;
    for v in vents.iter() {
        for p in v.line.iter() {
            let count = hash.entry(p).or_insert(0);
            *count += 1;
            if *count > max {
                max = *count;
            }
        }
    }
    return hash.values()
               .filter(|v| **v > 1)
               .count();
}

pub fn p1_hydrothermal_vents() -> usize {
    let vents = get_hydrothermal_vents(true);
    let max = _hydrothermal_vents(vents);
    return max;
}

pub fn p2_hydrothermal_vents() -> usize {
    // 11499 too low
    let vents = get_hydrothermal_vents(false);
    let max = _hydrothermal_vents(vents);
    return max;
}
