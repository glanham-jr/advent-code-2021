use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Forward,
    Down,
    Up,
}

struct CommandValue {
    command: Command,
    value: i32,
}

fn parse_command(cmd: &str) -> Command {
    return match cmd {
        "forward" => Command::Forward,
        "down" => Command::Down,
        "up" => Command::Up,
        _ => panic!("Invalid command: {:?}", cmd),
    }
}

struct PilotPosition {
    pub depth: i32,
    pub position: i32,
    pub aim: i32,
}
impl PilotPosition {
    pub fn mult(&self) -> i32 {
        return self.depth * self.position;
    }
}

fn parse_line(line: Result<String, std::io::Error>) -> CommandValue {
    fn inner(line: String) -> CommandValue {
        let split: Vec<_> = line.split(" ").collect();
        return match split[1].parse::<i32>() {
            Ok(num) => CommandValue {
                command: parse_command(split[0]),
                value: num,
            },
            Err(e) => panic!("Failed to parse as int: {:?}", e),
        };
    }

    return match line {
        Ok(l) => inner(l),
        Err(e) => panic!("Failed to read line: {:?}", e),
    };
}

fn read_file() -> BufReader<File> {
    let file_path = "data/day2_pilot.txt";
    let file =  match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("Failed to open file: {:?}", e),
    };
    return BufReader::new(file);
}

pub fn p1_pilot() -> i32 {
    return read_file()
        .lines()
        .map(parse_line)
        .fold(PilotPosition { depth: 0, position: 0, aim: 0  }, |mut acc, n| {
            if n.command == Command::Forward {
                acc.position += n.value;
            }
            else if n.command == Command::Up {
                acc.depth -= n.value;
            }
            else { // Command::Down
                acc.depth += n.value;
            }
            return acc;
        }).mult();
}

pub fn p2_pilot() -> i32 {
    return read_file()
        .lines()
        .map(parse_line)
        .fold(PilotPosition { depth: 0, position: 0, aim: 0  }, |mut acc, n| {
            if n.command == Command::Forward {
                acc.position += n.value;
                acc.depth += n.value * acc.aim;
            }
            else if n.command == Command::Up {
                acc.aim -= n.value;
            }
            else { // Command::Down
                acc.aim += n.value;
            }
            return acc;
        }).mult();
}
