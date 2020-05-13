use std::env;
use std::string::String;

mod search;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let make: &str = &args[1];

    let result = crate::search::crawl(make).await;

    println!("{:?}", result);

    Ok(())
}
