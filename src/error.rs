#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("file system i/o error: {0}")]
    IoFailure(#[from] std::io::Error),
    #[error("corrupted save file for {account}\n{cause}")]
    SaveBinaryCorrupted {
        account: String,
        #[source]
        cause: bson::de::Error,
    },
    #[error("not a save file: {file}")]
    SaveFormatMismatch { file: String },
    #[error("dollar value error: {0:?}")]
    InvalidDollarValue(String),
    #[error("invalid command error: ")]
    InvalidCommand(String),
    #[error(
        "budget account error: cannot afford {expense} ({amount}) with balance {remaining_balance}"
    )]
    BudgetErrorCannotAfford {
        expense: String,
        amount: i32,
        remaining_balance: i32,
    },
    #[error("budget account error: expense {0} does not exist")]
    BudgetErrorExpenseDoesNotExist(String),
}

pub type Result<T> = core::result::Result<T, Error>;
