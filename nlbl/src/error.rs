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
}

pub type Result<T> = core::result::Result<T, Error>;
