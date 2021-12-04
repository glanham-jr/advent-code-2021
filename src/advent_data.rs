use std::error::Error;

pub async fn get_data(day: i32) -> Result<Vec<String>, Box<dyn Error>> {
    let url = format!("https://adventofcode.com/2021/day/{0}/input", day);
    let resp = reqwest::get(url)
        .await?
        .text()
        .await?;

    let vec = resp
        .split("\n")
        .map(str::to_string)
        .collect::<Vec<String>>();

    return Ok(vec);
}

pub async fn get_data_as<T>(day: i32, map: &dyn Fn(String) -> T) -> Result<Vec<T>, Box<dyn Error>> {
    let data = get_data(day)
        .await?;
    return Ok(data
        .into_iter()
        .map(map)
        .collect::<Vec<T>>());
}
