use error_chain::error_chain;
use std::io::Read;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = get_request();
    post_request().await?;
    Ok(())
}

async fn post_request() -> Result<()> {
    let res = reqwest::get("https://httpbin.org/get").await?;
    let body = String::new();
    res.text().await?;
    println!("{:#?}", body);

    Ok(())
}

fn get_request() -> Result<()> {
    let mut res = reqwest::blocking::get("https://httpbin.org/get")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers: {:#?}\n", res);
    println!("Body: {}", body);

    Ok(())
}
