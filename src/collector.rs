use reqwest;

pub async fn get_document(url: String) -> String {
    let client = reqwest::Client::new();
    let body = client.get(url).send().await.unwrap().text().await.unwrap();

    return body;
}
