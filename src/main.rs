use regex::Regex;
use reqwest::header::USER_AGENT;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let re = Regex::new("filters-toolbar__product-count\">(.*?) p")?;

    let client = reqwest::Client::builder().build()?;

    let mut total_product_count: u8 = 76;

    loop {
        let response = client
            .get("https://eu.gear.blizzard.com/collections/diablo")
            .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36")
            .send()
            .await?
            .text()
            .await?;

        let product_count = &re.captures(&response).unwrap()[1];
        let product_count: u8 = product_count.parse()?;

        println!("{:?}", product_count);

        if product_count > total_product_count {
            println!("******************************************");
            println!("******************************************");
            println!("******************************************");
            println!("*********** MIRA LA WEB YA !! ************");
            println!("******************************************");
            println!("******************************************");
            println!("******************************************");

            total_product_count = product_count;
        }

        sleep(Duration::from_secs(60)).await;
    }
}
