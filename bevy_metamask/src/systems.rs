use bevy::{prelude::*, tasks::IoTaskPool};
use futures::SinkExt;

use crate::{
    request::{request, MetamaskRequest},
    resources::MetamaskResponseEventReceiver,
    MetamaskRequestEvent,
};

pub fn on_metamask_request<T: MetamaskRequestEvent>(
    mut req_event_reader: EventReader<T>,
    receiver: Res<MetamaskResponseEventReceiver<T::ResponseEvent>>,
) {
    for ev in req_event_reader.iter() {
        let req = ev.to_req();
        let mut tx = receiver.tx.clone();
        let task_pool = IoTaskPool::get();
        task_pool
            .spawn_local(async move {
                let result = match req {
                    MetamaskRequest::BlockNumber(req) => request(req).await,
                    MetamaskRequest::GetBalance(req) => request(req).await,
                    MetamaskRequest::RequestAccounts(req) => request(req).await,
                    MetamaskRequest::Call(req) => request(req).await,
                    MetamaskRequest::SendTransaction(req) => request(req).await,
                };
                match result {
                    Ok(resp) => {
                        let resp_ev = T::from_resp(resp);
                        if let Err(err) = tx.send(resp_ev).await {
                            bevy::log::error!("Metamask response failed to send: {err:?}");
                        }
                    }
                    Err(err) => {
                        bevy::log::error!("Metamask request failed: {err:?}");
                    }
                }
            })
            .detach();
    }
}

pub fn on_metamask_response<T: MetamaskRequestEvent>(
    mut receiver: ResMut<MetamaskResponseEventReceiver<T::ResponseEvent>>,
    mut resp_event_writer: EventWriter<T::ResponseEvent>,
) {
    while let Ok(Some(ev)) = receiver.rx.try_next() {
        resp_event_writer.send(ev);
    }
}
