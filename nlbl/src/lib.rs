pub mod budget;
pub mod error;
pub mod util;

use error::{Error, Result};
use wasm_bindgen::prelude::*;

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

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, WASM!");
}

#[wasm_bindgen]
pub fn help() {
    todo!("help commands on WASM!")
}

// for use when Budget obj is owned by JS
#[wasm_bindgen]
pub fn apply_string_command(js_bud: JsValue, input: &str) -> JsValue {
    let mut budget: Budget = serde_wasm_bindgen::from_value(js_bud).unwrap_throw();
    let command = parse_command(input).unwrap_throw();
    apply_budget_command(&mut budget, command).unwrap_throw();
    serde_wasm_bindgen::to_value(&budget).unwrap_throw()
}

pub fn parse_command(input: &str) -> Result<BudgetCommand> {
    use BudgetCommand as BC;

    let command = input.split_whitespace();
    let command: Vec<&str> = command.collect();

    let cmd = match command[0] {
        "help" => {
            help();
            BC::Nothing
        }
        "income" => match command[1] {
            "set" => {
                let amount = util::parse_dollar_string(command[2])?;
                BC::SetPaycheck { amount }
            }
            "raise" => {
                let amount = util::parse_dollar_string(command[2])?;
                todo!("raise income command!")
            }
            other => Err(Error::InvalidCommand(other.into()))?,
        },
        "paid" => match command[1] {
            "" => BC::Paid { amount: None },
            _ => {
                let amount = util::parse_dollar_string(command[2])?;
                BC::Paid {
                    amount: Some(amount),
                }
            }
        },
        "new" => {
            let name = command[1].into();
            let amount = util::parse_dollar_string(command[2])?;
            BC::NewExpense { name, amount }
        }
        "pay" => {
            let name = command[1].into();
            match command[2] {
                "" => BC::PayExpense { name, amount: None },
                _ => {
                    let amount = util::parse_dollar_string(command[2])?;
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

pub fn apply_budget_command(budget: &mut Budget, cmd: BudgetCommand) -> Result<()> {
    use BudgetCommand as BC;
    #[cfg(debug_assertions)]
    println!("[DEV] executing command: {cmd:?}");
    match cmd {
        BC::SetPaycheck { amount } => {
            budget.set_income(amount);
        }
        BC::Paid { amount } => {
            if let Some(c) = amount {
                budget.get_paid_value(c);
            } else {
                budget.get_paid();
            }
        }
        BC::ClearExpense {
            targets,
            invert_selection,
        } => {
            todo!("clear expenses")
        }
        BC::EditExpense {
            target,
            new_name,
            new_amount,
        } => {
            todo!("edit existing expense")
        }
        BC::NewExpense { name, amount } => {
            budget.add_expense(&name, amount);
        }
        BC::PayExpense { name, amount } => {
            if let Some(c) = amount {
                budget.make_dynamic_payment(&name, c)?;
            } else {
                budget.make_static_payment(&name)?;
            }
        }
        BC::Nothing => {}
    }
    Ok(())
}
