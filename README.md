# TinyIO

A tiny Rust concurrency runtime library.

## Example

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
