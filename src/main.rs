use crate::request_work::make_request;

mod request_work;

fn main() {
    let source: String = String::from("english");
    let target: String = String::from("russian");
    let texts: String = String::from("cat");

    println!("{}", make_request(source, target, texts));
}
