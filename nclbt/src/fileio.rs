use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

use nlbl::Budget;

//version 1 with initial release
const SAVE_FORMAT_VERSION: u8 = 0;

#[derive(Serialize, Deserialize, Debug)]
struct SaveFormat {
    app: String,
    version: String,
    save_format: u8,
    data: Vec<u8>,
}
impl SaveFormat {
    fn new(account: &str) -> Self {
        SaveFormat {
            app: env!("CARGO_PKG_NAME").into(),
            version: env!("CARGO_PKG_VERSION").into(),
            save_format: SAVE_FORMAT_VERSION,
            data: Self::budget_into_bytes(Budget::new(account)),
        }
    }
    fn save(budget: Budget) -> Self {
        SaveFormat {
            app: env!("CARGO_PKG_NAME").into(),
            version: env!("CARGO_PKG_VERSION").into(),
            save_format: SAVE_FORMAT_VERSION,
            data: Self::budget_into_bytes(budget),
        }
    }
    fn bytes(&self) -> Vec<u8> {
        bson::to_raw_document_buf(&self)
            .unwrap_or_else(|err| {
                panic!("SaveFormat struct failed to serialize to BSON doc, why? {err}\n{self:?}")
            })
            .into_bytes()
    }
    fn into_bytes(self) -> Vec<u8> {
        bson::to_raw_document_buf(&self)
            .unwrap_or_else(|err| {
                panic!("SaveFormat struct failed to serialize to BSON doc, why? {err}\n{self:?}")
            })
            .into_bytes()
    }
    fn budget_into_bytes(budget: Budget) -> Vec<u8> {
        bson::to_raw_document_buf(&budget)
            .unwrap_or_else(|err| {
                panic!("Budget struct failed to serialize to BSON doc, why? {err}\n{budget:?}")
            })
            .into_bytes()
    }
}

// -- LOADING --

//take in an account name and return the appropriate Budget object
pub fn load_budget_account(account: &str) -> Result<Budget> {
    let file = read_account_file(account)?;
    let save = access_account_save_from_file(account, file)?;

    //TODO: sanity check, check app name, format version, etc

    Ok(
        bson::from_slice(&save.data).map_err(|e| Error::SaveBinaryCorrupted {
            account: account.into(),
            cause: e,
        })?,
    )
}

//returns SaveFormat object from given account's save file
fn access_account_save_from_file(account: &str, mut file: File) -> Result<SaveFormat> {
    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(
        bson::from_slice(&bytes).map_err(|e| Error::SaveBinaryCorrupted {
            account: account.into(),
            cause: e,
        })?,
    )
}

//returns File for given account
fn read_account_file(account: &str) -> Result<File> {
    let file_name = format!("data/{account}.bson");

    #[cfg(debug_assertions)]
    println!("[DEV] opening file {file_name}");

    Ok(File::open(file_name)?)
}

// -- SAVING --

//write a budget out to the account's save file
pub fn save_budget_to_account_file(budget: Budget) -> Result<()> {
    let account = &budget.account;
    let mut file = open_account_file_for_editing(account)?;

    let save_bytes = SaveFormat::save(budget).into_bytes();

    file.set_len(0)?;
    file.write_all(save_bytes.as_slice())?;
    file.flush()?;
    Ok(())
}

// -- CREATION --

//take in an account name, and create a save file, returning the new initialized Budget object
pub fn create_new_budget_account(account: &str) -> Result<Budget> {
    let mut file = make_account_file(account)?;

    let save = SaveFormat::new(account);
    file.write_all(save.bytes().as_slice())?;
    file.flush()?;

    Ok(
        bson::from_slice(&save.data).map_err(|e| Error::SaveBinaryCorrupted {
            account: account.into(),
            cause: e,
        })?,
    )
}

//creates and returns an empty save file
pub fn make_account_file(account: &str) -> Result<File> {
    Ok(File::create(format!("data/{account}.bson"))?)
}

//creates and returns a new initialized save file
pub fn make_new_account_file(account: &str) -> Result<File> {
    let mut file = make_account_file(account)?;
    file.write_all(&SaveFormat::new(account).into_bytes())?;
    file.flush()?;
    Ok(file)
}

// -- EDITING --

//
pub fn change_account_display_name(account: &str, new_name: &str) -> Result<()> {
    let mut bud = load_budget_account(account)?;
    bud.account = String::from(new_name);
    save_budget_to_account_file(bud)?;

    Ok(())
}

pub fn change_default_account_display_name(new_name: &str) -> Result<()> {
    change_account_display_name("default", new_name)
}

fn open_account_file_for_editing(account: &str) -> Result<File> {
    Ok(OpenOptions::new()
        .read(true)
        .write(true)
        .open(format!("data/{account}.bson"))?)
}

// -- APP DIR --

//move active directory into application directory in system-dependent user data dir
pub fn relocate_to_application_dir() -> Result<()> {
    //get OS-dependent user data dir and append package folder
    let working_dir = dirs::data_local_dir()
        .unwrap_or_else(|| panic!("Unsupported Operating System"))
        .join(env!("CARGO_PKG_NAME"));

    #[cfg(debug_assertions)]
    println!("[DEV] working in: {working_dir:?}");

    //if the app directory does not exist, create it!
    if !working_dir.is_dir() {
        generate_application_dir(&working_dir)?
    }

    //move there
    Ok(std::env::set_current_dir(working_dir)?)
}

//creates application dir and subdirs
fn generate_application_dir(path: &Path) -> Result<()> {
    fs::create_dir_all(path)?;
    Ok(fs::create_dir(path.join("data"))?)
}
