use async_std::fs;
use async_std::task;
use async_fsw::{Watcher, WatchMode, Event};

#[async_std::test]
async fn observes_path() {
    let mut w = Watcher::new();
    w.set_path("/tmp", WatchMode::Recursive);
    w.observe().await.unwrap();

    task::spawn(async move {
        fs::write("/tmp/foo.txt", b"Lorem ipsum").await.unwrap();
    });
    
    let mut event: Option<Event> = None;
    while let Some(e) = w.incomming().recv().await {
        event = Some(e);
        break;
    }

    assert_eq!(event.is_some(), true);
}
