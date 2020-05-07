use scraper::{Html, Selector};

#[derive(Debug)]
struct OrganicResult {
    thumbnail: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url: String = String::from("https://rst.ua/oldcars/?task=newresults&make%5B%5D=0&price%5B%5D=0&price%5B%5D=0&year%5B%5D=0&year%5B%5D=0&condition=0&engine%5B%5D=0&engine%5B%5D=0&fuel=0&gear=0&drive=0&results=1&saled=0&notcust=&sort=1&city=0&from=sform&body%5B%5D=4");

    let res = reqwest::get(&url).await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;

    let fragment = Html::parse_fragment(&body);

    let organic_results_selector = Selector::parse("#rst-page-1 .rst-uix-radius").unwrap();
    let organic_results = fragment.select(&organic_results_selector);

    let image_selector = Selector::parse(".rst-ocb-i-i").unwrap();

    let mut result: Vec<OrganicResult> = vec![];

    for organic_result_node in organic_results {
        let image_node = organic_result_node.select(&image_selector).next().unwrap();
        let thumbnail = format!("https:{}", image_node.value().attr("src").unwrap().to_string());

        result.push(OrganicResult{ thumbnail: thumbnail });
    }

    println!("{:?}", result);

    Ok(())
}
