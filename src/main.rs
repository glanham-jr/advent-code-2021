mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {

fn main() {
    println!("day1-p1: {:#?}", day1::p1_sonar());
    println!("day1-p2: {:#?}", day1::p2_sonar());

    println!("day2-p1: {:#?}", day2::p1_pilot());
    println!("day2-p1: {:#?}", day2::p2_pilot());

    println!("day3-p1: {:#?}", day3::p1_diagnostic());
    println!("day3-p2: {:#?}", day3::p2_diagnostic());

    println!("day4-p1: {:#?}", day4::p1_bingo());
    println!("day4-p2: {:#?}", day4::p2_bingo());

    println!("day5-p1: {:#?}", day5::p1_hydrothermal_vents());
    println!("day5-p2: {:#?}", day5::p2_hydrothermal_vents());

    println!("day6-p1: {:#?}", day6::p1_lanternfish());
    println!("day6-p2: {:#?}", day6::p2_lanternfish());
}
