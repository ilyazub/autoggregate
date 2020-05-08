extern crate reqwest;
// extern crate select;

// use select::document::Document;
// use select::predicate::{Class, Name, Predicate};

use scraper::{Html, Selector};

#[derive(Debug)]
struct OrganicResult {
    title: String,
    thumbnail: String,
}

async fn get(url: String) -> Result<std::string::String, reqwest::Error> {
    let res = reqwest::get(&url).await?;

    println!("Status: {}", res.status());

    res.text().await
}

fn parse(html: String) -> Vec<OrganicResult> {
    let fragment = Html::parse_fragment(&html);

    let organic_results_selector = Selector::parse("#rst-page-1 .rst-uix-radius").unwrap();
    let organic_results = fragment.select(&organic_results_selector);

    let image_selector = Selector::parse(".rst-ocb-i-i").unwrap();
    let title_selector = Selector::parse(".rst-ocb-i-h span").unwrap();

    let parsed: Vec<OrganicResult> = organic_results.map(|organic_result_node| {
        let image_node = organic_result_node.select(&image_selector).next().unwrap();
        let thumbnail = format!("https:{}", image_node.value().attr("src").unwrap().to_string());

        let title_node = organic_result_node.select(&title_selector).next().unwrap();
        let title = title_node.text().nth(0).unwrap().to_string();

        OrganicResult{ title: title, thumbnail: thumbnail }
    }).collect::<Vec<OrganicResult>>();

    parsed
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url: String = String::from("https://rst.ua/oldcars/?task=newresults&make%5B%5D=0&price%5B%5D=0&price%5B%5D=0&year%5B%5D=0&year%5B%5D=0&condition=0&engine%5B%5D=0&engine%5B%5D=0&fuel=0&gear=0&drive=0&results=1&saled=0&notcust=&sort=1&city=0&from=sform&body%5B%5D=4");

    let html = get(url).await?;

    let result = parse(html);

    println!("{:?}", result);

    Ok(())
}
