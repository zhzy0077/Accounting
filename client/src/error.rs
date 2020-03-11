use std::num::ParseFloatError;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum ClientError {
    InternalError(String),
    UnexpectedError,
    DomError,
    InputError(String),
    ServerError(String),
}

impl From<wasm_bindgen::JsValue> for ClientError {
    fn from(_: wasm_bindgen::JsValue) -> Self {
        ClientError::InternalError("Js error.".to_owned())
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(_: serde_json::Error) -> Self {
        ClientError::InternalError("Json parse error.".to_owned())
    }
}

impl From<ClientError> for wasm_bindgen::JsValue {
    fn from(e: ClientError) -> Self {
        wasm_bindgen::JsValue::from(format!("{:?}", e))
    }
}

impl From<ParseIntError> for ClientError {
    fn from(e: ParseIntError) -> Self {
        ClientError::InputError(format!("Wrong number: {}", e))
    }
}

impl From<ParseFloatError> for ClientError {
    fn from(e: ParseFloatError) -> Self {
        ClientError::InputError(format!("Wrong number: {}", e))
    }
}
