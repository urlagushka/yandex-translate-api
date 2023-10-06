use reqwest::Response;

pub async fn out_translation(resp: Response)
{
    let resp_json = serde_json::from_str(resp.text().await.unwrap().as_str()).unwrap();
    println!("{}", resp_json["translations"][0]["text"]);
}
