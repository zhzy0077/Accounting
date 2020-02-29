use shared::{Account, LoginChallenge, Operation};

use crate::error::ClientError;
use crate::log;
use serde::de::DeserializeOwned;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

pub async fn accounts() -> Result<Vec<Account>, ClientError> {
    let mut opts = RequestInit::new();
    opts.method("GET");

    let request = Request::new_with_str_and_init("/account", &opts)?;
    let window = web_sys::window().ok_or(ClientError::UnexpectedError)?;
    let resp = JsFuture::from(window.fetch_with_request(&request)).await?;

    deserialize_into(resp).await
}

pub async fn transfer(operation: Operation) -> Result<(), ClientError> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    let operation_str = serde_json::to_string(&operation)?;
    opts.body(Some(&JsValue::from_str(&operation_str)));

    log(&operation_str);

    let request = Request::new_with_str_and_init("/operation", &opts)?;

    request.headers().set("Content-Type", "application/json")?;

    let window = web_sys::window().ok_or(ClientError::UnexpectedError)?;
    let resp = JsFuture::from(window.fetch_with_request(&request)).await?;

    let resp: Response = resp.dyn_into()?;
    if resp.status() < 400 {
        Ok(())
    } else {
        Err(ClientError::InternalError(resp.status_text()))
    }
}

pub async fn login(token: String) -> Result<(), ClientError> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    let login_str = serde_json::to_string(&LoginChallenge { token })?;
    opts.body(Some(&JsValue::from_str(&login_str)));

    let request = Request::new_with_str_and_init("/login", &opts)?;

    request.headers().set("Content-Type", "application/json")?;

    let window = web_sys::window().ok_or(ClientError::UnexpectedError)?;
    let resp = JsFuture::from(window.fetch_with_request(&request)).await?;

    let resp: Response = resp.dyn_into()?;
    if resp.status() < 400 {
        Ok(())
    } else {
        Err(ClientError::ServerError(resp.status_text()))
    }
}


pub async fn add_account(account: Account) -> Result<(), ClientError> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    let account_str = serde_json::to_string(&account)?;
    opts.body(Some(&JsValue::from_str(&account_str)));

    let request = Request::new_with_str_and_init("/account", &opts)?;

    request.headers().set("Content-Type", "application/json")?;

    let window = web_sys::window().ok_or(ClientError::UnexpectedError)?;
    let resp = JsFuture::from(window.fetch_with_request(&request)).await?;

    let resp: Response = resp.dyn_into()?;
    if resp.status() < 400 {
        Ok(())
    } else {
        Err(ClientError::ServerError(resp.status_text()))
    }
}

async fn deserialize_into<T>(resp_value: JsValue) -> Result<T, ClientError>
where
    T: DeserializeOwned,
{
    let resp: Response = resp_value.dyn_into()?;

    if resp.status() >= 400 {
        let server_resp = JsFuture::from(resp.text()?).await?;
        return Err(ClientError::ServerError(format!(
            "{:?}",
            server_resp.as_string()
        )));
    }

    let json = JsFuture::from(resp.json()?).await?;

    json.into_serde().map_err(ClientError::from)
}
