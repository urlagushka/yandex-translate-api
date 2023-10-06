extern crate reqwest;

use std::collections::HashMap;
use reqwest::Client;

fn make_json(source: String, target: String, texts: String) -> HashMap< String, String >
{
    let req = HashMap::from([
        ("sourceLanguageCode".to_string(), source),
        ("targetLanguageCode".to_string(), target),
        ("texts".to_string(), texts)
    ]);
    return req;
}

pub async fn make_request(source: String, target: String, texts: String) -> String
{
    let request_url: String = "POST https://translate.api.cloud.yandex.net/translate/v2/translate".to_string();
    let request_body: HashMap< String, String > = make_json(source, target, texts);

    let client: Client = Client::new();
    let result = client.post(request_url)
        .json(&request_body)
        .send()
        .await?;

    return result;
}
