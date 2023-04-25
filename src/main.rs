use reqwest::Error;
use scraper::{Html, Selector};

async fn get_request(link: &str) -> Result<String, Error> {
    let response = reqwest::get(link).await?;
    println!("Status: {}", response.status());

    let body = response.text().await?;
    //println!("Body:\n{}", body);

    Ok(body)
}

fn get_title(html: &str)-> Result<String, Error>{
    let document = Html::parse_document(&html);
    let title_selector = Selector::parse("title").unwrap();
    let title = document.select(&title_selector).next().unwrap().text().collect::<String>();

    Ok(title)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let html = get_request("https://www.example.com").await?;
    let title = get_title(&html);
    println!("Title:{:?}", title);
    Ok(())
}