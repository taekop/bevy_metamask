use bevy::prelude::Resource;
use futures::channel::mpsc::{channel, Receiver, Sender};

#[derive(Resource)]
pub struct MetamaskResponseEventReceiver<T> {
    pub tx: Sender<T>,
    pub rx: Receiver<T>,
}

impl<T> MetamaskResponseEventReceiver<T> {
    pub fn new() -> Self {
        let (tx, rx) = channel(100);
        Self { tx, rx }
    }
}
