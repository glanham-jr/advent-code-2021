use std::fs::File;
use std::io::{BufReader, BufRead};

fn div_remain(num: usize, div: usize) -> (usize, usize) {
    return (num / div, num % div);
}

struct Diagnostic {
    bit_count: Vec<u32>, // contains count for each bit
    bit_opers: Vec<u16>, // precalcluate 1 << x
    values: Vec<u16>,
    bits_end: usize, // total number of bits to enumerate
    bits_start: usize, // total number of bits to enumerate
    alpha_mask: u16,
}
impl Diagnostic {
    pub fn new(bits_start: usize, bits_end: usize) -> Self {
        let bit_opers: Vec<u16> = (0..(bits_end as u16)).map(|x| 1 << x).collect::<Vec<u16>>();
        // assert!(bits == bit_opers.len());
        let alpha_mask = bit_opers
            .iter()
            .fold(0x0, |mut acc, a| {
                acc |= a;
                return acc;
            });
        return Diagnostic {
            bit_count: vec![0; bits_end],
            values: Vec::with_capacity(bits_end),
            bits_end,
            bits_start,
            bit_opers,
            alpha_mask,
        };
    }

    // Only processes a single bit (for life_support)
    fn partial(diagnostic: &Diagnostic, new_values: Vec<u16>, single_bit: usize) -> Self {
        let mut d = Diagnostic {
            bit_count: vec![0; diagnostic.bits_end],// we always have to allocate from 0.. bit-end to ensure indexing works
            bit_opers: diagnostic.bit_opers.to_owned(),
            values: Vec::with_capacity(new_values.len()),
            bits_end: single_bit + 1, //
            bits_start: single_bit,
            alpha_mask: diagnostic.alpha_mask,

        };

        new_values.into_iter().for_each(|v| {
            d.process(&v);
        });

        return d;
    }

    pub fn process(&mut self, line: &u16) {
        for index in self.bits_start..self.bits_end {
            let bit_oper = self.bit_opers[index];
            if line & bit_oper == bit_oper {
                self.bit_count[index] += 1;
            }
        }
        self.values.push(*line);
    }

    pub fn get_gamma(&self) -> u16 {
        let total = self.values.len();
        let (div, rem) = div_remain(total, 2);
        let half_total = (div + rem) as u32;
        return (self.bits_start..self.bits_end)
            .fold(0x0, |mut acc, index| {
                let bit_oper = self.bit_opers[index];
                let cnt = self.bit_count[index];
                if cnt >= half_total {
                    acc |= bit_oper;
                }
                return acc;
            });
    }

    pub fn get_alpha_gamma(&self) -> (u16, u16) {
        let gamma = self.get_gamma();
        let alpha: u16 = self.alpha_mask & !gamma;
        return (alpha, gamma);
    }

    pub fn power_comsumption(&self) -> u32 {
        let (alpha, gamma) = self.get_alpha_gamma();
        return (alpha as u32) * (gamma as u32);
    }

    fn _life_support_filter(&self, index: usize, is_upper: bool) -> Vec<u16> {
        let (res, rem) = div_remain(self.values.len(), 2);
        let half_total = res + rem;
        let oper = self.bit_opers[index];
        let count = self.bit_count[index];
        let high_valid: bool = count >= (half_total as u32);
        let take_high: bool = if is_upper { high_valid } else { !high_valid };
        // 111110
        let fstrings: Vec<String> = self.values.iter().map(|x| format!("{:012b}", x)).collect();
        if !is_upper {
            println!("Count {:?} Bit {:?}, TakeHigh: {:?}, HalfLen: {:?}, values: {:?}", count, self.bits_end - 1, take_high, half_total, fstrings);
        }
        if take_high {
            return self.values
                       .iter()
                       .map(|x| *x)
                       .filter(|x| x & oper == oper)
                       .collect();
        }
        else {
            return self.values
                       .iter()
                       .map(|x| *x)
                       .filter(|x| (x & oper) != oper)
                       .collect();
        }
    }

    fn _life_support(&self, high_low_tup: (Vec<u16>, Vec<u16>), index: usize) -> (Vec<u16>, Vec<u16>) {
        let high_len = high_low_tup.0.len();
        let low_len = high_low_tup.1.len();
        // println!("Index: {:?}, High: {:?}, Low: {:?}", index, high_len, low_len);

        // exit early if both conditions were met
        if high_low_tup.0.len() <= 1 && high_low_tup.1.len() <= 1 {
            return high_low_tup;
        }
        else {
            // Reprocess bit count
            // It does calculate it for every bit, but the logic is still the same
            let du: Vec<u16> = if high_len <= 1 {
                high_low_tup.0
            } else {
                Diagnostic::partial(self, high_low_tup.0, index)._life_support_filter(index, true)
            };

            let dl: Vec<u16> = if low_len <= 1 {
                high_low_tup.1
            } else {
                Diagnostic::partial(self, high_low_tup.1, index)._life_support_filter(index, false)
            };

            if index == 0 {
                return (du, dl);
            }

            // 2715525 is too low
            return self._life_support((du, dl), index - 1);
        }
    }

    pub fn life_support(&self) -> u32 {
        // We needed to do a first pass to calculate the number
        // of bits in the first number, so the previous algo is still valid
        println!("BitCount: {:?}", self.bit_count);

        let (div, rem) = div_remain(self.values.len(), 2);
        let half_total: usize = div + rem;
        let high_valid: bool = self.bit_count[self.bits_end - 1] >= (half_total as u32);
        let mut high: Vec<u16> = Vec::with_capacity(half_total);
        let mut low: Vec<u16> = Vec::with_capacity(half_total);
        let bit_oper = self.bit_opers[self.bits_end - 1];

        // we can go from 2*O(N) to O(N) by initialize vlaues outside of fn inner
        // Since both sets start with the same vector
        println!("Bit {:?}: {:?}", self.bits_end - 1, high_valid);
        self.values.iter().for_each(|v| {
            if v & bit_oper == bit_oper && high_valid {
                high.push(*v);
            }
            else {
                low.push(*v);
            }
        });

        let (high_fin, low_fin) = self._life_support((high, low), self.bits_end - 2);

        assert!(high_fin.len() == 1);
        assert!(low_fin.len() == 1);
        println!("High: {:012b}, Low: {:012b}", high_fin[0], low_fin[0]);
        println!("High: {:?}, Low: {:?}", high_fin[0], low_fin[0]);
        // correct: 2775870
        return (high_fin[0] as u32) * (low_fin[0] as u32);
    }
}

fn parse_line(line: Result<String, std::io::Error>) -> u16 {
    fn inner(line: String) -> u16 {
        let line_len = line.len();
        let value = line.chars()
                        .into_iter()
                        .enumerate()
                        .fold(0x0, |mut acc, (i, c)| {
                            if c == '1' {
                                // we techincally are in "reverse" order
                                // for parsing, so we need to do line_len - 1
                                acc |= 1 << (line_len - i - 1);
                            }
                            return acc;
                        });

        // Test the converted value
        let test_value = format!("{:012b}", value);
        if !(line == test_value){
            panic!("Line {:?} does not equal Value {:?}", line, test_value);
        }
        return value;
    }

    return match line {
        Ok(l) => inner(l),
        Err(e) => panic!("Failed to read line: {:?}", e),
    };
}

fn read_file() -> BufReader<File> {
    let file_path = "data/day3_diagnostic.txt";
    let file =  match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("Failed to open file: {:?}", e),
    };
    return BufReader::new(file);
}

fn get_diagnostic(only_process_last_bit: bool) -> Diagnostic {
    let line_length: usize = 12;
    return read_file()
        .lines()
        .map(parse_line)
        .fold(Diagnostic::new(if only_process_last_bit { line_length - 1 } else { 0 }, line_length), |mut acc, d| {
            acc.process(&d);
            return acc;
        });
}
pub fn p1_diagnostic() -> u32 {
    return get_diagnostic(false).power_comsumption();
}

pub fn p2_diagnostic() -> u32 {
    return get_diagnostic(true).life_support();
}
