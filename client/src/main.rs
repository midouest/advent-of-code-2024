use std::sync::Arc;

use clap::Parser;

/// Fetch the puzzle input from adventofcode.com for the given day
#[derive(Parser, Debug)]
struct Args {
    /// Day to fetch puzzle input for (1-25)
    day: u8,

    #[arg(short, long, default_value = "2024")]
    year: u16,

    /// The value of your "session" cookie on adventofcode.com
    #[arg(short, long, env = "AOC_TOKEN")]
    token: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let cookie_url = "https://adventofcode.com".parse::<reqwest::Url>()?;
    let token = args.token;
    let cookie = format!("session={token}");
    let cookie_jar = reqwest::cookie::Jar::default();
    cookie_jar.add_cookie_str(&cookie, &cookie_url);
    let client = reqwest::Client::builder()
        .cookie_provider(Arc::new(cookie_jar))
        .build()?;

    let year = args.year;
    let day = args.day;
    let input_url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let resp = client.get(input_url).send().await?.text().await?;
    print!("{resp}");
    Ok(())
}
