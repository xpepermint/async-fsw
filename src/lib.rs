use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use async_std::task;
use async_std::sync::{channel, Sender, Receiver};
use notify::{RecommendedWatcher, Watcher as FsWatcher, RecursiveMode};
pub use notify::event::{Event, EventKind, AccessKind, AccessMode, CreateKind,
    ModifyKind, DataChange, MetadataKind, RenameMode, RemoveKind};

#[derive(Debug)]
pub struct Watcher {
    paths: HashMap<String, WatchMode>,
    channel: (Sender<Event>, Receiver<Event>),
}

impl Watcher {

    pub fn new() -> Self {
        Self {
            paths: HashMap::with_hasher(RandomState::default()),
            channel: channel(1),
        }
    }

    pub fn with_channel(channel: (Sender<Event>, Receiver<Event>)) -> Self {
        Self {
            paths: HashMap::with_hasher(RandomState::default()),
            channel,
        }
    }

    pub fn set_path<S: Into<String>>(&mut self, path: S, mode: WatchMode) {
        self.paths.insert(path.into(), mode);
    }

    pub fn remove_path(&mut self, path: &str) {
        self.paths.remove(path);
    }

    pub fn set_channel(&mut self, sender: Sender<Event>, receiver: Receiver<Event>) {
        self.channel = (sender, receiver);
    }

    pub fn incomming(&self) -> Receiver<Event> {
        self.channel.1.clone()
    }

    pub async fn observe(&self) -> Result<Receiver<Event>, std::io::Error> {
        let paths = self.paths.clone();
        let (sender, receiver) = self.channel.clone();
    
        task::spawn_blocking(move || {
            let (tx, rx) = std::sync::mpsc::channel();
            let mut watcher: RecommendedWatcher = FsWatcher::new_immediate(move |res| { tx.send(res).unwrap() }).unwrap();
            for (path, mode) in paths {
                 watcher.watch(path, match mode {
                    WatchMode::Recursive => RecursiveMode::Recursive,
                    _ => RecursiveMode::NonRecursive,
                }).unwrap();
            }
            for e in rx {
                task::block_on(async { sender.send(e.unwrap()).await });
            }
        });
    
        Ok(receiver)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum WatchMode {
    NonRecursive = 0,
    Recursive = 1,
}
