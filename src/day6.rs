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
    let file_path = "data/day6_lanternfish.txt";
    let file =  match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("Failed to open file: {:?}", e),
    };
    return BufReader::new(file);
}

fn get_lanternfish() -> Vec<u32> {
    let result = read_file()
        .lines()
        .map(parse_line)
        .collect::<Vec<Vec<u32>>>();
    return result[0].to_vec();
}

// fn _lanternfish(mut fish: Vec<u32>, days: usize) -> Vec<u32> {
//     let mut counter = 0;
//     let mut fishes_to_add: usize;
//     while counter < days {
//         fishes_to_add = 0;
//         for f in fish.iter_mut() {
//             if *f == 0 {
//                 fishes_to_add += 1;
//                 *f = 6;
//             }
//             else {
//                 *f -= 1;
//             }
//         }

//         for _ in 0..fishes_to_add {
//             fish.push(8);
//         }
//         counter += 1;
//     }
//     return fish;
// }

fn _lanternfish_fast(fish: &Vec<u32>, days: usize) -> u64 {
    let mut stages: [u64; 9] = [0; 9];

    // initialize the array
    for f in fish.iter() {
        stages[*f as usize] += 1;
    }

    let mut counter = 0;
    while counter < days {
        let new_fish = stages[0];

        for i in 0..8 {
            stages[i] = stages[i + 1];
        }

        stages[6] += new_fish;
        stages[8] = new_fish;
        counter += 1;
    }


    return stages.iter().sum();
}


pub fn p1_lanternfish() -> u64 {
    // 380 too low
    let fishes = get_lanternfish();
    // brute force calc
    return _lanternfish_fast(&fishes, 80);
}

pub fn p2_lanternfish() -> u64 {
    let fishes = get_lanternfish();
    return _lanternfish_fast(&fishes, 256);
}
