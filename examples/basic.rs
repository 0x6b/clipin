#[cfg(all(feature = "async", feature = "sync"))]
compile_error!("features \"async\" and \"sync\" are mutually exclusive; enable exactly one.");

#[cfg(not(any(feature = "async", feature = "sync")))]
compile_error!("enable a feature: cargo run --example basic --features async|sync");

#[cfg(feature = "async")]
#[tokio::main]
async fn main() {
    let (text, _clipboard) = stdin_or_clipboard::get().await.unwrap();
    println!("{text}");
}

#[cfg(feature = "sync")]
fn main() {
    let (text, _clipboard) = stdin_or_clipboard::get().unwrap();
    println!("{text}");
}
