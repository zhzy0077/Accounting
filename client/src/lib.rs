use crate::error::ClientError;
use js_sys::Date;
use entities::{Account, Operation};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement, HtmlInputElement, HtmlSelectElement, Window};

mod client;
mod error;

macro_rules! get_element_by_id {
    ( $x:expr, $t:ty ) => {{
        get_by_id($x)
            .dyn_ref::<$t>()
            .expect(&format!("{} is not an element.", $x))
    }};
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_owned(s: String);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_js_value(s: JsValue);
}

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    match init_accounts().await {
        Ok(_) => log("Assembly initialized."),
        Err(e) => log_owned(format!("{:?}", e)),
    }

    Ok(())
}

pub async fn init_accounts() -> Result<(), ClientError> {
    let accounts = client::accounts().await?;
    refresh_content(&accounts).await?;

    let options = accounts
        .iter()
        .enumerate()
        .map(|(idx, account)| {
            format!(
                r#"
                    <option value="{}" {}>{}</option>
                "#,
                account.name,
                if idx == 0 { "selected" } else { "" },
                account.name
            )
        })
        .fold("".to_owned(), |x, y| x + &y);

    get_element_by_id!("from_account", HtmlElement).set_inner_html(&options);

    get_element_by_id!("to_account", HtmlElement).set_inner_html(&options);

    Ok(())
}

#[wasm_bindgen]
pub async fn do_transfer() -> Result<(), JsValue> {
    let operation = make_operation()?;
    client::transfer(operation).await?;
    alert("Success");

    Ok(())
}

fn make_operation() -> Result<Operation, ClientError> {
    let from = get_element_by_id!("from_account", HtmlSelectElement).value();
    let to = get_element_by_id!("to_account", HtmlSelectElement).value();
    let amount: f64 = get_element_by_id!("amount", HtmlInputElement)
        .value()
        .parse()?;
    let comment = get_element_by_id!("comment", HtmlInputElement).value();
    let datetime = match get_element_by_id!("date", HtmlInputElement).value() {
        date_str if !date_str.is_empty() => Date::parse(&date_str),
        _ => Date::new_0().value_of(),
    } as i64;

    Ok(Operation {
        from,
        to,
        amount: (amount * 100.0) as i64,
        comment,
        datetime,
    })
}

#[wasm_bindgen]
pub async fn update_accounts() -> Result<(), JsValue> {
    let accounts = client::accounts().await?;
    refresh_content(&accounts).await?;

    Ok(())
}

#[wasm_bindgen]
pub async fn add_account() -> Result<(), JsValue> {
    let name = get_element_by_id!("account_name", HtmlInputElement).value();
    client::add_account(Account { name, balance: 0 }).await?;
    alert("Success");

    Ok(())
}

#[wasm_bindgen]
pub async fn login() -> Result<(), JsValue> {
    let code = get_element_by_id!("code", HtmlInputElement).value();
    match client::login(code).await {
        Ok(_) => {
            alert("Success");
            redirect("/")?;
        }
        Err(e) => {
            alert(&format!("{:?}", e));
        }
    }

    Ok(())
}

async fn refresh_content(accounts: &Vec<Account>) -> Result<(), ClientError> {
    let html = accounts
        .iter()
        .enumerate()
        .map(|(idx, account)| {
            format!(
                r#"
                    <tr>
                        <td scope="row">{}</td>
                        <td>{}</td>
                        <td>{:.2}</td>
                    </tr>
            "#,
                idx + 1,
                account.name,
                (account.balance as f64) / 100.0
            )
        })
        .fold("".to_owned(), |x, y| x + &y);

    get_element_by_id!("balance_content", HtmlElement).set_inner_html(&html);

    Ok(())
}

fn get_by_id(id: &str) -> Element {
    fn get_by_id_option(id: &str) -> Option<Element> {
        let document: Document = web_sys::window()?.document()?;
        document.get_element_by_id(id)
    }
    get_by_id_option(id).expect(&format!("{} not found.", id))
}

fn redirect(href: &str) -> Result<(), JsValue> {
    let window: Window = web_sys::window().ok_or(ClientError::UnexpectedError)?;
    let location = window.location();
    location.set_href(href)?;

    Ok(())
}
