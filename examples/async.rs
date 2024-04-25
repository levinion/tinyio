#[tinyio::main]
async fn main() {
    for i in 0..10 {
        tinyio::spawn(async move {
            println!("{}", i);
        })
    }
}
