use std::fs::File;
use std::io::{BufReader, BufRead};

struct SonarAccumulator {
    pub count: i32,
    pub previous_value: i32,
}

fn parse_line(line: Result<String, std::io::Error>) -> i32 {
    return match line {
        Ok(l) => match l.parse::<i32>()  {
            Ok(num) => num,
            Err(e) => panic!("Failed to parse as int: {:?}", e)
        },
        Err(e) => panic!("Failed to read line: {:?}", e)
    }
}

fn read_file() -> BufReader<File> {
    let file_path = "data/day1_sonar.txt";
    let file =  match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("Failed to open file: {:?}", e),
    };
    return BufReader::new(file);
}

pub fn p1_sonar() -> i32 {
    return read_file()
        .lines()
        .map(parse_line)
        .fold(SonarAccumulator{ count: 0, previous_value: i32::MAX }, |mut acc, n| {
            // println!("Prev: {}, Curr: {}", acc.previous_value, n);
            if n > acc.previous_value  {
                acc.count += 0x1;
            }
            acc.previous_value = n;
            return acc;
        }).count;
}

struct SonarSlidingWindow {
    pub lead: i32,
    pub second: i32,
    pub last: i32,
}
impl SonarSlidingWindow {
    fn sum(&self) -> i32 {
        return self.lead + self.second + self.last;
    }
    fn push(&mut self, value: i32)  {
        self.last = self.second;
        self.second = self.lead;
        self.lead = value;
    }
}

struct SonarSlidingWindowAccumulator {
    pub lead: SonarSlidingWindow,
    pub follow: SonarSlidingWindow,
    pub count: i32,
}

pub fn p2_sonar() -> i32 {
    let values = read_file()
        .lines()
        .map(parse_line)
        .collect::<Vec<i32>>();

    if values.len() <= 4 {
        if values.len() <= 3 {
            return 0;
        }
        if values[1] + values[2] + values[3] > values[0] + values[1] + values[2] {
            return 1
        }
        return 0;
    }


    let acc = SonarSlidingWindowAccumulator {
        lead: SonarSlidingWindow {
            last: values[1],
            second: values[2],
            lead: values[3],
        },
        follow: SonarSlidingWindow {
            last: values[0],
            second: values[1],
            lead: values[2],
        },
        count: 0,
    };

    return values
        .into_iter()
        .fold(acc, |mut acc, n| {
            if acc.lead.sum() > acc.follow.sum()  {
                acc.count += 0x1;
            }
            acc.follow.push(acc.lead.lead);
            acc.lead.push(n);
            return acc;
        }).count;
}
