use crate::json_work::out_translation;
use crate::request_work::make_request;

mod request_work;
mod json_work;

#[tokio::main]
async fn main()
{
    let args: Vec< String > = std::env::args().collect();
    assert_eq!(args.len(), 4);

    let source: String = String::from(&args[1]);
    let target: String = String::from(&args[2]);
    let texts: String = String::from(&args[3]);

    let response = make_request(source, target, texts).await;

    match response
    {
        Err(error) => eprintln!("{}", error),
        Ok(txt) => out_translation(txt).await
    }
}
