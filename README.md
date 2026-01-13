# clipin

A Rust library to get text from clipboard or stdin.

## Usage

Enable exactly one feature: `async` or `sync`. Returns trimmed text and optionally the clipboard instance if available.

### Async

```sh
cargo add clipin --features async
```

```rust
#[tokio::main]
async fn main() {
    let (text, clipboard) = clipin::get().await.unwrap();
    println!("{text}");
}
```

### Sync

```sh
cargo add clipin --features sync
```

```rust
fn main() {
    let (text, clipboard) = clipin::get().unwrap();
    println!("{text}");
}
```

## License

MIT. See [LICENSE](LICENSE) for details.
