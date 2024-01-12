use async_fsw::{WatchMode, Watcher};
use async_std::fs;
use async_std::task;

#[async_std::test]
async fn observes_path() {
    let mut w = Watcher::new();
    w.set_path("/tmp", WatchMode::Recursive);
    w.observe().await.unwrap();

    task::spawn(async move {
        fs::write("/tmp/foo.txt", b"Lorem ipsum").await.unwrap();
    });

    let mut event = None;
    while let Ok(e) = w.incomming().recv().await {
        event = Some(e);
        break;
    }

    assert!(event.is_some());
}
