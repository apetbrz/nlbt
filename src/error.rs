#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("i/o failure: {0}")]
    IoFailure(#[from] std::io::Error),
    #[error("corrupted save file for {account}\n{cause}")]
    SaveBinaryCorrupted {
        account: String,
        #[source]
        cause: bson::de::Error,
    },
    #[error("not a save file: {file}")]
    SaveFormatMismatch { file: String },
    #[error("budget error: {0}")]
    BudgetFailure(#[from] nlbl::error::Error),
}

pub type Result<T> = core::result::Result<T, Error>;
