use std::fs::File;
use std::io::{BufReader, BufRead};

fn parse_line(line: Result<String, std::io::Error>) -> Vec<u32> {
    fn inner(line: String) -> Vec<u32> {
        return line.split(",")
                   .map(|l| l.parse::<u32>().unwrap())
                   .into_iter()
                   .collect::<Vec<u32>>();
    }

    return match line {
        Ok(l) => inner(l),
        Err(e) => panic!("Failed to read line: {:?}", e),
    };
}

fn read_file() -> BufReader<File> {
    let file_path = "data/day7_whale.txt";
    let file =  match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("Failed to open file: {:?}", e),
    };
    return BufReader::new(file);
}

fn get_positions() -> Vec<u32> {
    let result = read_file()
        .lines()
        .map(parse_line)
        .collect::<Vec<Vec<u32>>>();
    return result[0].to_vec();
}

fn brute_force(values: &mut Vec<u32>) -> u32 {
    values.sort();
    let max = values.last().unwrap();

    let result = (0..*max).step_by(1).fold((u32::MAX, u32::MAX), |acc, a| {
        if a == acc.0 {
            return acc;
        }

        let sum = values.iter().fold(0, |sum, b| {
            if a > *b {
                return sum + a - b;
            }
            return sum + b - a;
        });

        if sum < acc.1 {
            return (a, sum);
        }
        return (a, acc.1);
    });

    return result.1;

}

fn crabby_formula(delta: u32) -> u32 {
    // X -> Y
    // --------
    // 0 -> 0 ==  2 * 0
    // 1 -> 1 ==  1 * 1
    // 2 -> 3 ==  2 * 1.5 == 2 * (1 + ((2 - 1) * 0.5)
    // 3 -> 6 ==  3 * 2   == 2 * (1 + ((3 - 1) * 0.5)
    // 4 -> 10 == 4 * 2.5 == 2 * (1 + ((4 - 1) * 0.5)
    // 5 -> 15 == 5 * 3   == 2 * (1 + ((5 - 1) * 0.5)
    // F(x) =  2 * (x + ((x - 1) * 0.5)
    // AKA sigma summation

    return (delta * (delta + 1)) / 2;
}

fn brute_force_p2(values: &mut Vec<u32>) -> u32 {
    values.sort();
    let max = values.last().unwrap();

    let result = (0..*max).step_by(1).fold((u32::MAX, u32::MAX), |acc, a| {
        if a == acc.0 {
            return acc;
        }

        let sum = values.iter().fold(0, |sum, b| {
            if a > *b {
                return sum + crabby_formula(a - *b);
            }
            return sum + crabby_formula(*b - a);
        });

        if sum < acc.1 {
            return (a, sum);
        }
        return (a, acc.1);
    });

    return result.1;

}

pub fn p1_whale() -> u32 {
    let mut positions = get_positions();
    let result = brute_force(&mut positions);
    return result;
}

pub fn p2_whale() -> u32 {
    let mut positions = get_positions();
    let result = brute_force_p2(&mut positions);
    return result;
}
