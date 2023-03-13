use serde::Deserialize;
use wasm_bindgen::JsValue;

use crate::{provider::Provider, *};

#[derive(Debug, Clone)]
pub enum MetamaskRequest {
    BlockNumber(BlockNumber),
    GetBalance(GetBalance),
    RequestAccounts(RequestAccounts),
    Call(Call),
    SendTransaction(SendTransaction),
}

#[derive(Debug, Clone)]
pub enum MetamaskResponse {
    BlockNumber(String),
    GetBalance(String),
    RequestAccounts(Vec<String>),
    Call(String),
    SendTransaction(String),
}

pub trait MetamaskRequestTrait {
    type Response: 'static + for<'a> Deserialize<'a>;

    fn to_js_value(&self) -> JsValue;
    fn resp_into_event(resp: Self::Response) -> MetamaskResponse;
}

pub async fn request<R: 'static + MetamaskRequestTrait + std::marker::Send>(
    req: R,
) -> Result<MetamaskResponse, JsValue> {
    let result = Provider::new().async_request(req.to_js_value()).await;
    match result {
        Ok(resp) => match serde_wasm_bindgen::from_value(resp) {
            Ok(resp) => Ok(R::resp_into_event(resp)),
            Err(err) => Err(err.into()),
        },
        Err(err) => Err(err),
    }
}
