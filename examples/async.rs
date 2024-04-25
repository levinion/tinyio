async fn task() {
    for i in 0..10 {
        println!("{}", i);
    }
}

fn main() {
    let (executor, spawner) = tinyio::init();
    spawner.spawn({
        let spawner = spawner.clone();
        async move {
            task().await;
            spawner.spawn(async move { task().await })
        }
    });
    drop(spawner);
    executor.run();
}
