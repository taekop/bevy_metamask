use js_sys::Function;
use js_sys::Promise;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone)]
pub struct Provider {
    pub this: JsValue,
    pub request: Function,
}

impl Provider {
    pub fn new() -> Self {
        let provider = web_sys::window().unwrap().get("ethereum").unwrap();
        let request = js_sys::Reflect::get(&provider, &JsValue::from("request")).unwrap();
        Provider {
            this: JsValue::from(provider),
            request: Function::from(request),
        }
    }

    /// Request must have two fields: method (String), params (Option<JsValue>)
    pub async fn async_request(self, req: JsValue) -> Result<JsValue, JsValue> {
        let ret = self.request.call1(&self.this, &req);
        match ret {
            Ok(s) => {
                let promise = Promise::resolve(&s);
                Ok(wasm_bindgen_futures::JsFuture::from(promise).await?)
            }
            Err(e) => Err(e),
        }
    }
}
