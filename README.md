# stdin_or_clipboard

Get a text from the stdin or a clipboard.

I often found myself writing code to retrieve text from the clipboard or stdin, so I created this small crate to make my life easier. However, you might want to consider using a more robust option available out here and there.

## Usage

Note: returned text is trimmed (leading/trailing whitespace removed). When stdin is used,
`clipboard` is `None`.

Note: `async` and `sync` are mutually exclusive; enable exactly one.

### Asynchronously

Enable the `async` feature explicitly.

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

### Synchronously

Enable the `sync` feature explicitly.

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
