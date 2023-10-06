extern crate reqwest;

use std::collections::HashMap;
use reqwest::{Client, Error, Response};
use reqwest::header::HeaderMap;

fn make_body(source: String, target: String, texts: String, folder: String) -> HashMap< String, String >
{
    return HashMap::from([
        ("sourceLanguageCode".to_string(), source),
        ("targetLanguageCode".to_string(), target),
        ("texts".to_string(), texts),
        ("folderId".to_string(), folder)
    ]);
}

fn make_headers(token: String) -> HeaderMap
{
    let auth = format!("Bearer {}", token);
    let mut map: HeaderMap = HeaderMap::new();
    map.insert("Content-Type", "application/json".parse().unwrap());
    map.insert("Authorization", auth.parse().unwrap());
    return map;
}

pub async fn make_request(source: String, target: String, texts: String) -> Result< Response, Error >
{
    let iam_token = "".to_string();
    let folder_id = "".to_string();
    let request_url: String = "https://translate.api.cloud.yandex.net/translate/v2/translate".to_string();

    let request_body: HashMap< String, String > = make_body(source, target, texts, folder_id);
    let request_headers: HeaderMap = make_headers(iam_token);

    let client: Client = Client::new();
    let response = client
        .post(request_url)
        .json(&request_body)
        .headers(request_headers)
        .send().await;

    return response;
}
