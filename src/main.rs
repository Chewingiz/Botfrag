use reqwest::Error;

async fn get_request(link: &str) -> Result<(), Error> {
    let response = reqwest::get(link).await?;
    println!("Status: {}", response.status());

    let body = response.text().await?;
    println!("Body:\n{}", body);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    get_request("https://www.example.com").await?;
    Ok(())
}