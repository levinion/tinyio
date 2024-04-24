use std::time::Duration;

fn main() {
    let spawner = tinyio::init();
    for i in 0..10 {
        spawner.spawn(move || {
            println!("{}", i);
        });
    }
    std::thread::sleep(Duration::from_secs(5));
}
