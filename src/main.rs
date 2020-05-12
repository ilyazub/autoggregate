extern crate reqwest;
extern crate soup;
extern crate regex;

use regex::Regex;
use soup::prelude::*;
use std::string::String;

#[derive(Debug)]
pub struct OrganicResult {
    title: String,
    description: String,
    thumbnail: String,
    updated_at: String,

    // price_uah: f32,
    // price_usd: f32,

    // year: i8,
    // mileage: i8,

    // fuel: String,
    // volume: f32,
    // gear: String,

    // state: String,
    // condition: String,

    // paid_listing: String,
}

async fn get(url: String) -> Result<String, reqwest::Error> {
    let res = reqwest::get(&url).await?;

    println!("Status: {}", res.status());

    res.text().await
}

pub fn parse(html: &str) -> Vec<OrganicResult> {
    let soup = Soup::new(html);

    soup.class("rst-ocb-i")
        .find_all()
        .map(|organic_result_node| OrganicResult {
            thumbnail: format!(
                "https:{}",
                organic_result_node
                    .class("rst-ocb-i-i")
                    .find()
                    .expect("Couldn't find 'thumbnail' node")
                    .get("src")
                    .unwrap()
                    .to_owned()
            ),

            title: organic_result_node
                .class("rst-ocb-i-h")
                .find()
                .expect("Couldn't find 'title wrapper' node")
                .tag("span")
                .find()
                .expect("Couldn't find 'title' node")
                .text()
                .to_owned(),
            description: organic_result_node
                .class("rst-ocb-i-d-d")
                .find()
                .expect("Couldn't find 'description' node")
                .text()
                .to_owned(),
            updated_at: organic_result_node
                .class("rst-ocb-i-s")
                .find()
                .expect("Couldn't find 'updated_at' node")
                .text()
                .to_owned(),

            // price_uah: organic_result_node
            //     .class("rst-ocb-i-d-l-i-s-p")
            //     .find()
            //     .expect("Couldn't find 'price_uah' node")
            //     .text()
            //     .parse::<f32>()
            //     .expect("Couldn't parse 'price_uah'"),
            // price_usd: organic_result_node
            //     .class("rst-uix-grey")
            //     .find()
            //     .expect("Couldn't find 'price_usd' node")
            //     .text()
            //     .parse::<f32>()
            //     .expect("Couldn't parse 'price_usd'"),

            // year: organic_result_node
            //     .class("rst-ocb-i-d-l-i")
            //     .limit(3)
            //     .class("rst-ocb-i-d-l-i-s")
            //     .find()
            //     .expect("Couldn't find 'year' node")
            //     .text()
            //     .parse::<i8>()
            //     .unwrap(),
            // mileage: organic_result_node
            //     .class("rst-ocb-i-d-l-i-s")
            //     .find()
            //     .expect("Couldn't find 'mileage' node")
            //     .text()
            //     .parse::<i8>()
            //     .unwrap(),

            // fuel: "rst-ocb-i-d-l-i-s".to_owned(),
            // volume: r#"//*[@id="rst-ocid-11121411"]/div[1]/ul/li[5]/text()[2]"#
            //     .parse::<f32>()
            //     .unwrap(),
            // gear: ".rst-ocb-i-d-l-i-s+ .rst-ocb-i-d-l-i-s".to_owned(),

            // state: ".rst-ocb-i-d-l-j:nth-child(2) .rst-ocb-i-d-l-i-s".to_owned(),
            // condition: ".rst-ocb-i-d-l-j~ .rst-ocb-i-d-l-j .rst-ocb-i-d-l-i-s".to_owned(),

            // paid_listing: ".rst-ocb-i-s-s".to_owned(),
        })
        .collect::<Vec<OrganicResult>>()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url: String = String::from("https://rst.ua/oldcars/?task=newresults&make%5B%5D=0&price%5B%5D=0&price%5B%5D=0&year%5B%5D=0&year%5B%5D=0&condition=0&engine%5B%5D=0&engine%5B%5D=0&fuel=0&gear=0&drive=0&results=1&saled=0&notcust=&sort=1&city=0&from=sform&body%5B%5D=4");

    let html = get(url).await?;

    let result = parse(&html);

    println!("{:?}", result);

    Ok(())
}
