use std::fs::File;
use std::io::{BufReader, BufRead};

struct SonarAccumulator {
    pub count: i32,
    pub previous_value: i32,
}

pub fn p1_sonar() -> i32 {
    let file_path = "data/day1_sonar.txt";
    let file =  match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("Failed to open file: {:?}", e),
    };
    let reader = BufReader::new(file);

    let result = reader.lines()
        .map(|line| {
            match line {
                Ok(l) => match l.parse::<i32>()  {
                    Ok(num) => num,
                    Err(e) => panic!("Failed to parse as int: {:?}", e)
                },
                Err(e) => panic!("Failed to read line: {:?}", e)
            }
        })
        .fold(SonarAccumulator{ count: 0, previous_value: i32::MAX }, |mut acc, n| {
            // println!("Prev: {}, Curr: {}", acc.previous_value, n);
            if n > acc.previous_value  {
                acc.count += 0x1;
            }
            acc.previous_value = n;
            return acc;
        });

    return result.count;
}

struct SonarSlidingAccumulator {
    // TODO
}

pub fn p2_sonar() -> i32 {
    // TODO
}
