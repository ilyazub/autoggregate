#![deny(rust_2018_idioms, warnings)]

use actix_web::{get, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};
use soup::prelude::*;

use std::{fs, string::String};

async fn get(url: &str) -> Result<String, reqwest::Error> {
    let res = reqwest::get(url).await?;

    assert_eq!(200, res.status());

    res.text().await
}

#[derive(Debug, Clone, Serialize)]
pub struct OrganicResult {
    link: String,
    title: String,
    description: Option<String>,
    thumbnail: String,
    updated_at: String,
    // price_uah: f32,
    // price_usd: f32,
    is_paid: bool,
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

fn parse_rst(html: &str) -> Vec<OrganicResult> {
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
                is_paid: organic_result_node
                    .get("class")
                    .expect("Couldn't get 'class' attribute")
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
                        .tag("img")
                        .find()
                        .expect("Couldn't find 'img' node")
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

                description: Some(
                    organic_result_node
                        .class("rst-ocb-i-d-d")
                        .find()
                        .expect("Couldn't find 'description' node")
                        .text()
                        .to_owned(),
                ),

                updated_at: organic_result_node
                    .class("rst-ocb-i-s")
                    .find()
                    .expect("Couldn't find 'updated_at' node")
                    .text()
                    .to_owned(),

                name: organic_result_node
                        .class("rst-ocb-i-i")
                        .find()
                        .expect("Couldn't find 'thumbnail' node")
                        .tag("img")
                        .find()
                        .expect("Couldn't find 'img' node")
                        .get("alt")
                        .expect("Couldn't get thumbnail 'alt'")
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

fn parse_ria(html: &str) -> Vec<OrganicResult> {
    let soup = Soup::new(html);

    soup.class("ticket-item")
        .tag("section")
        .find_all()
        .map(|organic_result_node| OrganicResult {
            link: organic_result_node
                .class("address")
                .find()
                .expect("Couldn't find 'link' node")
                .get("href")
                .expect("Couldn't get 'href'"),

            title: organic_result_node
                .class("address")
                .find()
                .expect("Couldn't find 'link' node")
                .get("title")
                .expect("Couldn't get 'title'"),

            thumbnail: match organic_result_node
                .class("ticket-photo")
                .find()
                .expect("Couldn't find 'thumbnail' node")
                .tag("picture")
                .find()
            {
                Some(picture_node) => picture_node
                    .tag("source")
                    .find()
                    .expect("Couldn't find 'source' node")
                    .get("srcSet")
                    .expect("Couldn't find 'srcSet'"),
                None => organic_result_node
                    .class("ticket-photo")
                    .find()
                    .expect("Couldn't find 'thumbnail' node")
                    .tag("img")
                    .find()
                    .unwrap()
                    .get("src")
                    .unwrap(),
            },

            description: match organic_result_node.tag("p").find() {
                Some(description_node) => Some(
                    description_node
                        .tag("span")
                        .find()
                        .expect("Couldn't find 'span' node")
                        .text(),
                ),
                None => None,
            },

            name: organic_result_node
                .attr_name("data-advertisement-data")
                .find()
                .expect("Couldn't find element with an 'id'")
                .get("data-mark-name")
                .expect("Couldn't find 'mark'"),

            is_paid: false,
            updated_at: "updated_at".to_owned(),
        })
        .collect::<Vec<OrganicResult>>()
}

pub async fn crawl_rst(make: &String) -> Result<Vec<OrganicResult>, Error> {
    let html: String = crawl(
        make,
        String::from("/tmp/rst"),
        &format!(
            "{origin}/oldcars/{make}/?results=4",
            origin = "https://rst.ua",
            make = make
        ),
    )
    .await;

    Ok(parse_rst(&html))
}

async fn crawl(make: &String, tmp_base_path: String, url: &String) -> String {
    let filename_string = format!(
        "{tmp_base_path}/{make}.html",
        tmp_base_path = tmp_base_path,
        make = make
    );
    let filename = filename_string.as_str();

    match fs::read_to_string(filename) {
        Ok(file) => file,
        Err(_) => {
            let html: String = get(url)
                .await
                .expect(&format!("Can't download HTML from url: {}", url));

            let html_to_write: &str = &html;
            match fs::write(filename, html_to_write) {
                Ok(_) => {}
                Err(_) => {
                    fs::create_dir(&tmp_base_path)
                        .expect(&format!("Can't create {} folder.", &tmp_base_path));

                    fs::write(filename, html_to_write)
                        .expect(&format!("Unable to write file: '{}'", filename));
                }
            }

            html
        }
    }
}

pub async fn crawl_ria(make: &String) -> Result<Vec<OrganicResult>, Error> {
    let html: String = crawl(
        make,
        String::from("/tmp/ria"),
        &format!(
            "{origin}/car/{make}",
            origin = "https://auto.ria.com",
            make = make
        ),
    )
    .await;

    Ok(parse_ria(&html))
}

#[derive(Debug, Deserialize)]
struct CarRequest {
    make: String,
}

#[get("/cars/{make}.json")]
async fn get_cars_by_make(car: web::Path<CarRequest>) -> Result<HttpResponse, Error> {
    let response = [crawl_rst(&car.make).await?, crawl_ria(&car.make).await?].concat();

    Ok(HttpResponse::Ok().json(response))
}
