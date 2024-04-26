# TinyIO

A tiny Rust concurrency runtime library.

## Example

### Print in parallel

```rust
#[tinyio::main]
async fn main() {
    for i in 0..10 {
        tinyio::spawn(async move {
            println!("{}", i);
        })
    }
}
```

### Make HTTP/HTTPS requests

```rust
#[tinyio::main]
async fn main() {
    let mut res =
        isahc::get_async("https://raw.githubusercontent.com/levinion/tinyio/main/README.md")
            .await
            .unwrap();
    println!("{}", res.text().await.unwrap());
}
```
