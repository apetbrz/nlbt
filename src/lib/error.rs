#[cfg(feature = "wasm")]
use wasm_bindgen::JsValue;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("dollar value error: {0:?}")]
    InvalidDollarValue(String),

    #[error("invalid command error: ")]
    InvalidCommand(String),

    #[error(
        "budget account error: cannot afford {expense} ({amount}) with balance {remaining_balance}"
    )]
    CannotAfford {
        expense: String,
        amount: i32,
        remaining_balance: i32,
    },

    #[error("budget account error: expense {0} does not exist")]
    ExpenseDoesNotExist(String),

    #[error("json error: {0}")]
    JsonFailure(#[from] serde_json::Error),

    #[cfg(feature = "wasm")]
    #[error("js object translation error: {0}")]
    WasmObjFailure(#[from] serde_wasm_bindgen::Error),
}

pub type Result<T> = core::result::Result<T, Error>;

#[cfg(feature = "wasm")]
impl From<Error> for JsValue {
    fn from(err: Error) -> JsValue {
        JsValue::from_str(&err.to_string())
    }
}
