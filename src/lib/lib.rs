pub mod budget;
pub mod error;
pub mod util;

use error::{Error, Result};

pub type Budget = budget::Budget;

#[derive(Debug)]
pub enum BudgetCommand {
    SetPaycheck {
        amount: i32,
    },
    Paid {
        //None = full paycheck
        amount: Option<i32>,
    },
    ClearExpense {
        targets: Vec<String>,
        invert_selection: bool,
    },
    EditExpense {
        target: String,
        new_name: Option<String>,
        new_amount: Option<i32>,
    },
    NewExpense {
        name: String,
        amount: i32,
    },
    PayExpense {
        name: String,
        //None = full amount
        amount: Option<i32>,
    },
    Nothing,
}

pub type BudgetCommands = Vec<BudgetCommand>;

pub fn parse_command(input: &str) -> Result<BudgetCommand> {
    use BudgetCommand as BC;

    let command = input.split_whitespace();
    let command: Vec<&str> = command.collect();

    let cmd = match *command.first().unwrap_or(&"") {
        "help" => {
            help();
            BC::Nothing
        }
        "income" => match *command.get(1).unwrap_or(&"") {
            "set" => {
                let amount = *command.get(2).ok_or(Error::InvalidCommand("new".into()))?;
                let amount = util::parse_dollar_string(amount)?;
                BC::SetPaycheck { amount }
            }
            "raise" => {
                todo!("raise income command!");
                let amount = *command.get(2).ok_or(Error::InvalidCommand("new".into()))?;
                let amount = util::parse_dollar_string(amount)?;
            }
            other => Err(Error::InvalidCommand(other.into()))?,
        },
        "paid" => match *command.get(1).unwrap_or(&"") {
            "" => BC::Paid { amount: None },
            val => {
                let amount = util::parse_dollar_string(val)?;
                BC::Paid {
                    amount: Some(amount),
                }
            }
        },
        "new" => {
            let name = String::from(*command.get(1).ok_or(Error::InvalidCommand("new".into()))?);
            let amount = *command.get(2).ok_or(Error::InvalidCommand("new".into()))?;
            let amount = util::parse_dollar_string(amount)?;
            BC::NewExpense { name, amount }
        }
        "pay" => {
            let name = String::from(*command.get(1).ok_or(Error::InvalidCommand("pay".into()))?);
            match *command.get(2).unwrap_or(&"") {
                "" => BC::PayExpense { name, amount: None },
                val => {
                    let amount = util::parse_dollar_string(val)?;
                    BC::PayExpense {
                        name,
                        amount: Some(amount),
                    }
                }
            }
        }
        "save" => {
            todo!("savings!");
            // match command[1] {
            // "" => Err(Error::InvalidCommand("empty save amount".into()))?,
            // "all" => bud.save_all(),
            // _ => {
            //     let amount = util::parse_dollar_string(command[1])?;
            //     bud.save(amount)
            // }
        }
        // "clear" => match term.clear_screen() {
        //     Ok(()) => Ok(String::new()),
        //     Err(e) => Err(Error::IoFailure(e)),
        // },
        cmd => return Err(Error::InvalidCommand(cmd.into()))?,
    };

    Ok(cmd)
}
