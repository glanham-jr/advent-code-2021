mod day1;
mod day2;
mod day3;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("day1-p1: {:#?}", day1::p1_sonar());
    println!("day1-p2: {:#?}", day1::p2_sonar());

    println!("day2-p1: {:#?}", day2::p1_pilot());
    println!("day2-p1: {:#?}", day2::p2_pilot());

    println!("day3-p1: {:#?}", day3::p1_diagnostic());
    println!("day3-p2: {:#?}", day3::p2_diagnostic());
    Ok(())
}
