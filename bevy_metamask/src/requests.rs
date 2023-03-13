use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;

use crate::request::{MetamaskRequestTrait, MetamaskResponse};

#[derive(Debug, Clone)]
pub struct BlockNumber;

impl MetamaskRequestTrait for BlockNumber {
    type Response = String;

    fn resp_into_event(resp: Self::Response) -> MetamaskResponse {
        MetamaskResponse::BlockNumber(resp)
    }

    fn to_js_value(&self) -> JsValue {
        let obj = Object::new();
        Reflect::set(&obj, &"method".into(), &"eth_blockNumber".into()).unwrap();
        obj.into()
    }
}

#[derive(Debug, Clone)]
pub struct GetBalance {
    pub data: String,
}

impl MetamaskRequestTrait for GetBalance {
    type Response = String;

    fn resp_into_event(resp: Self::Response) -> MetamaskResponse {
        MetamaskResponse::GetBalance(resp)
    }

    fn to_js_value(&self) -> JsValue {
        let params = js_sys::Array::new();
        params.push(&self.data.clone().into());
        params.push(&"latest".into());
        let obj = Object::new();
        Reflect::set(&obj, &"method".into(), &"eth_getBalance".into()).unwrap();
        Reflect::set(&obj, &"params".into(), &params.into()).unwrap();
        obj.into()
    }
}

#[derive(Debug, Clone)]
pub struct RequestAccounts;

impl MetamaskRequestTrait for RequestAccounts {
    type Response = Vec<String>;

    fn resp_into_event(resp: Self::Response) -> MetamaskResponse {
        MetamaskResponse::RequestAccounts(resp)
    }

    fn to_js_value(&self) -> JsValue {
        let obj = Object::new();
        Reflect::set(&obj, &"method".into(), &"eth_requestAccounts".into()).unwrap();
        obj.into()
    }
}

#[derive(Debug, Clone)]
pub struct Call {
    pub from: Option<String>,
    pub to: String,
    pub gas: Option<String>,
    pub gas_price: Option<String>,
    pub value: Option<String>,
    pub data: Option<String>,
}

impl MetamaskRequestTrait for Call {
    type Response = String;

    fn resp_into_event(resp: Self::Response) -> MetamaskResponse {
        MetamaskResponse::Call(resp)
    }

    fn to_js_value(&self) -> JsValue {
        let params = js_sys::Array::new();
        let arg0 = {
            let obj = Object::new();
            Reflect::set(&obj, &"from".into(), &self.from.clone().into()).unwrap();
            Reflect::set(&obj, &"to".into(), &self.to.clone().into()).unwrap();
            Reflect::set(&obj, &"gas".into(), &self.gas.clone().into()).unwrap();
            Reflect::set(&obj, &"gasPrice".into(), &self.gas_price.clone().into()).unwrap();
            Reflect::set(&obj, &"value".into(), &self.value.clone().into()).unwrap();
            Reflect::set(&obj, &"data".into(), &self.data.clone().into()).unwrap();
            obj
        };
        params.push(&arg0.into());
        params.push(&"latest".into());
        let obj = Object::new();
        Reflect::set(&obj, &"method".into(), &"eth_call".into()).unwrap();
        Reflect::set(&obj, &"params".into(), &params).unwrap();
        obj.into()
    }
}

#[derive(Debug, Clone)]
pub struct SendTransaction {
    pub from: String,
    pub to: Option<String>,
    pub gas: Option<String>,
    pub gas_price: Option<String>,
    pub value: Option<String>,
    pub data: String,
    pub nonce: Option<String>,
}

impl MetamaskRequestTrait for SendTransaction {
    type Response = String;

    fn to_js_value(&self) -> JsValue {
        let params = js_sys::Array::new();
        let arg0 = {
            let obj = Object::new();
            Reflect::set(&obj, &"from".into(), &self.from.clone().into()).unwrap();
            Reflect::set(&obj, &"to".into(), &self.to.clone().into()).unwrap();
            Reflect::set(&obj, &"gas".into(), &self.gas.clone().into()).unwrap();
            Reflect::set(&obj, &"gasPrice".into(), &self.gas_price.clone().into()).unwrap();
            Reflect::set(&obj, &"value".into(), &self.value.clone().into()).unwrap();
            Reflect::set(&obj, &"data".into(), &self.data.clone().into()).unwrap();
            Reflect::set(&obj, &"nonce".into(), &self.nonce.clone().into()).unwrap();
            obj
        };
        params.push(&arg0.into());
        let obj = Object::new();
        Reflect::set(&obj, &"method".into(), &"eth_sendTransaction".into()).unwrap();
        Reflect::set(&obj, &"params".into(), &params).unwrap();
        obj.into()
    }

    fn resp_into_event(resp: Self::Response) -> MetamaskResponse {
        MetamaskResponse::SendTransaction(resp)
    }
}
