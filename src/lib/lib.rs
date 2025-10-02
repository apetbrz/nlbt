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
    Savings {
        //None = full amount
        amount: Option<i32>,
    },
    Nothing,
}

pub type BudgetCommands = Vec<BudgetCommand>;
impl From<BudgetCommand> for BudgetCommands {
    fn from(cmd: BudgetCommand) -> Self {
        vec![cmd]
    }
}

pub fn execute_cmd(bud: Budget, cmd: BudgetCommand, force: u8) -> Result<Budget> {
    execute_cmds(bud, cmd.into(), force)
}

pub fn execute_cmds(mut bud: Budget, cmds: BudgetCommands, force: u8) -> Result<Budget> {
    //TODO: IMPLEMENT force VALUE
    use crate::BudgetCommand as BC;

    for cmd in cmds {
        #[cfg(debug_assertions)]
        println!("[DEV] executing command: {cmd:?}");

        match cmd {
            BC::SetPaycheck { amount } => {
                bud.set_income(amount);
            }
            BC::Paid { amount } => {
                match amount {
                    Some(c) => bud.get_paid_value(c),
                    None => bud.get_paid(),
                };
            }
            BC::ClearExpense {
                targets,
                invert_selection,
            } => {
                if targets.is_empty() && !invert_selection {
                    bud.refresh();
                } else {
                    todo!("clear command expense selection")
                }
            }
            BC::EditExpense {
                target,
                new_name,
                new_amount,
            } => {
                todo!("edit existing expense")
            }
            BC::NewExpense { name, amount } => {
                bud.add_expense(&name, amount);
            }
            BC::PayExpense { name, amount } => {
                match amount {
                    Some(c) => bud.make_dynamic_payment(&name, c)?,
                    None => bud.make_static_payment(&name)?,
                };
            }
            BC::Savings { amount } => {
                match amount {
                    Some(c) => bud.save(c)?,
                    None => bud.save_all()?,
                };
            }
            BC::Nothing => {}
        }
    }

    Ok(bud)
}
