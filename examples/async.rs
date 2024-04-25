async fn task() {
    for i in 0..10 {
        println!("{}", i);
    }
}

#[tinyio::main]
async fn main() {
    task().await;
    tinyio::spawn(async move { task().await })
}
