mod collector;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let s: String = "https://google.com".to_string();
    let res = collector::get_document(s).await;
    println!("{}", res);
}
