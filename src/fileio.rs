use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use thiserror::Error;

use crate::budget::Budget;

#[derive(Serialize, Deserialize, Debug)]
struct SaveFormat {
    app: String,
    version: String,
    save_format: u8,
    data: Vec<u8>,
}
impl SaveFormat {
    fn new(account: &str) -> Self {
        let new_budget = Budget::new(account);
        SaveFormat {
            app: env!("CARGO_PKG_NAME").into(),
            version: env!("CARGO_PKG_VERSION").into(),
            save_format: 1,
            data: bson::to_raw_document_buf(&Budget::new(account))
                .unwrap_or_else(|err| {
                    panic!(
                    "New Budget struct failed to serialize to BSON doc, why? {err}\n{new_budget:?}"
                )
                })
                .into_bytes(),
        }
    }
    fn into_bytes(self) -> Vec<u8> {
        bson::to_raw_document_buf(&self)
            .unwrap_or_else(|err| {
                panic!("SaveFormat struct failed to serialize to BSON doc, why? {err}\n{self:?}")
            })
            .into_bytes()
    }
}

#[derive(Error, Debug)]
pub enum LoadErr {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    DataError(#[from] bson::de::Error),
}
#[derive(Error, Debug)]
pub enum SaveErr {}

//move active directory into application directory in system-dependent user data dir
pub fn relocate_to_application_dir() -> Result<(), io::Error> {
    //get OS-dependent user data dir and append package folder
    let working_dir = dirs::data_local_dir()
        .unwrap_or_else(|| panic!("Unsupported Operating System"))
        .join(env!("CARGO_PKG_NAME"));

    #[cfg(debug_assertions)]
    println!("{working_dir:?}");

    //if the app directory does not exist, create it!
    if !working_dir.is_dir() {
        generate_application_dir(&working_dir)?
    }

    //move there
    std::env::set_current_dir(working_dir)
}

//creates application dir and subdirs
fn generate_application_dir(path: &Path) -> Result<(), io::Error> {
    fs::create_dir_all(path)?;
    fs::create_dir(path.join("data"))
}

//only returns a file if it exists, user must intentionally opt-in to making a new account
fn access_account_save_from_file(account: &str) -> Result<SaveFormat, LoadErr> {
    Ok(bson::from_slice(
        &fs::read(format!("data/{account}.bson"))?.to_vec(),
    )?)
}

//take in an account name and return the appropriate Budget object
pub fn load_budget_account(account: Option<&str>) -> Result<Budget, LoadErr> {
    let account = account.unwrap_or("default");
    let save = access_account_save_from_file(account)?;

    todo!("loading")
}

//write a budget out to the account's save file
pub fn save_budget_to_account_file(budget: Budget) -> Result<(), io::Error> {
    todo!("saving")
}

//creates and returns a new file
pub fn make_account_file(account: &str) -> Result<(), io::Error> {
    File::create_new(format!("data/{account}.bson"))?
        .write_all(&SaveFormat::new(account).into_bytes())
}
