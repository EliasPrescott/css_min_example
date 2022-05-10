const INPUT_CSS: &str = "*:hover { display: none !important;}";

#[tokio::main]
async fn main() {
    println!("{:?}", get_minified_css(INPUT_CSS).await);
}

async fn get_minified_css(raw_css: &str) -> Result<String, String> {
    let params = [("input", raw_css)];
    let client = reqwest::Client::new();
    let res = client.post("https://www.toptal.com/developers/cssminifier/raw")
        .form(&params)
        .send()
        .await;
    match res {
        Ok(ok_response) => ok_response.text().await.map_err(|err| err.to_string()),
        Err(err_response) => Err(err_response.to_string())
    }
}