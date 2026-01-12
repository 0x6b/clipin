# stdin_or_clipboard

Get text from stdin or clipboard.

## Usage

Enable exactly one feature: `async` or `sync`. Returns trimmed text and optionally the clipboard instance (clipboard is `None` when stdin is used).

### Async

```sh
cargo add stdin_or_clipboard --features async
```

```rust
use stdin_or_clipboard::get_text_from_stdin_or_clipboard;

#[tokio::main]
async fn main() {
    let (text, clipboard) = get_text_from_stdin_or_clipboard().await.unwrap();
    println!("{text}");
}
```

### Sync

```sh
cargo add stdin_or_clipboard --features sync
```

```rust
use stdin_or_clipboard::get_text_from_stdin_or_clipboard;

fn main() {
    let (text, clipboard) = get_text_from_stdin_or_clipboard().unwrap();
    println!("{text}");
}
```

## License

MIT. See [LICENSE](LICENSE) for details.
