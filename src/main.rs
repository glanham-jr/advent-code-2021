mod day1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("day1-a: {:#?}", day1::sonar());
    Ok(())
}
