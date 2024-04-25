use isahc::AsyncReadResponseExt;

fn main() {
    let (executor, spawner) = tinyio::init();
    spawner.spawn({
        async move {
            let mut res = isahc::get_async(
                "https://raw.githubusercontent.com/levinion/tinyio/main/README.md",
            )
            .await
            .unwrap();
            println!("{}", res.text().await.unwrap());
        }
    });
    drop(spawner);
    executor.run();
}
