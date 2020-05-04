> Cross-platform asynchronous filesystem notification library for Rust.

This library allows for asynchronous observation of filesystem changes. This project is built on top of [async-std](https://async.rs/) an [notify](https://github.com/notify-rs/notify) projects.

## Example

```rs
use async_fsw::{Watcher, WatchMode};

let mut w = Watcher::new();
w.set_path("/tmp", WatchMode::Recursive);
w.observe().await;

while let Some(event) = w.incomming().recv().await {
    println!("Event: {:?}", event);
}
```
