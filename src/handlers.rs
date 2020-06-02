#![deny(rust_2018_idioms, warnings)]

use actix_web::{get, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};
use soup::prelude::*;
// use tokio::stream::StreamExt;

use std::{fs, string::String};

#[derive(Debug, Serialize)]
pub struct OrganicResult {
    link: String,
    title: String,
    description: String,
    thumbnail: String,
    updated_at: String,
    // price_uah: f32,
    // price_usd: f32,
    is_premium: bool,
    name: String,
    // year: i8,
    // mileage: i8,

    // fuel: String,
    // volume: f32,
    // gear: String,

    // state: String,
    // condition: String,

    // paid_listing: String,
}

fn parse(html: &str) -> Vec<OrganicResult> {
    let soup = Soup::new(html);

    soup.class("rst-ocb-i")
        .find_all()
        .filter(|organic_result_node| {
            organic_result_node
                .get("id")
                .expect("Couldn't get 'id' of a node")
                != "rst-oc-smaster-block"
        })
        .map(|organic_result_node| {
            OrganicResult {
                is_premium: organic_result_node
                    .get("class")
                    .unwrap()
                    .contains("rst-ocb-i-premium"),
                link: format!(
                    "https://rst.ua{}",
                    organic_result_node
                        .class("rst-ocb-i-a")
                        .find()
                        .expect("Couldn't find 'link' node")
                        .get("href")
                        .expect("Couldn't get 'href'")
                ),

                thumbnail: format!(
                    "https:{}",
                    organic_result_node
                        .class("rst-ocb-i-i")
                        .find()
                        .expect("Couldn't find 'thumbnail' node")
                        .get("src")
                        .expect("Couldn't get thumbnail 'src'")
                ),

                title: organic_result_node
                    .class("rst-ocb-i-h")
                    .find()
                    .expect("Couldn't find 'title wrapper' node")
                    .tag("span")
                    .find()
                    .expect("Couldn't find 'title' node")
                    .text(),

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

                name: organic_result_node
                    .class("rst-ocb-i-i")
                    .find()
                    .unwrap()
                    .get("alt")
                    .unwrap(),
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
            }
        })
        .collect::<Vec<OrganicResult>>()
}

async fn get(url: &str) -> Result<String, reqwest::Error> {
    let res = reqwest::get(url).await?;

    assert_eq!(200, res.status());

    res.text().await
}

pub async fn crawl_rst(make: String) -> Result<Vec<OrganicResult>, Error> {
    let base_folder = "/tmp/rst/";
    let filename_string = format!(
        "{base_folder}{make}.html",
        base_folder = base_folder,
        make = make
    );
    let filename = filename_string.as_str();

    let html: String = match fs::read_to_string(filename) {
        Ok(html) => html,
        Err(_) => {
            let url_string: String = format!(
                "{origin}/oldcars/{make}/",
                origin = "https://rst.ua",
                make = make
            );

            let url: &str = &url_string;

            let html: String = get(url)
                .await
                .expect(&format!("Can't download HTML from url: {}", url));

            let html_to_write: &str = &html;

            match fs::write(filename, html_to_write) {
                Ok(_) => {}
                Err(_) => {
                    fs::create_dir(base_folder)?;

                    fs::write(filename, html_to_write)?;
                }
            };

            html
        }
    };

    Ok(parse(&html))
}

#[derive(Debug, Deserialize)]
struct CarRequest {
    name: String,
}

#[get("/cars/{name}.json")]
async fn get_cars_by_name(car: web::Path<CarRequest>) -> Result<HttpResponse, Error> {
    let response = crawl_rst(car.name.to_string()).await?;

    Ok(HttpResponse::Ok().json(response))
}
