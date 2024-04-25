use isahc::AsyncReadResponseExt;

#[tinyio::main]
async fn main() {
    let mut res =
        isahc::get_async("https://raw.githubusercontent.com/levinion/tinyio/main/README.md")
            .await
            .unwrap();
    println!("{}", res.text().await.unwrap());
}
