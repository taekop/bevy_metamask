use std::marker::PhantomData;

use bevy::{ecs::event::Event, prelude::*};

mod provider;
mod request;
mod requests;
mod resources;
mod systems;

pub use request::{MetamaskRequest, MetamaskResponse};
pub use requests::*;
use resources::MetamaskResponseEventReceiver;
use systems::{on_metamask_request, on_metamask_response};

pub trait MetamaskRequestEvent: Event {
    type ResponseEvent: Event;

    fn to_req(&self) -> MetamaskRequest;
    fn from_resp(resp: MetamaskResponse) -> Self::ResponseEvent;
}

pub struct MetamaskPlugin<T: MetamaskRequestEvent> {
    _marker: PhantomData<T>,
}

impl<T: MetamaskRequestEvent> MetamaskPlugin<T> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T: MetamaskRequestEvent> Default for MetamaskPlugin<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: MetamaskRequestEvent> Plugin for MetamaskPlugin<T> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(MetamaskResponseEventReceiver::<T::ResponseEvent>::new())
            .add_event::<T>()
            .add_event::<T::ResponseEvent>()
            .add_system(on_metamask_request::<T>)
            .add_system(on_metamask_response::<T>);
    }
}
