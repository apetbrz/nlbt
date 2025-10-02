#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("dollar value error: {0:?}")]
    InvalidDollarValue(String),

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
}

pub type Result<T> = core::result::Result<T, Error>;
