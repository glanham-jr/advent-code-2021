mod day1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("day1-p1: {:#?}", day1::p1_sonar());
    println!("day1-p2: {:#?}", day1::p2_sonar());
    Ok(())
}
